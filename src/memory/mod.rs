//! 内存模块
//!
//! 提供一些和内存相关的工具

/// 获取引用的内存地址
///
/// # 参数
///
/// - s: 想要获取内存地址的引用
///
/// # 返回
///
/// - String: 十六进制的内存地址
pub fn get_memory_address<T>(s: &T) -> String {
    let x_ptr = s as *const T;
    format!("{:p}", x_ptr)
}
