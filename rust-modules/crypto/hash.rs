//!哈希算法模块
//!
//!提供 MD5、SHA256、SHA512 等常用哈希算法。
//!
//!依赖：
//!- sha2（使用时查询最新版本：https://crates.io/crates/sha2）
//!- md-5（使用时查询最新版本：https://crates.io/crates/md-5）
//!- hex（使用时查询最新版本：https://crates.io/crates/hex）
//!
//!# 示例
//!```rust
//!use crypto::hash;
//!
//!let md5_hash = hash::md5("hello");
//!let sha256_hash = hash::sha256("hello");
//!let sha512_hash = hash::sha512("hello");
//!```

use sha2::Digest;

//========================================
//MD5 哈希
//警告：MD5 已不安全，仅用于兼容旧系统
//========================================

///计算字符串的 MD5 哈希值
pub fn md5(data: &str) -> String {
    md5_bytes(data.as_bytes())
}

///计算字节数据的 MD5 哈希值
pub fn md5_bytes(data: &[u8]) -> String {
    let mut hasher = md5::Md5::new();
    hasher.update(data);
    let result = hasher.finalize();
    to_hex(&result)
}

//========================================
//SHA256 哈希（推荐）
//========================================

///计算字符串的 SHA256 哈希值
pub fn sha256(data: &str) -> String {
    sha256_bytes(data.as_bytes())
}

///计算字节数据的 SHA256 哈希值
pub fn sha256_bytes(data: &[u8]) -> String {
    let mut hasher = sha2::Sha256::new();
    hasher.update(data);
    let result = hasher.finalize();
    to_hex(&result)
}

//========================================
//SHA512 哈希
//========================================

///计算字符串的 SHA512 哈希值
pub fn sha512(data: &str) -> String {
    sha512_bytes(data.as_bytes())
}

///计算字节数据的 SHA512 哈希值
pub fn sha512_bytes(data: &[u8]) -> String {
    let mut hasher = sha2::Sha512::new();
    hasher.update(data);
    let result = hasher.finalize();
    to_hex(&result)
}

//========================================
//辅助函数
//========================================

///字节转十六进制字符串
fn to_hex(bytes: &[u8]) -> String {
    if super::config::HASH_UPPERCASE {
        hex::encode_upper(bytes)
    } else {
        hex::encode(bytes)
    }
}

///十六进制字符串转字节
pub fn from_hex(hex_str: &str) -> Result<Vec<u8>, hex::FromHexError> {
    hex::decode(hex_str)
}

//========================================
//HMAC（可选，需要额外依赖 hmac 库）
//========================================

//如需 HMAC 功能，添加依赖：
//hmac = "0.12"  # https://crates.io/crates/hmac
//
//示例：
//use hmac::{Hmac, Mac};
//type HmacSha256 = Hmac<sha2::Sha256>;
//
//pub fn hmac_sha256(key: &[u8], data: &[u8]) -> Vec<u8> {
//    let mut mac = HmacSha256::new_from_slice(key).unwrap();
//    mac.update(data);
//    mac.finalize().into_bytes().to_vec()
//}
