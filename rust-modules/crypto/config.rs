//!加密配置模块
//!
//!统一管理加密相关的配置项。

//========================================
//RSA 配置
//========================================

///RSA 默认密钥长度（位）
pub const RSA_DEFAULT_BITS: usize = 2048;

///RSA 最小密钥长度（位）
pub const RSA_MIN_BITS: usize = 1024;

///RSA 最大密钥长度（位）
pub const RSA_MAX_BITS: usize = 4096;

//========================================
//AES 配置
//========================================

///AES 密钥长度（字节，32 = AES-256）
pub const AES_KEY_SIZE: usize = 32;

///AES-GCM Nonce 长度（字节）
pub const AES_GCM_NONCE_SIZE: usize = 12;

///AES-CBC IV 长度（字节）
pub const AES_CBC_IV_SIZE: usize = 16;

//========================================
//哈希配置
//========================================

///是否使用大写十六进制输出
pub const HASH_UPPERCASE: bool = false;
