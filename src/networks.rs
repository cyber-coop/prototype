use magic_bytes::MagicBytes;
use std::error::Error;

pub struct Network {
    pub testnet: bool,
    pub magic_bytes: [u8; 4],
    pub aux_pow: bool,
    pub genesis_hash: [u8; 32],
    pub dns_seeds: &'static [&'static str],
    pub port: u16,
}

impl Network {
    pub const BITCOIN_MAINNET: Network = Network {
        testnet: false,
        magic_bytes: MagicBytes::BITCOIN_MAINNET,
        aux_pow: false,
        port: 8333,
        genesis_hash: [
            0, 0, 0, 0, 0, 25, 214, 104, 156, 8, 90, 225, 101, 131, 30, 147, 79, 247, 99, 174, 70,
            162, 166, 193, 114, 179, 241, 182, 10, 140, 226, 111,
        ],
        dns_seeds: &[
            "seed.bitcoin.sipa.be",
            "dnsseed.bluematt.me",
            "dnsseed.bitcoin.dashjr.org",
            "seed.bitcoinstats.com",
            "seed.bitcoin.jonasschnelli.ch",
            "seed.btc.petertodd.net",
        ],
    };

    pub const BITCOIN_TESTNET: Network = Network {
        testnet: true,
        // FIXME: incorrect bitcoin testnet/regtest value in magic byte lib
        magic_bytes: MagicBytes::BITCOIN_REGTEST,
        aux_pow: false,
        port: 18333,
        genesis_hash: [
            0, 0, 0, 0, 9, 51, 234, 1, 173, 14, 233, 132, 32, 151, 121, 186, 174, 195, 206, 217,
            15, 163, 244, 8, 113, 149, 38, 248, 215, 127, 73, 67,
        ],
        dns_seeds: &[
            "testnet-seed.bitcoin.jonasschnelli.ch",
            "seed.tbtc.petertodd.net",
            "testnet-seed.bluematt.me",
        ],
    };

    pub const LITECOIN_MAINNET: Network = Network {
        testnet: false,
        magic_bytes: MagicBytes::LITECOIN_MAINNET,
        aux_pow: false,
        port: 9333,
        genesis_hash: [
            18, 167, 101, 227, 31, 253, 64, 89, 186, 218, 30, 37, 25, 15, 110, 152, 201, 157, 151,
            20, 211, 52, 239, 164, 26, 25, 90, 126, 126, 4, 191, 226,
        ],
        dns_seeds: &[
            "seed-a.litecoin.loshan.co.uk",
            "dnsseed.thrasher.io",
        ],
    };

    pub const LITECOIN_TESTNET: Network = Network {
        testnet: true,
        magic_bytes: MagicBytes::LITECOIN_TESTNET,
        aux_pow: false,
        port: 19333,
        genesis_hash: [
            73, 102, 98, 90, 75, 40, 81, 217, 253, 238, 19, 158, 86, 33, 26, 13, 136, 87, 95, 89,
            237, 129, 111, 245, 230, 166, 61, 235, 78, 62, 41, 160,
        ],
        dns_seeds: &[
            "seed-b.litecoin.loshan.co.uk",
            "dnsseed-testnet.thrasher.io",
            "testnet-seed.ltc.xurious.com",
        ],
    };

    pub const NAMECOIN_MAINNET: Network = Network {
        testnet: false,
        magic_bytes: MagicBytes::NAMECOIN_MAINNET,
        aux_pow: true,
        port: 8334,
        genesis_hash: [
            0, 0, 0, 0, 0, 98, 183, 44, 94, 44, 235, 69, 251, 200, 88, 126, 128, 124, 21, 91, 13,
            167, 53, 230, 72, 61, 251, 162, 240, 169, 199, 112,
        ],
        dns_seeds: &["dnsseed.namecoin.webbtc.com"],
    };

    pub const DOGECOIN_MAINNET: Network = Network {
        testnet: false,
        magic_bytes: MagicBytes::DOGECOIN_MAINNET,
        aux_pow: true,
        port: 22556,
        genesis_hash: [
            26, 145, 227, 218, 206, 54, 226, 190, 59, 240, 48, 166, 86, 121, 254, 130, 26, 161,
            214, 239, 146, 231, 201, 144, 46, 179, 24, 24, 44, 53, 86, 145,
        ],
        dns_seeds: &["seed.multidoge.org", "seed2.multidoge.org"],
    };

    pub const DOGECOIN_TESTNET: Network = Network {
        testnet: true,
        magic_bytes: MagicBytes::DOGECOIN_TESTNET,
        aux_pow: true,
        port: 44556,
        genesis_hash: [
            187, 10, 120, 38, 70, 55, 64, 107, 99, 96, 170, 217, 38, 40, 77, 84, 77, 112, 73, 244,
            81, 137, 219, 86, 100, 243, 196, 208, 115, 80, 85, 158,
        ],
        dns_seeds: &["testseed.jrn.me.uk"],
    };

    pub fn find(network: &str) -> Result<Self, Box<dyn Error>> {
        match network {
            "bitcoin_mainnet" => Ok(Self::BITCOIN_MAINNET),
            "bitcoin_testnet" => Ok(Self::BITCOIN_TESTNET),
            "dogecoin_mainnet" => Ok(Self::DOGECOIN_MAINNET),
            "dogecoin_testnet" => Ok(Self::DOGECOIN_TESTNET),
            "litecoin_mainnet" => Ok(Self::LITECOIN_MAINNET),
            "litecoin_testnet" => Ok(Self::LITECOIN_TESTNET),
            "namecoin_mainnet" => Ok(Self::NAMECOIN_MAINNET),
            _ => Err("not matching available networks.".into()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::net::ToSocketAddrs;

    fn lookup_seeds(network: &Network) {
        for seed in network.dns_seeds {
            let addrs: Vec<_> = (*seed, 0u16)
                .to_socket_addrs()
                .unwrap_or_else(|e| panic!("{seed}: DNS lookup failed: {e}"))
                .collect();
            assert!(!addrs.is_empty(), "{seed}: resolved to no addresses");
            println!("{seed}: {:?}", addrs.iter().map(|a| a.ip()).collect::<Vec<_>>());
        }
    }

    #[test]
    fn bitcoin_mainnet_dns_seeds() {
        lookup_seeds(&Network::BITCOIN_MAINNET);
    }

    #[test]
    fn bitcoin_testnet_dns_seeds() {
        lookup_seeds(&Network::BITCOIN_TESTNET);
    }

    #[test]
    fn litecoin_mainnet_dns_seeds() {
        lookup_seeds(&Network::LITECOIN_MAINNET);
    }

    #[test]
    fn litecoin_testnet_dns_seeds() {
        lookup_seeds(&Network::LITECOIN_TESTNET);
    }

    #[test]
    fn namecoin_mainnet_dns_seeds() {
        lookup_seeds(&Network::NAMECOIN_MAINNET);
    }

    #[test]
    fn dogecoin_mainnet_dns_seeds() {
        lookup_seeds(&Network::DOGECOIN_MAINNET);
    }

    #[test]
    fn dogecoin_testnet_dns_seeds() {
        lookup_seeds(&Network::DOGECOIN_TESTNET);
    }
}
