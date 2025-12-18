use aes_gcm::aead::rand_core::RngCore;
use aes_gcm::{
    Aes256Gcm, Nonce,
    aead::{Aead, KeyInit, OsRng},
};
use hex;

/// Генерирует случайный ключ AES-256 (32 байта)
pub fn generate_key() -> [u8; 32] {
    let mut key = [0u8; 32];
    OsRng.fill_bytes(&mut key);
    key
}

/// Генерирует случайный nonce (12 байт для GCM)
pub fn generate_nonce() -> [u8; 12] {
    let mut nonce = [0u8; 12];
    OsRng.fill_bytes(&mut nonce);
    nonce
}

pub fn encrypt(plaintext: &str, key_hex: &str, nonce_hex: &str) -> Result<String, String> {
    // Декодируем key и nonce из hex
    let key_bytes =
        hex::decode(key_hex).map_err(|e| format!("Ошибка декодирования ключа: {}", e))?;
    let nonce_bytes =
        hex::decode(nonce_hex).map_err(|e| format!("Ошибка декодирования nonce: {}", e))?;

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

    // Шифруем
    let ciphertext = cipher
        .encrypt(nonce, plaintext.as_bytes())
        .map_err(|e| format!("Ошибка шифрования: {:?}", e))?;

    // Возвращаем только ciphertext в hex
    Ok(hex::encode(ciphertext))
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn go_encryption() {
        let message = "Привет, это секретное сообщение! ку 888 :) ку 888 :)";
        let key_hex = hex::encode(generate_key());
        let nonce_hex = hex::encode(generate_nonce());

        // Шифруем, передавая ключ и nonce в hex
        let ciphertext_hex = encrypt(message, &key_hex, &nonce_hex).unwrap();
        assert_ne!(ciphertext_hex, message);

        // Дешифруем
        let decrypted = decrypt(&key_hex, &nonce_hex, &ciphertext_hex).unwrap();
        assert_eq!(decrypted, message);
    }
    #[test]
    fn encryption_text() {
        let message = "Привет, это секретное сообщение! ку 888 :) ку 888 :)";
        let key_hex = hex::encode(generate_key());
        let nonce_hex = hex::encode(generate_nonce());

        // Шифруем, передавая ключ и nonce в hex
        let ciphertext_hex = encrypt(message, &key_hex, &nonce_hex).unwrap();
        assert_ne!(ciphertext_hex, message);
    }

    #[test]
    fn generate_len() {
        assert_eq!(hex::encode(generate_key()).len(), 64);
        assert_eq!(hex::encode(generate_nonce()).len(), 24);
    }

    #[test]
    fn generate() {
        assert_ne!(hex::encode(generate_key()), hex::encode(generate_key()));
        assert_ne!(hex::encode(generate_nonce()), hex::encode(generate_nonce()));
    }

    #[test]
    fn incorrectkeysize() {
        let key_hex =
            hex::encode("f6bc1dca092e518d71c66d44d04a8508f79961b3addeb41a0812818611eb1812");
        let nonce_hex = hex::encode("f6bc1dca092e518d71c66d44d04a8611eb1812");
        let nonce_hex_ok = hex::encode(generate_nonce());
        let key_hex_ok = hex::encode(generate_key());

        // Шифруем, передавая ключ и nonce в hex
        decrypt(&key_hex, &nonce_hex, "Привет :(").unwrap_err();
        encrypt("Привет", &key_hex, &nonce_hex_ok).unwrap_err();
        decrypt(&key_hex, &nonce_hex_ok, &hex::encode("Привет")).unwrap_err();
        encrypt("Привет", &key_hex_ok, &nonce_hex).unwrap_err();
        decrypt(&key_hex_ok, &nonce_hex, &hex::encode("Привет")).unwrap_err();
    }
}
