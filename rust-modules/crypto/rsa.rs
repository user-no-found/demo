//!RSA 非对称加密模块
//!
//!提供 RSA 密钥生成、加密/解密、签名/验签功能。
//!
//!依赖：
//!- rsa（使用时查询最新版本：https://crates.io/crates/rsa）
//!- rand（使用时查询最新版本：https://crates.io/crates/rand）
//!
//!# 示例
//!```rust
//!use crypto::rsa;
//!
//!//生成密钥对
//!let (public, private) = rsa::generate_keypair(2048).unwrap();
//!
//!//加密/解密
//!let encrypted = rsa::encrypt(&public, b"hello").unwrap();
//!let decrypted = rsa::decrypt(&private, &encrypted).unwrap();
//!
//!//签名/验签
//!let signature = rsa::sign(&private, b"message").unwrap();
//!let valid = rsa::verify(&public, b"message", &signature).unwrap();
//!```

use rsa::{RsaPrivateKey, RsaPublicKey};
use rsa::pkcs1v15::{SigningKey, VerifyingKey};
use rsa::signature::{Signer, Verifier};

//========================================
//类型别名
//========================================

///RSA 公钥类型
pub type PublicKey = RsaPublicKey;

///RSA 私钥类型
pub type PrivateKey = RsaPrivateKey;

//========================================
//密钥生成
//========================================

///生成 RSA 密钥对
///
///# 参数
///- bits: 密钥长度（推荐 2048 或 4096）
///
///# 返回
///(公钥, 私钥)
pub fn generate_keypair(bits: usize) -> Result<(PublicKey, PrivateKey), String> {
    if bits < super::config::RSA_MIN_BITS {
        return Err(format!("密钥长度至少 {} 位", super::config::RSA_MIN_BITS));
    }
    if bits > super::config::RSA_MAX_BITS {
        return Err(format!("密钥长度最多 {} 位", super::config::RSA_MAX_BITS));
    }

    let mut rng = rand::thread_rng();
    let private_key = RsaPrivateKey::new(&mut rng, bits)
        .map_err(|e| format!("生成密钥失败: {}", e))?;
    let public_key = RsaPublicKey::from(&private_key);

    Ok((public_key, private_key))
}

///使用默认位数生成密钥对
pub fn generate_keypair_default() -> Result<(PublicKey, PrivateKey), String> {
    generate_keypair(super::config::RSA_DEFAULT_BITS)
}

//========================================
//加密/解密
//========================================

///RSA 公钥加密
///
///# 注意
///RSA 加密有长度限制，明文长度不能超过 (密钥长度/8 - 11) 字节
///对于 2048 位密钥，最大明文长度为 245 字节
///如需加密大数据，应结合 AES 使用（RSA 加密 AES 密钥）
pub fn encrypt(public_key: &PublicKey, plaintext: &[u8]) -> Result<Vec<u8>, String> {
    let mut rng = rand::thread_rng();
    let padding = rsa::Pkcs1v15Encrypt;

    public_key.encrypt(&mut rng, padding, plaintext)
        .map_err(|e| format!("加密失败: {}", e))
}

///RSA 私钥解密
pub fn decrypt(private_key: &PrivateKey, ciphertext: &[u8]) -> Result<Vec<u8>, String> {
    let padding = rsa::Pkcs1v15Encrypt;

    private_key.decrypt(padding, ciphertext)
        .map_err(|e| format!("解密失败: {}", e))
}

//========================================
//签名/验签
//========================================

///RSA 私钥签名（SHA256）
pub fn sign(private_key: &PrivateKey, message: &[u8]) -> Result<Vec<u8>, String> {
    let signing_key = SigningKey::<sha2::Sha256>::new(private_key.clone());
    let signature = signing_key.sign(message);
    Ok(signature.to_vec())
}

