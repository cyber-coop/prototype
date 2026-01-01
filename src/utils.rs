use crate::p2p::{address::Address, version::Version};
use crate::p2p::{block::Block, inventory::Inventory};
use std::error::Error;
use std::time::{SystemTime, UNIX_EPOCH};

pub fn check_chain(blocks: &mut Vec<Block>, hash: Vec<u8>) -> Result<Vec<Block>, Box<dyn Error>> {
    let mut sorted_blocks: Vec<Block> = Vec::new();

    match blocks.iter().position(|b| hash == b.previous_hash) {
        Some(block_index) => {
            let block = blocks.remove(block_index);
            sorted_blocks.push(block);
        }
        None => {
            return Ok(sorted_blocks);
        }
    }

    while blocks.len() > 0 {
        let hash = sorted_blocks.last().unwrap().hash();
        match blocks.iter().position(|b| hash == b.previous_hash) {
            Some(block_index) => {
                let block = blocks.remove(block_index);
                sorted_blocks.push(block);
            }
            None => break,
        }
    }
    Ok(sorted_blocks)
}

pub fn create_version(host: &str) -> Version {
    let split: Vec<&str> = host.split(":").collect();
    let ip_string: Vec<&str> = split[0].split(".").collect();
    let _port: u16 = split[1].parse().unwrap();

    // FIXME: Only support ipv4
    let ip = [
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        255,
        255,
        ip_string[0].parse::<u8>().unwrap(),
        ip_string[1].parse::<u8>().unwrap(),
        ip_string[2].parse::<u8>().unwrap(),
        ip_string[3].parse::<u8>().unwrap(),
    ];

    Version {
        version: 70012,
        services: 4,
        timestamp: SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs(),
        addr_recv: Address {
            services: 1,
            // FIXME: should be receiver address
            ip,
            port: 0,
        },
        addr_trans: Address {
            services: 4,
            ip: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 255, 255, 127, 0, 0, 1],
            port: 0,
        },
        nonce: 1,
        user_agent: "deadbrain corp.".to_owned(),
        start_height: 0,
        relay: false,
    }
}

pub fn verify_inv_identifier(inventories: Vec<Inventory>) -> bool {
    let mut result = true;
    inventories.iter().for_each(|inv| {
        if inv.identifier != 2 {
            result = false;
        }
    });

    return result;
}
