#![doc = include_str!("../README.md")]
#![warn(missing_docs)]
#![warn(clippy::missing_docs_in_private_items)]
#![warn(clippy::unwrap_used)]
#![warn(unreachable_pub)]

pub mod axum;
#[cfg(feature = "esp")]
pub mod http_server;
mod macros;
