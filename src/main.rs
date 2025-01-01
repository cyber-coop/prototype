use bitcoin_network::{block::Block, get_blocks::GetBlocks, get_data::GetData, message::Message};
use std::env;
use std::sync::mpsc::sync_channel;
use std::thread;
use std::time::Instant;

pub mod configs;
pub mod database;
pub mod duplicate;
pub mod networks;
pub mod peer;
pub mod utils;

use crate::database::save_blocks;
use crate::peer::Peer;

#[macro_use]
extern crate log;

fn main() {
    // Init log
    env_logger::init();

    info!("Starting prototype!");

    // Read cli args
    let network_arg: String = env::args()
        .nth(1)
        .expect("expecting a network (bitcoin_mainnet, bitcoin_testnet, dogecoin_mainnet, dogecoin_testnet, litecoin_mainnet, litecoin_tesnet or namecoin_mainnet).");
    let network = networks::Network::find(network_arg.as_str()).unwrap();

    // Load config values from the config file
    let config = configs::read_config();

    /********************
     *
     *      CONNECT
     *
     ********************/

    let mut message_rcv: Message;
    let mut peer = Peer::new(
        format!("{}:{}", config.peer.ip, config.peer.port),
        network.magic_bytes,
    );
    let mut current_height: u32 = 0;

    let mut hash = network.genesis_hash.to_vec();
    hash.reverse();

    let height = peer.connect();

    /********************
     *
     *  Get latest block
     *
     ********************/
    let database_params = format!(
        "host={} user={} password={} dbname={}",
        config.database.host,
        config.database.user,
        config.database.password,
        config.database.dbname
    );

    let mut postgres_client = postgres::Client::connect(&database_params, postgres::NoTls).unwrap();

    // create the tables if they don't exist
    database::create_tables(&network_arg, &mut postgres_client);

    let result = postgres_client.query(format!("SELECT * FROM {0}.blocks a JOIN (SELECT MAX(height) as h FROM {0}.blocks) b ON a.height = b.h;", network_arg).as_str(), &[]).unwrap();
    if result.len() > 0 {
        let row = &result[0];
        current_height = row
            .get::<usize, i32>(0)
            .try_into()
            .expect("to be able to convert i32 to u32 for block height");
        hash = row.get(1);

        info!(
            "We have found a new hash {} and height {}",
            hex::encode(&hash),
            current_height
        );
    }

    info!(
        "We have found a new hash {} and height {}",
        hex::encode(&hash),
        current_height
    );

    /********************
     *
     *  Start database thread
     *
     ********************/

    // Create a simple streaming channel (limited buffer of 4 batch of 500 blocks to avoid filling ram)
    let (tx, rx) = sync_channel(4);

    let _thread_handle = thread::spawn(move || {
        info!("Starting database thread");
        let mut process_current_height = current_height;

        // Connect to database
        let mut postgres_client =
            postgres::Client::connect(&database_params, postgres::NoTls).unwrap();

        // while recv save blocks in database
        loop {
            let blocks: Vec<Block> = rx.recv().unwrap();

            save_blocks(
                blocks,
                &network_arg,
                &mut postgres_client,
                &mut process_current_height,
            );

            // We are synced
            if process_current_height > height {
                info!("We are synced !");
                break;
            }
        }
    });

    /****************************
     *
     *  START FETCHING BLOCKS
     *
     ****************************/

    loop {
        let mut blocks_inv: GetData;

        // GetBlocks
        let get_blocks =
            GetBlocks::new(70004, vec![hash.clone().try_into().unwrap()], None).serialize();
        let message_get_blocks =
            Message::new(network.magic_bytes, "getblocks".to_owned(), get_blocks);

        peer.send(&message_get_blocks);

        loop {
            message_rcv = peer.read().unwrap();

            if message_rcv.command.starts_with("inv") {
                blocks_inv = GetData::deserialize(&message_rcv.payload).unwrap();

                // Verify if inventory is what we asked for
                let verified = utils::verify_inv_identifier(blocks_inv.inventory);
                if !verified {
                    warn!("One of the inventory record is not a block message. We might have a problem.")
                }

                if blocks_inv.count > 1 {
                    // For now ignore notify inv message
                    break;
                }
            }
        }

        // blocks_inv = GetData::deserialize(&message_rcv.payload).unwrap();
        info!("Inv block count : {}", blocks_inv.count);
        // Inv message and get_data are the same message
        let get_data_message = Message::new(
            network.magic_bytes,
            "getdata".to_owned(),
            message_rcv.payload,
        );

        let now = Instant::now();
        peer.send(&get_data_message);

        let mut count: u64 = 0;
        let mut blocks: Vec<Block> = vec![];

        loop {
            // Sometimes we have 501 blocks because they are adding the newly discovered block at the end
            if count == blocks_inv.count {
                break;
            }

            message_rcv = peer.read().unwrap();
            if message_rcv.command.starts_with("block") {
                count += 1;

                // NOTES: we won't receive the blocks in order. We would need to match with previous_hash to rebuild the chain.
                let block = Block::deserialize(&message_rcv.payload, network.aux_pow).expect(
                    &format!("Fail to deserialize {}", hex::encode(&message_rcv.payload)),
                );

                blocks.push(block);
            }
        }
        info!("Received all the blocks after {:.2?}", now.elapsed());

        // HERE - we need to handle forks
        let blocks = utils::check_chain(&mut blocks, hash.clone()).unwrap();
        // Looks into the database for the previous hash. If the previous hash exist we have a fork

        current_height += blocks.len() as u32;
        info!("Progress {}/{}", current_height, height);

        // update hash with the latest one
        match blocks.last() {
            Some(x) => hash = x.hash().to_vec(),
            _ => {}
        };

        // send blocks to the other thread to save in database
        if blocks.len() > 0 {
            tx.send(blocks).unwrap();
        }
    }
}
