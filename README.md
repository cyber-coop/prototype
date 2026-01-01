# Bitcoin P2P indexer

This project is an indexer for Bitcoin and Bitcoin forks. It takes advantage of the P2P protocol in Bitcoin to fetch blocks and transactions.

## Run

### Config

The indexer need a couple of information to be specified in a `config.toml` file : database info and one peer to the network. To create a config file copy `config.example.toml` into `config.toml`.

`config.toml` example
```
[database]
host = "localhost"
user = "postgres"
password = "wow"
dbname = "blockchains"

[peer]
ip = "127.0.0.1"
port = 8333
```

To find a peer we can do a `nslookup` on the dns seed of the network we wnat to index. For the port it is the p2p port used by this specific network (e.g bitcoin mainnet is `8333` and fordogecoin testnet it is `44556`).

### Docker compose

In the `contrib/` folder there is a `docker-compose.yml` file to start any indexer. First we need to create a config file matching the network we want to index (e.g `config.dogecoin-mainnet.toml`).

Then run the docker compose command.
```
$ docker compose up postgres indexer_dogecoin_mainnet
```

or to start them all
```
$ docker compose up
```