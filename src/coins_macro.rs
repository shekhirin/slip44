#[macro_export]
macro_rules! coins {
    ($((
        $(#[$meta:meta])*
        [$($id:expr),+],
        $ident:ident,
        $name:expr,
        $($link:expr)?,
        $($symbol:ident)?,
        $($duplicate_symbol:expr)?
        $(,)?
    )$(,)?),+) => {
        macro_rules! slip44_error {
            ($msg:expr) => {
                Err(concat!(
                    $msg,
                    ". See https://github.com/satoshilabs/slips/blob/master/slip-0044.md.",
                ))
            };
        }

        #[derive(Debug, PartialEq, Copy, Clone)]
        #[allow(non_camel_case_types)]
        /// Cryptocurrency according to [SLIP-0044](https://github.com/satoshilabs/slips/blob/master/slip-0044.md) spec.
        pub enum Coin {
            $(
                $(#[$meta])*
                $ident,
            )*
        }

        impl std::fmt::Display for Coin {
            /// ```
            /// use slip44::Coin;
            ///
            /// assert_eq!(Coin::Bitcoin.to_string(), "Bitcoin");
            /// ```
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", match self { $(Self::$ident => stringify!($ident), )* })
            }
        }

        impl Coin {
            /// Cryptocurrency ID according to [SLIP-0044](https://github.com/satoshilabs/slips/blob/master/slip-0044.md) spec.
            /// ```
            /// use slip44::Coin;
            ///
            /// assert_eq!(Coin::Bitcoin.id(), 0);
            /// ```
            pub fn id(self) -> u32 { self.ids()[0] }

            /// Cryptocurrency IDs according to [SLIP-0044](https://github.com/satoshilabs/slips/blob/master/slip-0044.md) spec.
            ///
            /// Cryptocurrencies may have multiple IDs if both name and symbol identical.
            /// ```
            /// use slip44::Coin;
            ///
            /// assert_eq!(Coin::Credits.ids(), vec![334, 498]);
            /// ```
            pub fn ids(self) -> Vec<u32> { match self { $(Self::$ident => vec![$($id),+], )* } }

            /// Cryptocurrency unedited name according to [SLIP-0044](https://github.com/satoshilabs/slips/blob/master/slip-0044.md) spec.
            /// ```
            /// use slip44::Coin;
            ///
            /// assert_eq!(Coin::UniformFiscalObject.name(), "Uniform Fiscal Object");
            /// ```
            pub fn name(self) -> String { match self { $(Self::$ident => $name.to_string(), )* } }

            /// Cryptocurrency link extracted from name according to [SLIP-0044](https://github.com/satoshilabs/slips/blob/master/slip-0044.md) spec.
            /// ```
            /// use slip44::Coin;
            ///
            /// assert_eq!(Coin::Bitcoin.link(), Some("https://bitcoin.org/".to_string()));
            /// ```
            pub fn link(self) -> Option<String> {
                match self {
                    $($(Self::$ident => Some($link.to_string()), )?)*
                    _ => None
                }
            }

            /// Cryptocurrency symbol that's not included into [Symbol] enum due to being a duplicate of another cryptocurrency by symbol name.
            /// ```
            /// use slip44::Coin;
            ///
            /// assert_eq!(Coin::CPChain.duplicate_symbol(), Some("CPC".to_string()));
            /// ```
            /// Such conflicts are resolved by taking only first Cryptocurrency (ordered by id) into [Symbol] enum
            /// (e.g. both [Coin::CPChain] and [Coin::Capricoin] has symbol "CPC" but only [Coin::Capricoin] is eligible to be linked to [Symbol::CPC] since
            /// ```
            /// use slip44::Coin;
            ///
            /// assert!(Coin::Capricoin.id() < Coin::CPChain.id());
            /// ```
            pub fn duplicate_symbol(self) -> Option<String> {
                match self {
                    $($(Self::$ident => Some($duplicate_symbol.to_string()), )?)*
                    _ => None
                }
            }
        }

        impl std::convert::TryFrom<u32> for Coin {
            type Error = &'static str;

            /// ```
            /// use std::convert::TryFrom;
            /// use slip44::Coin;
            ///
            /// assert_eq!(Coin::try_from(0), Ok(Coin::Bitcoin));
            /// assert!(Coin::try_from(2147483647).is_err());
            /// ```
            fn try_from(id: u32) -> Result<Self, Self::Error> {
                match id {
                    $($($id => Ok(Self::$ident), )+ )*
                    _ => slip44_error!("unknown coin type")
                }
            }
        }

        impl From<Symbol> for Coin {
            /// ```
            /// use slip44::{Coin, Symbol};
            ///
            /// assert_eq!(Coin::from(Symbol::BTC), Coin::Bitcoin);
            /// ```
            fn from(symbol: Symbol) -> Self {
                match symbol { $($(Symbol::$symbol => Self::$ident, )?)* }
            }
        }

        impl std::str::FromStr for Coin {
            type Err = &'static str;

            /// ```
            /// use std::str::FromStr;
            /// use slip44::Coin;
            ///
            /// assert_eq!(Coin::from_str("Uniform Fiscal Object"), Ok(Coin::UniformFiscalObject));
            /// assert!(Coin::from_str("Unknown Coin That's Definitely Not Exist").is_err());
            /// ```
            #[allow(unreachable_patterns)]
            fn from_str(s: &str) -> Result<Self, Self::Err> {
                match s {
                    $($name => Ok(Self::$ident), )*
                    _ => slip44_error!("unknown coin")
                }
            }
        }

        #[derive(Debug, PartialEq, Copy, Clone)]
        #[allow(non_camel_case_types)]
        /// Cryptocurrency symbol according to [SLIP-0044](https://github.com/satoshilabs/slips/blob/master/slip-0044.md) spec.
        pub enum Symbol { $($($symbol, )?)* }

        impl std::fmt::Display for Symbol {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", match self { $($(Self::$symbol => stringify!($symbol), )?)* })
            }
        }

        impl std::convert::TryFrom<u32> for Symbol {
            type Error = &'static str;

            /// ```
            /// use slip44::Symbol;
            ///
            /// assert_eq!(Symbol::BTC.to_string(), "BTC");
            /// ```
            fn try_from(id: u32) -> Result<Self, Self::Error> {
                Ok(Coin::try_from(id).map(Symbol::try_from)??)
            }
        }

        impl std::convert::TryFrom<Coin> for Symbol {
            type Error = &'static str;

            /// ```
            /// use std::convert::TryFrom;
            /// use slip44::{Coin, Symbol};
            ///
            /// assert_eq!(Symbol::try_from(Coin::Bitcoin), Ok(Symbol::BTC));
            /// assert!(Symbol::try_from(Coin::Testnet).is_err());
            /// ```
            fn try_from(coin: Coin) -> Result<Self, Self::Error> {
                match coin {
                    $($(Coin::$ident => Ok(Self::$symbol), )?)*
                    _ => slip44_error!("coin does not have associated symbol")
                }
            }
        }

        impl std::str::FromStr for Symbol {
            type Err = &'static str;

            /// ```
            /// use std::str::FromStr;
            /// use slip44::Symbol;
            ///
            /// assert_eq!(Symbol::from_str("BTC"), Ok(Symbol::BTC));
            /// assert!(Symbol::from_str("UNKNOWN").is_err());
            /// ```
            fn from_str(s: &str) -> Result<Self, Self::Err> {
                match s {
                    $($(stringify!($symbol) => Ok(Self::$symbol), )?)*
                    _ => slip44_error!("unknown symbol")
                }
            }
        }
    };
}