///RSA 公钥验签（SHA256）
pub fn verify(public_key: &PublicKey, message: &[u8], signature: &[u8]) -> Result<bool, String> {
    let verifying_key = VerifyingKey::<sha2::Sha256>::new(public_key.clone());
    let sig = rsa::pkcs1v15::Signature::try_from(signature)
        .map_err(|e| format!("签名格式错误: {}", e))?;

    Ok(verifying_key.verify(message, &sig).is_ok())
}

//========================================
//密钥序列化（PEM 格式）
//需要额外依赖：rsa = { version = "0.9", features = ["pem"] }
//========================================

//导出公钥为 PEM 格式
//pub fn public_key_to_pem(key: &PublicKey) -> Result<String, String> {
//    use rsa::pkcs8::EncodePublicKey;
//    key.to_public_key_pem(rsa::pkcs8::LineEnding::LF)
//        .map_err(|e| format!("导出公钥失败: {}", e))
//}

//从 PEM 格式导入公钥
//pub fn public_key_from_pem(pem: &str) -> Result<PublicKey, String> {
//    use rsa::pkcs8::DecodePublicKey;
//    RsaPublicKey::from_public_key_pem(pem)
//        .map_err(|e| format!("导入公钥失败: {}", e))
//}

//导出私钥为 PEM 格式
//pub fn private_key_to_pem(key: &PrivateKey) -> Result<String, String> {
//    use rsa::pkcs8::EncodePrivateKey;
//    key.to_pkcs8_pem(rsa::pkcs8::LineEnding::LF)
//        .map(|s| s.to_string())
//        .map_err(|e| format!("导出私钥失败: {}", e))
//}

//从 PEM 格式导入私钥
//pub fn private_key_from_pem(pem: &str) -> Result<PrivateKey, String> {
//    use rsa::pkcs8::DecodePrivateKey;
//    RsaPrivateKey::from_pkcs8_pem(pem)
//        .map_err(|e| format!("导入私钥失败: {}", e))
//}

//========================================
//混合加密（RSA + AES）
//用于加密大数据
//========================================

///混合加密：生成随机 AES 密钥，用 RSA 加密 AES 密钥，用 AES 加密数据
pub fn encrypt_hybrid(public_key: &PublicKey, plaintext: &[u8]) -> Result<Vec<u8>, String> {
    //生成 AES 密钥和 nonce
    let aes_key = super::aes::generate_key();
    let nonce = super::aes::generate_nonce();

    //用 RSA 加密 AES 密钥
    let encrypted_key = encrypt(public_key, &aes_key)?;

    //用 AES 加密数据
    let encrypted_data = super::aes::gcm_encrypt(&aes_key, &nonce, plaintext)?;

    //组装：密钥长度(2字节) + 加密的AES密钥 + nonce + 密文
    let key_len = encrypted_key.len() as u16;
    let mut result = Vec::new();
    result.extend_from_slice(&key_len.to_be_bytes());
    result.extend_from_slice(&encrypted_key);
    result.extend_from_slice(&nonce);
    result.extend_from_slice(&encrypted_data);

    Ok(result)
}

///混合解密
pub fn decrypt_hybrid(private_key: &PrivateKey, data: &[u8]) -> Result<Vec<u8>, String> {
    if data.len() < 2 {
        return Err("数据太短".to_string());
    }

    //解析密钥长度
    let key_len = u16::from_be_bytes([data[0], data[1]]) as usize;

    if data.len() < 2 + key_len + 12 {
        return Err("数据格式错误".to_string());
    }

    //提取加密的 AES 密钥
    let encrypted_key = &data[2..2 + key_len];

    //提取 nonce
    let nonce_start = 2 + key_len;
    let nonce: [u8; 12] = data[nonce_start..nonce_start + 12]
        .try_into()
        .map_err(|_| "nonce 格式错误")?;

    //提取密文
    let ciphertext = &data[nonce_start + 12..];

    //解密 AES 密钥
    let aes_key_vec = decrypt(private_key, encrypted_key)?;
    let aes_key: [u8; 32] = aes_key_vec.try_into()
        .map_err(|_| "AES 密钥长度错误")?;

    //解密数据
    super::aes::gcm_decrypt(&aes_key, &nonce, ciphertext)
}
