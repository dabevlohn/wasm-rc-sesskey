use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine as _};
use rand_core::RngCore;
use reqwest;
use rsa::pkcs1::DecodeRsaPublicKey;
use rsa::{Pkcs1v15Encrypt, RsaPublicKey};

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub async fn generate_aes_key() -> Result<(), JsValue> {
    // Случайные данные для шифрования
    let mut rng = rand_core::OsRng;
    let mut data = [0u8; 16];
    rng.fill_bytes(&mut data);

    // Публичный RSA-ключ
    let public_key = RsaPublicKey::from_pkcs1_der(include_bytes!("public_key.der"))
        .expect("Failed to load public key");

    let b64uridata = URL_SAFE_NO_PAD.encode(data);
    // Шифрование данных
    let encrypted_data = public_key
        .encrypt(&mut rng, Pkcs1v15Encrypt, &data)
        .expect("Encryption failed");

    Ok(())
}

#[wasm_bindgen]
pub async fn store_key(encdata: Vec<u8>) -> Result<(), JsValue> {
    // Отправка данных на сервер
    let url = "https://example.com/api/encrypted_data"; // Замените на нужный URL
    let client = reqwest::Client::new();
    let response = client
        .post(url)
        .body(encdata)
        .send()
        .await
        .map_err(|e| JsValue::from_str(&format!("Request error: {}", e)))?;

    if response.status().is_success() {
        Ok(())
    } else {
        Err(JsValue::from_str(&format!(
            "Server error: {}",
            response.status()
        )))
    }
}
