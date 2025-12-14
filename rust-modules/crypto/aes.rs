//!AES 对称加密模块
//!
//!提供 AES-256-GCM（推荐）和 AES-256-CBC 加密功能。
//!
//!依赖：
//!- aes-gcm（使用时查询最新版本：https://crates.io/crates/aes-gcm）
//!- aes（使用时查询最新版本：https://crates.io/crates/aes）
//!- cbc（使用时查询最新版本：https://crates.io/crates/cbc）
//!- rand（使用时查询最新版本：https://crates.io/crates/rand）
//!
//!# AES-GCM vs AES-CBC
//!- AES-GCM：带认证的加密，能检测数据篡改，推荐使用
//!- AES-CBC：传统模式，需要自行处理数据完整性校验
//!
//!# 示例
//!```rust
//!use crypto::aes;
//!
//!//AES-GCM 加密（推荐）
//!let key = aes::generate_key();
//!let nonce = aes::generate_nonce();
//!let encrypted = aes::gcm_encrypt(&key, &nonce, b"hello").unwrap();
//!let decrypted = aes::gcm_decrypt(&key, &nonce, &encrypted).unwrap();
//!```

use aes_gcm::{
    Aes256Gcm,
    aead::{Aead, KeyInit},
};
use rand::RngCore;

//========================================
//密钥和随机数生成
//========================================

///生成 AES-256 密钥（32字节）
pub fn generate_key() -> [u8; super::config::AES_KEY_SIZE] {
    let mut key = [0u8; super::config::AES_KEY_SIZE];
    rand::thread_rng().fill_bytes(&mut key);
    key
}

///生成 AES-GCM Nonce（12字节）
pub fn generate_nonce() -> [u8; super::config::AES_GCM_NONCE_SIZE] {
    let mut nonce = [0u8; super::config::AES_GCM_NONCE_SIZE];
    rand::thread_rng().fill_bytes(&mut nonce);
    nonce
}

///生成 AES-CBC IV（16字节）
pub fn generate_iv() -> [u8; super::config::AES_CBC_IV_SIZE] {
    let mut iv = [0u8; super::config::AES_CBC_IV_SIZE];
    rand::thread_rng().fill_bytes(&mut iv);
    iv
}

//========================================
//AES-GCM 加密（推荐）
//带认证的加密，能检测数据篡改
//========================================

///AES-256-GCM 加密
///
///# 参数
///- key: 32字节密钥
///- nonce: 12字节随机数（每次加密必须不同）
///- plaintext: 明文数据
///
///# 返回
///加密后的密文（包含认证标签）
pub fn gcm_encrypt(key: &[u8; 32], nonce: &[u8; 12], plaintext: &[u8]) -> Result<Vec<u8>, String> {
    let cipher = Aes256Gcm::new_from_slice(key)
        .map_err(|e| format!("创建加密器失败: {}", e))?;

    let nonce = aes_gcm::Nonce::from_slice(nonce);

    cipher.encrypt(nonce, plaintext)
        .map_err(|e| format!("加密失败: {}", e))
}

///AES-256-GCM 解密
///
///# 参数
///- key: 32字节密钥
///- nonce: 12字节随机数（必须与加密时相同）
///- ciphertext: 密文数据（包含认证标签）
///
///# 返回
///解密后的明文
pub fn gcm_decrypt(key: &[u8; 32], nonce: &[u8; 12], ciphertext: &[u8]) -> Result<Vec<u8>, String> {
    let cipher = Aes256Gcm::new_from_slice(key)
        .map_err(|e| format!("创建解密器失败: {}", e))?;

    let nonce = aes_gcm::Nonce::from_slice(nonce);

    cipher.decrypt(nonce, ciphertext)
        .map_err(|e| format!("解密失败（数据可能被篡改）: {}", e))
}

//========================================
//AES-CBC 加密
//传统模式，不带认证
//========================================

use aes::cipher::{BlockEncryptMut, BlockDecryptMut, KeyIvInit};
use aes::cipher::block_padding::Pkcs7;

type Aes256CbcEnc = cbc::Encryptor<aes::Aes256>;
type Aes256CbcDec = cbc::Decryptor<aes::Aes256>;

///AES-256-CBC 加密
///
///# 参数
///- key: 32字节密钥
///- iv: 16字节初始化向量
///- plaintext: 明文数据
///
///# 返回
///加密后的密文（已填充）
pub fn cbc_encrypt(key: &[u8; 32], iv: &[u8; 16], plaintext: &[u8]) -> Vec<u8> {
    let cipher = Aes256CbcEnc::new_from_slices(key, iv).unwrap();
    cipher.encrypt_padded_vec_mut::<Pkcs7>(plaintext)
}

///AES-256-CBC 解密
///
///# 参数
///- key: 32字节密钥
///- iv: 16字节初始化向量（必须与加密时相同）
///- ciphertext: 密文数据
///
///# 返回
///解密后的明文
pub fn cbc_decrypt(key: &[u8; 32], iv: &[u8; 16], ciphertext: &[u8]) -> Result<Vec<u8>, String> {
    let cipher = Aes256CbcDec::new_from_slices(key, iv).unwrap();
    cipher.decrypt_padded_vec_mut::<Pkcs7>(ciphertext)
        .map_err(|e| format!("解密失败: {:?}", e))
}

//========================================
//便捷函数
//========================================

///简单加密（自动生成 nonce，返回 nonce + 密文）
pub fn encrypt_simple(key: &[u8; 32], plaintext: &[u8]) -> Result<Vec<u8>, String> {
    let nonce = generate_nonce();
    let ciphertext = gcm_encrypt(key, &nonce, plaintext)?;

    //nonce + ciphertext
    let mut result = Vec::with_capacity(nonce.len() + ciphertext.len());
    result.extend_from_slice(&nonce);
    result.extend_from_slice(&ciphertext);
    Ok(result)
}

///简单解密（从数据中提取 nonce）
pub fn decrypt_simple(key: &[u8; 32], data: &[u8]) -> Result<Vec<u8>, String> {
    if data.len() < super::config::AES_GCM_NONCE_SIZE {
        return Err("数据太短".to_string());
    }

    let (nonce_bytes, ciphertext) = data.split_at(super::config::AES_GCM_NONCE_SIZE);
    let nonce: [u8; 12] = nonce_bytes.try_into().unwrap();

    gcm_decrypt(key, &nonce, ciphertext)
}
