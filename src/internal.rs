pub use scrypto::prelude::{Address};

pub trait ResourceDecl {
    const ADDRESS: Option<Address>;
}