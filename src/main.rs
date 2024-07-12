use bitcoin_network::{
    block::Block, get_blocks::GetBlocks, get_data::GetData, message::Message, version::Version,
};
use std::env;
use std::io::prelude::*;
use std::net::TcpStream;
use std::str::FromStr;
use std::sync::mpsc::sync_channel;
use std::thread;
use std::time::Duration;
use std::time::Instant;

pub mod database;
pub mod duplicate;
pub mod networks;
pub mod peer;
pub mod utils;
pub mod configs;

use crate::database::{save_blocks, finish};
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
        .expect("expecting a network (Bitcoin, Namecoin, Litecoin or Dogecoin).");
    let testnet_arg: String = env::args()
        .nth(2)
        .expect("expecting 'true' or 'false' to indicate if you want to connect to testnet.");
    let network = networks::Networks::from_str(network_arg.as_str()).unwrap();
    let testnet: bool = FromStr::from_str(testnet_arg.as_str()).unwrap();

    // Lazy hack to wait for postgres to be up and running
    info!("wait 60 secs");
    thread::sleep(Duration::from_secs(60));

    // Load config values from the config file
    let config = configs::read_config();

    // Find network based on args
    let network_info = networks::Network::find(network, testnet).unwrap();
    let schema_name = network.get_schema_name(testnet);

    /********************
     *
     *      CONNECT
     *
     ********************/

    let mut message_rcv: Message;
    let mut peer = Peer::new(format!("{},{}", config.peer.ip, config.peer.port), network_info.magic_bytes);
    let mut current_height: u32  = 0;

    let mut hash = network_info.genesis_hash.to_vec();
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

    let mut postgres_client = postgres::Client::connect(
        &database_params,
        postgres::NoTls,
    )
    .unwrap();

    let result = postgres_client.query(format!("SELECT * FROM {}.blocks a JOIN (SELECT MAX(height) as h FROM {}.blocks) b ON a.height = b.h;", schema_name, schema_name).as_str(), &[]).unwrap();    

    if result.len() > 0 {
        let row = &result[0];
        current_height = row.get(0);
        hash = row.get(1);

        info!("We have found a new hash {} and height {}", hex::encode(&hash), current_height);
    }

    info!("We have found a new hash {} and height {}", hex::encode(&hash), current_height);

    /********************
     *
     *  Start database thread
     *
     ********************/

    // Create a simple streaming channel (limited buffer of 4 batch of 500 blocks to avoid filling ram)
    let (tx, rx) = sync_channel(4);

    let thread_handle = thread::spawn(move || {
        info!("Starting database thread");
        let mut process_current_height = current_height;

        // Connect to database
        let mut postgres_client = postgres::Client::connect(
            &database_params,
            postgres::NoTls,
        )
        .unwrap();

        // while recv save blocks in database
        loop {
            let mut blocks: Vec<Block> = vec![];

            blocks = rx.recv().unwrap();

            save_blocks(blocks, &schema_name, &mut postgres_client, &mut process_current_height);

            // We are synced
            if process_current_height > height {
                info!("We are synced !");
                break;
            }
        }

        finish();
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
            Message::new(network_info.magic_bytes, "getblocks".to_owned(), get_blocks);

        peer.send(&message_get_blocks);

        loop {
            message_rcv = peer.read().unwrap();

            if message_rcv.command.starts_with("inv") {
                blocks_inv = GetData::deserialize(&message_rcv.payload).unwrap();

                // Verify if inventory is what we asked for
                let verified = utils::verify_inv_identifier(blocks_inv.inventory);
                if !verified { warn!("One of the inventory record is not a block message. We might have a problem.") }
                
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
            network_info.magic_bytes,
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
                let block = Block::deserialize(&message_rcv.payload, network_info.aux_pow).expect(
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
