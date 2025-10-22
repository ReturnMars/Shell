// 加密工具 - 使用AES-GCM进行安全加密
#![allow(deprecated)] // 允许aes-gcm内部的deprecated警告

use aes_gcm::{Aes256Gcm, Key, Nonce, KeyInit};
use aes_gcm::aead::Aead;
use rand::Rng;
use base64::{Engine as _, engine::general_purpose};

/// 加密密码
pub fn encrypt_password(password: &str, master_key: &[u8]) -> Result<String, String> {
    // 生成随机nonce
    let mut rng = rand::thread_rng();
    let mut nonce_bytes = [0u8; 12];
    rng.fill(&mut nonce_bytes);
    
    let key = Key::<Aes256Gcm>::from_slice(master_key);
    let cipher = Aes256Gcm::new(key);
    let nonce = Nonce::from_slice(&nonce_bytes);
    
    // 加密密码
    let ciphertext = cipher.encrypt(nonce, password.as_bytes())
        .map_err(|e| format!("加密失败: {}", e))?;
    
    // 将nonce和密文组合并base64编码
    let mut result = nonce_bytes.to_vec();
    result.extend_from_slice(&ciphertext);
    
    Ok(general_purpose::STANDARD.encode(result))
}

/// 解密密码
pub fn decrypt_password(encrypted: &str, master_key: &[u8]) -> Result<String, String> {
    // base64解码
    let data = general_purpose::STANDARD.decode(encrypted)
        .map_err(|e| format!("base64解码失败: {}", e))?;
    
    if data.len() < 12 {
        return Err("数据太短".to_string());
    }
    
    // 分离nonce和密文
    let nonce_bytes = &data[..12];
    let ciphertext = &data[12..];
    
    let key = Key::<Aes256Gcm>::from_slice(master_key);
    let cipher = Aes256Gcm::new(key);
    let nonce = Nonce::from_slice(nonce_bytes);
    
    // 解密
    let plaintext = cipher.decrypt(nonce, ciphertext)
        .map_err(|e| format!("解密失败: {}", e))?;
    
    String::from_utf8(plaintext)
        .map_err(|e| format!("UTF8转换失败: {}", e))
}

/// 生成主密钥（用于加密存储的密码）
pub fn generate_master_key() -> Vec<u8> {
    let mut key = [0u8; 32]; // AES-256需要32字节密钥
    rand::thread_rng().fill(&mut key);
    key.to_vec()
}

/// 从用户输入生成密钥（使用PBKDF2）
pub fn derive_key_from_password(password: &str, salt: &[u8]) -> Vec<u8> {
    use pbkdf2::{pbkdf2_hmac};
    use sha2::Sha256;
    
    let mut key = [0u8; 32];
    pbkdf2_hmac::<Sha256>(password.as_bytes(), salt, 100000, &mut key);
    key.to_vec()
}