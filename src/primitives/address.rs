use blake2::Blake2b;
use bech32::Bech32; // https://github.com/bitcoin/bips/blob/master/bip-0173.mediawiki#bech32
use std::str::FromStr;

pub struct Address {
    pub version: u32,
    pub hash: Blake2b,
    // Eventually will be type 'network'
    pub network: String
}

// Maybe do this https://doc.rust-lang.org/std/str/trait.FromStr.html
impl From<&str> for Address {
    fn from(data: &str) -> Self {
        // bech32 --> [ addr.hrp | separator: 1 | version = addr.data[0] | remainder of addr.data ]
        // Example: "ts1qq79hzunlkj50fvm7rxg3xetx4kml4e0am43htk"
        // [ addr.hrp = "ts" | separator = 1 | version = q | data = q79hzunlkj50fvm7rxg3xetx4kml4e0am43htk ]
        let addr = Bech32::from_str(data);

        // if addr.hrp == "hs1" || addr.hrp == "ts1" || addr.hrp == "ss1" || addr.hrp == "rs1"

        // Hard coded for test
        dbg!(addr);
        Address{
            version: 0,
            hash: Default::default(),
            network: "testnet".to_string()
        }

        //TODO: Need to dynamically get the network
        // let network = match addr.hrp {
        //     "hs" => "mainnet",
        //     "ts" => "testnet",
        //     "rs" => "regtest",
        //     "ss" => "simnet"
        // }
        //
        // let network
    }
}

#[cfg(test)] // Printing the test 'cargo test -- --nocapture'
mod tests {
    // Allows use of all code above
    use super::*;
    #[test]
    fn test_addr_from_str() {
        let addr = Address::from("ts1qq79hzunlkj50fvm7rxg3xetx4kml4e0am43htk");
    }
}
