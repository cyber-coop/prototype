
use std::error::Error;
use std::io::prelude::*;
use std::net::TcpStream;
use std::sync::mpsc::channel;
use std::sync::mpsc::{Receiver, Sender};
use std::thread;
use std::time::Duration;
use log::{info, trace};

use crate::utils;
use crate::p2p::message::Message;
use crate::p2p::version::Version;

pub struct Peer {
    stream: TcpStream,
    ip: String,
    magic_bytes: [u8; 4],
    tx: Sender<Message>,
    rx: Receiver<Message>,
}

impl Peer {
    pub fn new(ip: String, magic_bytes: [u8; 4]) -> Self {
        let stream = TcpStream::connect(&ip).expect("Create TcpStream");

        let (tx, rx) = channel::<Message>();

        let mut peer = Peer {
            stream,
            ip,
            magic_bytes,
            tx,
            rx,
        };

        peer.start_thread();

        return peer;
    }

    pub fn start_thread(&mut self) -> thread::JoinHandle<()> {
        let mut stream = self.stream.try_clone().unwrap();
        let magic_bytes = self.magic_bytes.clone();
        let tx = self.tx.clone();

        thread::spawn(move || {
            loop {
                let mut data: Vec<u8> = vec![];

                // Message first bytes
                let mut buf = [0; 24];

                // we have this loop to be sure we have received at least 24 bytes
                while stream.peek(&mut buf).unwrap() != 24 {
                    thread::sleep(Duration::from_millis(5));
                }
                stream.read_exact(&mut buf).unwrap();
                data.extend(&buf);

                let payload_size = u32::from_le_bytes(buf[16..20].try_into().unwrap()) as usize;
                let _command = String::from_utf8(buf[4..16].to_vec()).unwrap();

                // Message payload
                let mut payload: Vec<u8> = vec![];

                // we have this loop to be sure we have received the complete payload
                while payload.len() < payload_size {
                    let mut buf: Vec<u8> = vec![0; payload_size - payload.len()];
                    let l = stream.read(&mut buf).unwrap();

                    payload.extend(&buf[0..l]);
                    thread::sleep(Duration::from_millis(5));
                }
                data.extend(payload);

                let message = Message::deserialize(&data).unwrap();

                // answer ping message and don't add it to the queue
                if message.command.starts_with("ping") {
                    trace!("'ping' received");
                    let pong =
                        Message::new(magic_bytes, "pong".to_owned(), message.payload.clone());
                    stream.write(&pong.serialize()).unwrap();
                    stream.flush().unwrap();

                    trace!("'pong' sent");

                    continue;
                }

                let _ = tx.send(message);
            }
        })
    }

    pub fn connect(&mut self) -> u32 {
        // version
        let version = utils::create_version(self.ip.as_str());
        let message = Message::new(self.magic_bytes, "version".to_owned(), version.serialize());

        self.send(&message);
        info!("'version' sent");

        let message_rcv = self.read().unwrap();
        info!("'{}' received", message_rcv.command);

        // We need to know the node current block height
        let version = Version::deserialize(&message_rcv.payload).unwrap();
        info!("Current block height: {:?}", version.start_height);

        // verack
        let verack = Message::new(self.magic_bytes, "verack".to_owned(), vec![]);

        self.send(&verack);
        let message_rcv = self.read().unwrap();
        info!("'{}' received", message_rcv.command);

        return version.start_height;
    }

    /*pub fn get_blocks(hash: Vec<u8>) -> GetData {
        // GetBlocks
        let get_blocks =
            GetBlocks::new(70004, vec![hash.clone().try_into().unwrap()], None).serialize();
        let message_get_blocks =
            Message::new(self.magic_bytes, "getblocks".to_owned(), get_blocks);

        self.send(&message_get_blocks);

        loop {
            let message_rcv = self.read().unwrap();

            // we have the right message
            if message_rcv.command.starts_with("inv") {
                break;
            }
        }

        let blocks_inv = GetData::deserialize(&message_rcv.payload).unwrap();
        info!("Inv block count : {}", blocks_inv.count);
    }*/

    pub fn send(&mut self, message: &Message) {
        self.stream
            .write(&message.serialize())
            .expect("write to stream");
        self.stream.flush().expect("flush writer");
    }

    pub fn read(&self) -> Result<Message, Box<dyn Error>> {
        let message = self.rx.recv().unwrap();

        Ok(message)
    }
}
