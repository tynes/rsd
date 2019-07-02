use crate::{Address, Covenant};
use handshake_types::Amount;

#[derive(Clone, PartialEq, Eq, Debug, Default)]
pub struct Output {
    value: Amount,
    address: Address,
    covenant: Covenant,
}

//TODO get size, is_dust, format, equal + peq, to hex from hex, to buffer, from buffer.
impl Output {
    pub fn new(value: Amount, address: Address, covenant: Covenant ) -> Output {
        Output { value, address, covenant }
    }

    pub fn is_unspendable(&self) -> bool {
        self.address.is_unspendable() | self.covenant.is_unspendable()
    }
}