#[cfg(test)]
mod tests {
    use std::convert::TryFrom;
    use std::str::FromStr;

    coins!(
        (
            /// OG Crypto
            [0, 500], Bitcoin, "Bitcoin by Satoshi", "https://bitcoin.org", BTC,
        ),
        (
            /// Any Crypto's Testnet
            [1], Testnet, "Testnet (all coins)", , , "TSNT",
        ),
    );

    #[test]
    fn coin() {
        assert_eq!(Coin::Bitcoin.to_string(), "Bitcoin");
        assert_eq!(Coin::Bitcoin.id(), 0);
        assert_eq!(Coin::Bitcoin.ids(), vec![0, 500]);
        assert_eq!(Coin::Bitcoin.name(), "Bitcoin by Satoshi");
        assert_eq!(
            Coin::Bitcoin.link(),
            Some("https://bitcoin.org".to_string())
        );
        assert_eq!(Coin::Bitcoin.duplicate_symbol(), None);

        assert_eq!(Coin::try_from(0), Ok(Coin::Bitcoin));
        assert!(Coin::try_from(100).is_err());

        assert_eq!(Coin::from(Symbol::BTC), Coin::Bitcoin);

        assert_eq!(Coin::Testnet.link(), None);
        assert_eq!(Coin::Testnet.duplicate_symbol(), Some("TSNT".to_string()));

        assert_eq!(Coin::from_str("Bitcoin by Satoshi"), Ok(Coin::Bitcoin));
        assert!(Coin::from_str("Somecoin").is_err());
    }

    #[test]
    fn symbol() {
        assert_eq!(Symbol::BTC.to_string(), "BTC");

        assert_eq!(Symbol::try_from(0), Ok(Symbol::BTC));
        assert!(Symbol::try_from(2).is_err());

        assert_eq!(Symbol::try_from(Coin::Bitcoin), Ok(Symbol::BTC));
        assert!(Symbol::try_from(Coin::Testnet).is_err());

        assert_eq!(Symbol::from_str("BTC"), Ok(Symbol::BTC));
        assert!(Symbol::from_str("TBC").is_err());
    }
}
