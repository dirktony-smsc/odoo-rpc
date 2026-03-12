pub mod client;
pub mod error;
pub mod utils;

pub trait ModelName {
    const NAME: &'static str;
}

pub use client::OdooJsonRPCClient;
