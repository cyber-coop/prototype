use bitcoin_network::{Address, GetData, Inventory, Message, Version};
use magic_bytes::MagicBytes;
use std::error::Error;
use std::io::prelude::*;
use std::net::TcpStream;
use std::time::{SystemTime, UNIX_EPOCH};

fn main() -> Result<(), Box<dyn Error>> {
    let mut stream = TcpStream::connect("163.172.182.246:44556")?; //testnet
    const VERSION_HEX: &str = "741101000400000000000000bdfd8d3e76010000010000000000000000000000000000000000ffff7f000001480c040000000000000000000000000000000000ffff7f00000100000000000000000000000000000000";
    //let x = hex::decode(VERSION_HEX).unwrap();
    let version = Version {
        version: 70004,
        services: 4,
        timestamp: SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs(),
        addr_recv: Address {
            services: 1,
            ip: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 255, 255, 163, 172, 182, 246],
            port: 0,
        },
        addr_trans: Address {
            services: 4,
            ip: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 255, 255, 127, 0, 0, 1],
            port: 0,
        },
        nonce: 1,
        user_agent: "elon".to_owned(),
        start_height: 0,
        relay: false,
    }
    .serialize();

    // version
    let message = Message::new(MagicBytes::DOGECOIN_TESTNET, "version".to_owned(), version);

    stream.write(&message.serialize())?;
    stream.flush()?;
    let mut buf = [0; 4096];
    let n = stream.read(&mut buf)?;
    dbg!(n);
    dbg!(hex::encode(&buf[..n].to_vec()));

    // verack
    let verack = Message::new(MagicBytes::DOGECOIN_TESTNET, "verack".to_owned(), vec![]);

    stream.write(&verack.serialize())?;
    stream.flush()?;
    let mut buf = [0; 4096];
    let n = stream.read(&mut buf)?;
    dbg!(n);
    dbg!(hex::encode(&buf[..n].to_vec()));

    // GetData
    let mut hash =
        hex::decode("5bf400bf44ac7a7cb0542ee7e3f9374f68be2dfdf0d64a654c2def6288b3936b").unwrap();
    hash.reverse();

    let get_data = GetData::new(vec![Inventory {
        identifier: 1,
        hash: hash.try_into().unwrap(),
    }])
    .serialize();

    dbg!(hex::encode(get_data.clone()));
    let get_tx = Message::new(MagicBytes::DOGECOIN_TESTNET, "getdata".to_owned(), get_data);

    stream.write(&get_tx.serialize())?;
    stream.flush()?;
    let mut buf = [0; 4096];
    let n = stream.read(&mut buf)?;
    dbg!(n);
    dbg!(hex::encode(&buf[..n].to_vec()));

    Ok(())
}
