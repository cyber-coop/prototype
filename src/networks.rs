use magic_bytes::MagicBytes;
use std::error::Error;

pub struct Network {
    pub testnet: bool,
    pub magic_bytes: [u8; 4],
    pub aux_pow: bool,
    pub genesis_hash: [u8; 32],
}

impl Network {
    pub const BITCOIN_MAINNET: Network = Network {
        testnet: false,
        magic_bytes: MagicBytes::BITCOIN_MAINNET,
        aux_pow: false,
        genesis_hash: [
            0, 0, 0, 0, 0, 25, 214, 104, 156, 8, 90, 225, 101, 131, 30, 147, 79, 247, 99, 174, 70,
            162, 166, 193, 114, 179, 241, 182, 10, 140, 226, 111,
        ],
    };

    pub const BITCOIN_TESTNET: Network = Network {
        testnet: true,
        // FIXME: incorrect bitcoin testnet/regtest value in magic byte lib
        magic_bytes: MagicBytes::BITCOIN_REGTEST,
        aux_pow: false,
        genesis_hash: [
            0, 0, 0, 0, 9, 51, 234, 1, 173, 14, 233, 132, 32, 151, 121, 186, 174, 195, 206, 217,
            15, 163, 244, 8, 113, 149, 38, 248, 215, 127, 73, 67,
        ],
    };

    pub const LITECOIN_MAINNET: Network = Network {
        testnet: false,
        magic_bytes: MagicBytes::LITECOIN_MAINNET,
        aux_pow: false,
        genesis_hash: [
            18, 167, 101, 227, 31, 253, 64, 89, 186, 218, 30, 37, 25, 15, 110, 152, 201, 157, 151,
            20, 211, 52, 239, 164, 26, 25, 90, 126, 126, 4, 191, 226,
        ],
    };

    pub const LITECOIN_TESTNET: Network = Network {
        testnet: true,
        magic_bytes: MagicBytes::LITECOIN_TESTNET,
        aux_pow: false,
        genesis_hash: [
            73, 102, 98, 90, 75, 40, 81, 217, 253, 238, 19, 158, 86, 33, 26, 13, 136, 87, 95, 89,
            237, 129, 111, 245, 230, 166, 61, 235, 78, 62, 41, 160,
        ],
    };

    pub const NAMECOIN_MAINNET: Network = Network {
        testnet: false,
        magic_bytes: MagicBytes::NAMECOIN_MAINNET,
        aux_pow: true,
        genesis_hash: [
            0, 0, 0, 0, 0, 98, 183, 44, 94, 44, 235, 69, 251, 200, 88, 126, 128, 124, 21, 91, 13,
            167, 53, 230, 72, 61, 251, 162, 240, 169, 199, 112,
        ],
    };

    pub const DOGECOIN_MAINNET: Network = Network {
        testnet: false,
        magic_bytes: MagicBytes::DOGECOIN_MAINNET,
        aux_pow: true,
        genesis_hash: [
            26, 145, 227, 218, 206, 54, 226, 190, 59, 240, 48, 166, 86, 121, 254, 130, 26, 161,
            214, 239, 146, 231, 201, 144, 46, 179, 24, 24, 44, 53, 86, 145,
        ],
    };

    pub const DOGECOIN_TESTNET: Network = Network {
        testnet: true,
        magic_bytes: MagicBytes::DOGECOIN_TESTNET,
        aux_pow: true,
        genesis_hash: [
            187, 10, 120, 38, 70, 55, 64, 107, 99, 96, 170, 217, 38, 40, 77, 84, 77, 112, 73, 244,
            81, 137, 219, 86, 100, 243, 196, 208, 115, 80, 85, 158,
        ],
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
