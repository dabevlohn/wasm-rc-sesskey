use base64::{Engine as _, engine::general_purpose::URL_SAFE_NO_PAD};
use rand_core::RngCore;
use reqwest;
use rsa::pkcs1::DecodeRsaPublicKey;
use rsa::{Pkcs1v15Encrypt, RsaPublicKey};

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub async fn generate_aes_key() -> String {
    // Случайные данные для шифрования
    let mut rng = rand_core::OsRng;
    let mut data = [0u8; 16];
    rng.fill_bytes(&mut data);

    // Публичный RSA-ключ
    let public_key = RsaPublicKey::from_pkcs1_der(include_bytes!("public_key.der"))
        .expect("Failed to load public key");

    // Шифрование данных
    let encrypted_data = public_key
        .encrypt(&mut rng, Pkcs1v15Encrypt, &data)
        .expect("Encryption failed");
    store_key(encrypted_data).await;

    URL_SAFE_NO_PAD.encode(data)
}

#[wasm_bindgen]
pub async fn store_key(encdata: Vec<u8>) {
    // Отправка данных на сервер
    let url = "https://example.com/api/encrypted_data";
    let client = reqwest::Client::new();
    let response = client.post(url).body(encdata).send().await.unwrap();

    if response.status().is_success() {
        log("Data sent successfully");
    } else {
        log(response.status().as_str());
    }
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    pub fn log(s: &str);
}
