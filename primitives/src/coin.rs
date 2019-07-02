use extended_primitives::Hash; 
use crate::Output;

pub struct Coin {
    version: u32,
    height: Option<u32>,
    coinbase: bool,
    hash: Hash,
    index: u32,
    output: Output,
}
    
impl Coin {
     pub fn new(version: u32, height: Option<u32>, coinbase: bool, hash: Hash, index: u32, output: Output) -> Coin {
        Coin { version, height, coinbase, hash, index, output }
     }
}
