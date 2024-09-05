#![warn(missing_docs)]

//! 个人使用的 rust 工具库
//! 
//! ## features
//! 
//! - logger: 日志工具
//! - randx: 随机工具

#[cfg(feature = "logger")]
pub mod logger;

#[cfg(feature = "rand")]
pub mod rand;
