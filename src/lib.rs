//! Mapping between [SLIP-0044](https://github.com/satoshilabs/slips/blob/master/slip-0044.md)
//! coin types and the associated metadata.
//!
//! Parse IDs, coin names and symbols into pre-defined [`Coin`] and [`Symbol`] enums
//! and convert one into another.
//! ```
//! use std::{convert::TryFrom, str::FromStr};
//! use slip44::{Coin, Symbol};
//!
//! const BITCOIN_ID: u32 = Coin::Bitcoin.id(); // Coin IDs are constant
//!
//! fn main() {
//!     assert_eq!(BITCOIN_ID, 0);
//!     assert_eq!(Coin::Bitcoin.id(), 0);
//!     assert_eq!(Coin::Bitcoin.ids(), vec![0]); // Coin may have multiple IDs (e.g. Credits)
//!     assert_eq!(Coin::Bitcoin.name(), "Bitcoin");
//!     assert_eq!(Coin::Bitcoin.link(), Some("https://bitcoin.org/".to_string()));
//!     assert_eq!(Coin::Bitcoin.to_string(), "Bitcoin");
//!     
//!     assert_eq!(Coin::try_from(0), Ok(Coin::Bitcoin)); // Try to get Coin from its ID
//!     assert_eq!(Coin::from_str("Bitcoin"), Ok(Coin::Bitcoin));
//!     assert_eq!(Coin::from(Symbol::BTC), Coin::Bitcoin); // Get Coin from its Symbol (can't fail, all symbols have associated coins)
//!
//!     assert_eq!(Symbol::BTC.to_string(), "BTC");
//!     
//!     assert_eq!(Symbol::try_from(0), Ok(Symbol::BTC)); // Try to get coin Symbol from its ID
//!     assert_eq!(Symbol::try_from(Coin::Bitcoin), Ok(Symbol::BTC)); // Try to convert Coin to Symbol (can fail if no Symbol for Coin is specified)
//!     assert_eq!(Symbol::from_str("BTC"), Ok(Symbol::BTC));
//! }
//! ```
mod coin;
mod coins_macro;

pub use coin::*;
