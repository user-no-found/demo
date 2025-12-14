//!加密工具模块
//!
//!提供常用的加密/解密功能，包括哈希算法、对称加密和非对称加密。
//!
//!# 目录结构
//!```
//!crypto/
//!├── mod.rs      # 模块入口（本文件）
//!├── config.rs   # 配置项
//!├── hash.rs     # 哈希算法（MD5/SHA256/SHA512）
//!├── aes.rs      # AES 对称加密
//!└── rsa.rs      # RSA 非对称加密
//!```
//!
//!# Cargo.toml 依赖
//!```toml
//![dependencies]
//!sha2 = "0.10"      # https://crates.io/crates/sha2
//!md-5 = "0.10"      # https://crates.io/crates/md-5
//!aes-gcm = "0.10"   # https://crates.io/crates/aes-gcm
//!aes = "0.8"        # https://crates.io/crates/aes
//!cbc = "0.1"        # https://crates.io/crates/cbc
//!rsa = "0.9"        # https://crates.io/crates/rsa
//!rand = "0.8"       # https://crates.io/crates/rand
//!hex = "0.4"        # https://crates.io/crates/hex
//!```
//!
//!> 注：使用前请到 crates.io 查询依赖的最新版本
//!
//!# 快速开始
//!
//!## 哈希
//!```rust
//!mod crypto;
//!
//!fn main() {
//!    let hash = crypto::hash::sha256("hello");
//!    println!("SHA256: {}", hash);
//!
//!    let md5 = crypto::hash::md5("hello");
//!    println!("MD5: {}", md5);
//!}
//!```
//!
//!## AES 加密
//!```rust
//!mod crypto;
//!
//!fn main() {
//!    //AES-GCM（推荐，带认证）
//!    let key = crypto::aes::generate_key();
//!    let nonce = crypto::aes::generate_nonce();
//!    let encrypted = crypto::aes::gcm_encrypt(&key, &nonce, b"hello").unwrap();
//!    let decrypted = crypto::aes::gcm_decrypt(&key, &nonce, &encrypted).unwrap();
//!}
//!```
//!
//!## RSA 加密
//!```rust
//!mod crypto;
//!
//!fn main() {
//!    //生成密钥对
//!    let (public, private) = crypto::rsa::generate_keypair(2048).unwrap();
//!
//!    //加密/解密
//!    let encrypted = crypto::rsa::encrypt(&public, b"hello").unwrap();
//!    let decrypted = crypto::rsa::decrypt(&private, &encrypted).unwrap();
//!
//!    //签名/验签
//!    let signature = crypto::rsa::sign(&private, b"message").unwrap();
//!    let valid = crypto::rsa::verify(&public, b"message", &signature).unwrap();
//!}
//!```

pub mod config;
pub mod hash;
pub mod aes;
pub mod rsa;

//重新导出常用类型
pub use hash::{md5, sha256, sha512};
pub use aes::{gcm_encrypt, gcm_decrypt};
