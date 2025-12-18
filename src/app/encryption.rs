use aes_gcm::{
    Aes256Gcm, Nonce,
    aead::{Aead, KeyInit, OsRng},
};
use hex;

/// Генерирует случайный ключ AES-256 (32 байта)
pub fn generate_key() -> [u8; 32] {
    use aes_gcm::aead::rand_core::RngCore;
    let mut key = [0u8; 32];
    OsRng.fill_bytes(&mut key);
    key
}

/// Генерирует случайный nonce (12 байт для GCM)
pub fn generate_nonce() -> [u8; 12] {
    use aes_gcm::aead::rand_core::RngCore;
    let mut nonce = [0u8; 12];
    OsRng.fill_bytes(&mut nonce);
    nonce
}

/// Шифрует сообщение с AES-256-GCM
/// Возвращает (ключ_hex, nonce_hex, зашифрованные_данные_hex)
pub fn encrypt(plaintext: &str) -> Result<(String, String, String), String> {
    // Генерируем ключ
    let key_bytes = generate_key();
    let key = aes_gcm::Key::<Aes256Gcm>::from_slice(&key_bytes);

    // Создаем cipher
    let cipher = Aes256Gcm::new(&key);

    // Генерируем nonce
    let nonce_bytes = generate_nonce();
    let nonce = Nonce::from_slice(&nonce_bytes);

    // Шифруем
    let ciphertext = cipher
        .encrypt(nonce, plaintext.as_bytes())
        .map_err(|e| format!("Ошибка шифрования: {:?}", e))?;

    // Конвертируем в hex
    let key_hex = hex::encode(key_bytes);
    let nonce_hex = hex::encode(nonce_bytes);
    let ciphertext_hex = hex::encode(ciphertext);

    Ok((key_hex, nonce_hex, ciphertext_hex))
}

/// Дешифрует сообщение с AES-256-GCM
pub fn decrypt(key_hex: &str, nonce_hex: &str, ciphertext_hex: &str) -> Result<String, String> {
    // Декодируем из hex
    let key_bytes =
        hex::decode(key_hex).map_err(|e| format!("Ошибка декодирования ключа: {}", e))?;
    let nonce_bytes =
        hex::decode(nonce_hex).map_err(|e| format!("Ошибка декодирования nonce: {}", e))?;
    let ciphertext = hex::decode(ciphertext_hex)
        .map_err(|e| format!("Ошибка декодирования ciphertext: {}", e))?;

    // Проверяем размеры
    if key_bytes.len() != 32 {
        return Err(format!(
            "Неверный размер ключа: {} байт (ожидается 32)",
            key_bytes.len()
        ));
    }
    if nonce_bytes.len() != 12 {
        return Err(format!(
            "Неверный размер nonce: {} байт (ожидается 12)",
            nonce_bytes.len()
        ));
    }

    // Создаем cipher
    let key = aes_gcm::Key::<Aes256Gcm>::from_slice(&key_bytes);
    let cipher = Aes256Gcm::new(&key);
    let nonce = Nonce::from_slice(&nonce_bytes);

    // Дешифруем
    let plaintext = cipher
        .decrypt(nonce, ciphertext.as_ref())
        .map_err(|e| format!("Ошибка дешифрования: {:?}", e))?;

    // Конвертируем в строку
    String::from_utf8(plaintext).map_err(|e| format!("Ошибка UTF-8: {}", e))
}
