use super::duplicate::{DOGECOIN_TESTNET_DUPLICATE, NAMECOIN_MAINNET_DUPLICATE};
use bitcoin_network::block::Block;
use postgres::Client;
use std::io::prelude::*;
use std::time::Instant;

pub fn save_blocks(
    blocks: Vec<Block>,
    schema_name: &String,
    postgres_client:&mut Client,
    current_height: &mut u32,
) {
    // register in database
    let now = Instant::now();
    let mut blocks_string: String = String::new();
    let mut transactions_string: String = String::new();
    let mut txins_string: String = String::new();
    let mut txouts_string: String = String::new();

    trace!("starting to format blocks and transactions");
    blocks.iter().for_each(|b| {
        *current_height = *current_height + 1;

        let hash = b.hash().to_vec();
        let tmp = format!(
            "{},\\\\x{},\\\\x{},\\\\x{}\n",
            current_height,
            hex::encode(&hash),
            hex::encode(&b.previous_hash),
            hex::encode(&b.merkle_root)
        );

        blocks_string.push_str(&tmp);
        b.transactions.iter().for_each(|t| {
            let txid = t.hash().to_vec();

            // Remove dogecoin testnet duplicate txid
            // TODO: improve this; we could maybe delete in sql before applying constraints
            if schema_name == "dogecoin_testnet" {
                if DOGECOIN_TESTNET_DUPLICATE
                    .iter()
                    .find(|x| x.0 == hex::encode(&txid) && x.1 == hex::encode(&hash))
                    .is_some()
                {
                    info!("duplicate txid");
                    return;
                }
            }

            // Remove namecoin mainnet duplicate txid
            if schema_name == "namecoin_mainnet" {
                if NAMECOIN_MAINNET_DUPLICATE
                    .iter()
                    .find(|x| x.0 == hex::encode(&txid) && x.1 == hex::encode(&hash))
                    .is_some()
                {
                    info!("duplicate txid");
                    return;
                }
            }

            let tmp = format!(
                "{},\\\\x{},\\\\x{},{}\n",
                t.version,
                hex::encode(&txid),
                hex::encode(&hash),
                t.lock_time
            );

            // let tmp = format!("\\\\x{},\\\\x{}\n", hex::encode(&txid), hex::encode(&hash));
            transactions_string.push_str(&tmp);

            t.tx_ins.iter().enumerate().for_each(|(i, txin)| {
                let tmp = format!(
                    "\\\\x{},{},\\\\x{},{},\\\\x{},{}\n",
                    hex::encode(&txid),
                    i,
                    hex::encode(&txin.previous_output.previous_hash),
                    txin.previous_output.index,
                    hex::encode(&txin.signature_script),
                    txin.sequence
                );
                txins_string.push_str(&tmp);
            });
            t.tx_outs.iter().enumerate().for_each(|(i, txout)| {
                let tmp = format!(
                    "\\\\x{},{},{},\\\\x{}\n",
                    hex::encode(&txid),
                    i,
                    txout.value,
                    hex::encode(&txout.pk_script)
                );
                txouts_string.push_str(&tmp);
            });
        });
    });
    trace!("Finished formating blocks and transactions message");

    let mut block_writer = postgres_client
    .copy_in(format!("COPY {}.blocks FROM stdin (DELIMITER ',')", schema_name).as_str())
    .unwrap();
    block_writer
        .write_all(blocks_string.as_bytes())
        .unwrap();
    block_writer.finish().unwrap();

    let mut transaction_writer = postgres_client
    .copy_in(
        format!(
            "COPY {}.transactions FROM stdin (DELIMITER ',')",
            schema_name
        )
        .as_str(),
    )
    .unwrap();
    transaction_writer
        .write_all(transactions_string.as_bytes())
        .unwrap();
    transaction_writer.finish().unwrap();

    let mut txins_writer = postgres_client
    .copy_in(format!("COPY {}.txins FROM stdin (DELIMITER ',')", schema_name).as_str())
    .unwrap();
    txins_writer
        .write_all(txins_string.as_bytes())
        .unwrap();
    txins_writer.finish().unwrap();

    let mut txouts_writer = postgres_client
    .copy_in(format!("COPY {}.txouts FROM stdin (DELIMITER ',')", schema_name).as_str())
    .unwrap();
    txouts_writer
        .write_all(txouts_string.as_bytes())
        .unwrap();
    txouts_writer.finish().unwrap();

    // let mut transaction_writer = client.copy_in(format!("COPY {}.txs FROM stdin (DELIMITER ',')", schema_name).as_str()).unwrap();
    // transaction_writer.write(transactions_string.as_bytes()).unwrap();
    // transaction_writer.finish().unwrap();

    info!(
        "Blocks registered in database (current_height {}) {:.2?}",
        current_height,
        now.elapsed()
    );
}

pub fn finish() {
    info!("done!")
}
