/* coinentry.rs - coin entry object for hsd
 * Copyright (c) 2019-2020, Urkel Labs (MIT License).
 * https://github.com/HandshakeAlliance/rsd
 * Derived from Handshake/HSD - https://github.com/handshake-org/hsd
 */

use extended_primitives::Buffer;
use handshake_primitives::{Output, Outpoint, Coin};
// use crate::coinCompress::*;

// Coin Entry Types
// Info --> represents an unspent output 
// Version {Number} - transaction version
// Height {Number} - transaction height (unconfirmed = -1)
// Coinbase {Boolean} - whether the transaction is a coinbase   
// Output {Output}
// Spent {Boolean}
// Raw {Buffer}

// Define the CoinEntry types
#[derive(Debug)]
pub struct CoinEntry {
    pub version: u32,
    pub height: Option<u32>,
    pub coinbase: bool,
    pub output: Output,
    pub spent: bool,
    pub raw: Buffer,
}

impl CoinEntry {
    
    // Converts coin entry to a coin
    // @param {Outpoint} prevout
    // returns Coin
    pub fn to_coin(self, prevout: Outpoint) -> Coin {
        Coin::new(
            self.version, 
            self.height, 
            self.coinbase, 
            prevout.hash(), 
            prevout.index(), 
            self.output,
        )
    }
  
    // Inject properties from TX.
    // @param {TX} tx
    // @param {Number} index
    // TODO - finish and figure out if a static function is needed
    pub fn from_tx() {

    }

}

#[cfg(test)]
mod test {
    use super::*;
    
    #[test]
    fn test_output() {
        let coin = CoinEntry {
            version: 0,
            height: None,
            coinbase: false,
            output: Default::default(),
            spent: false,
            raw: Buffer::new(),
        };
        dbg!(coin);
    }
}
