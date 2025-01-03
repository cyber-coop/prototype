use serde::Deserialize;
use std::fs::File;
use std::io::prelude::*;
use toml;

#[derive(Debug, Deserialize)]
pub struct DatabaseConfig {
    pub host: String,
    pub user: String,
    pub password: String,
    pub dbname: String,
}

#[derive(Debug, Deserialize)]
pub struct Peer {
    pub ip: String,
    pub port: u32,
}

#[derive(Debug, Deserialize)]
pub struct Config {
    pub database: DatabaseConfig,
    pub peer: Peer,
}

pub fn read_config() -> Config {
    let mut file = File::open("config.toml").expect("config.toml file required");
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let config: Config = toml::from_str(&contents).unwrap();

    return config;
}
