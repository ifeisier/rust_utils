#![warn(missing_docs)]

//! 个人使用的 rust 工具库
//!
//! ## features
//!
//! - logger_ext: 日志工具
//! - rand_ext: 随机工具
//! - reqwest_ext: http 请求工具

#[cfg(feature = "logger_ext")]
pub mod logger;

#[cfg(feature = "rand_ext")]
pub mod rand;

#[cfg(feature = "reqwest_ext")]
pub mod reqwest;
