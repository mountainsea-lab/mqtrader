use std::str::FromStr;

pub mod bot;
pub mod config;
pub mod service;
pub mod trading_kernel;
pub mod wrapper;

#[derive(Debug, Clone)]
pub enum Exchange {
    BINANCE,
    OKX,
}

impl FromStr for Exchange {
    type Err = String; // Define the error type in case the string can't be converted

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            // Convert the string to uppercase for case-insensitive comparison
            "BINANCE" => Ok(Exchange::BINANCE),
            "OKX" => Ok(Exchange::OKX),
            _ => Err(format!("Unknown exchange: {}", s)),
        }
    }
}

impl ToString for Exchange {
    fn to_string(&self) -> String {
        match self {
            Exchange::BINANCE => "BINANCE".to_string(),
            Exchange::OKX => "OKX".to_string(),
        }
    }
}
