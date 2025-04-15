use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine as _};
use gloo_utils::format::JsValueSerdeExt;
use rand_core::RngCore;
use reqwest;
use rsa::pkcs1::DecodeRsaPublicKey;
use rsa::{Pkcs1v15Encrypt, RsaPublicKey};
use serde_json::json;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub async fn generate_aes_key() -> JsValue {
    // Случайные данные для шифрования
    let mut rng = rand_core::OsRng;
    let mut data = [0u8; 16];
    rng.fill_bytes(&mut data);

    //store_key(data).await;

    let sesskey = json!({
        "kty": "oct",
        "alg": "A128CBC",
        "k": format!("{}",URL_SAFE_NO_PAD.encode(data)),
        "ext": true,
        "key_ops": ["encrypt", "decrypt"]
    });
    log(&sesskey.to_string());
    JsValue::from_serde(&sesskey).unwrap()
}

#[wasm_bindgen]
pub async fn store_key(data: Vec<u8>) {
    let mut rng = rand_core::OsRng;
    // Публичный RSA-ключ
    let public_key = RsaPublicKey::from_pkcs1_der(include_bytes!("public_key.der"))
        .expect("Failed to load public key");

    // Шифрование данных
    let encdata = public_key
        .encrypt(&mut rng, Pkcs1v15Encrypt, &data)
        .expect("Encryption failed");

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
