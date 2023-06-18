#![doc = include_str!("../README.md")]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![allow(clippy::needless_doctest_main)]
#![warn(
    missing_docs,
    clippy::missing_docs_in_private_items,
    clippy::unwrap_used
)]

pub use axum;

#[cfg_attr(docsrs, doc(cfg(feature = "esp")))]
#[cfg(feature = "esp")]
pub mod http_server;
mod macros;
