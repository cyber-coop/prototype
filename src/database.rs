use crate::p2p::block::Block;
use postgres::Client;
use std::io::prelude::*;
use std::time::Instant;
use log::info;

pub fn create_tables(schema_name: &String, postgres_client: &mut Client) {
    let query = format!(
        "
        CREATE SCHEMA IF NOT EXISTS {schema_name};
        CREATE TABLE IF NOT EXISTS {schema_name}.blocks (
            height INTEGER NOT NULL,
            hash BYTEA NOT NULL,
            prevhash BYTEA NOT NULL,
            merkleroot BYTEA NOT NULL
        );
        CREATE TABLE IF NOT EXISTS {schema_name}.transactions (
            version INTEGER NOT NULL,
            txid BYTEA,
            block BYTEA NOT NULL,
            locktime BIGINT NOT NULL
        );
        CREATE TABLE IF NOT EXISTS {schema_name}.txins (
            txid BYTEA NOT NULL,
            index INTEGER NOT NULL,
            outputhash BYTEA NOT NULL,
            outputindex BIGINT NOT NULL,
            sigscript BYTEA,
            sequence BIGINT NOT NULL
        );
        CREATE TABLE IF NOT EXISTS {schema_name}.txouts (
            txid BYTEA NOT NULL,
            index INTEGER NOT NULL,
            value BIGINT,
            pkscript BYTEA
        );
        "
    );

    postgres_client.batch_execute(&query).unwrap();
}

pub fn save_blocks(
    blocks: Vec<Block>,
    schema_name: &String,
    postgres_client: &mut Client,
    current_height: &mut u32,
) {
    // register in database
    let now = Instant::now();
    let mut blocks_string: String = String::new();
    let mut transactions_string: String = String::new();
    let mut txins_string: String = String::new();
    let mut txouts_string: String = String::new();
    // we have a 1Gigabyte limit and the transactions bulk will go over that limit so we split it
    let mut txouts_strings: Vec<String> = vec![];

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

            let tmp = format!(
                "{},\\\\x{},\\\\x{},{}\n",
                t.version,
                hex::encode(&txid),
                hex::encode(&hash),
                t.lock_time
            );

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

                // verifying if we are not going over the 1Gigabyte limit if yes we start a new copy in query
                if txouts_string.as_bytes().len() + tmp.as_bytes().len() > 1000000000 {
                    txouts_strings.push(txouts_string.clone());
                    txouts_string = String::new();
                }

                txouts_string.push_str(&tmp);
            });
        });
    });

    let mut transaction = postgres_client.transaction().unwrap();
    let mut block_writer = transaction
        .copy_in(format!("COPY {}.blocks FROM stdin (DELIMITER ',')", schema_name).as_str())
        .unwrap();
    block_writer.write_all(blocks_string.as_bytes()).unwrap();
    block_writer.finish().unwrap();

    let mut transaction_writer = transaction
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

    let mut txins_writer = transaction
        .copy_in(format!("COPY {}.txins FROM stdin (DELIMITER ',')", schema_name).as_str())
        .unwrap();
    txins_writer.write_all(txins_string.as_bytes()).unwrap();
    txins_writer.finish().unwrap();

    let mut chunk_index = 1;
    let number_of_chunks = txouts_strings.len();
    for txouts in txouts_strings {
        info!(
            "Sending txouts chunk {}/{} (size {} bytes)",
            chunk_index,
            number_of_chunks,
            txouts.as_bytes().len()
        );

        let mut txouts_writer = transaction
            .copy_in(format!("COPY {}.txouts FROM stdin (DELIMITER ',')", schema_name).as_str())
            .unwrap();
        txouts_writer.write_all(txouts.as_bytes()).unwrap();
        txouts_writer.finish().unwrap();

        chunk_index = chunk_index + 1;
    }

    // commit the transaction
    transaction.commit().unwrap();

    info!(
        "Blocks registered in database (current_height {}) {:.2?}",
        current_height,
        now.elapsed()
    );
}
