use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine as _};
use gloo_utils::format::JsValueSerdeExt;
use rand_core::RngCore;
use reqwest;
use rsa::pkcs8::DecodePublicKey;
use rsa::{Pkcs1v15Encrypt, RsaPublicKey};
use serde_json::json;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub async fn generate_aes_key(rid: String) -> Result<JsValue, JsValue> {
    // Случайные данные для шифрования
    let mut rng = rand_core::OsRng;
    let mut data = [0u8; 16];
    rng.fill_bytes(&mut data);

    let sesskey = json!({
        "kty": "oct",
        "alg": "A128CBC",
        "k": format!("{}",URL_SAFE_NO_PAD.encode(data)),
        "ext": true,
        "key_ops": ["encrypt", "decrypt"]
    });

    //log(&sesskey.to_string());

    let pubkey_pem = r#"
-----BEGIN PUBLIC KEY-----
MIIBIjANBgkqhkiG9w0BAQEFAAOCAQ8AMIIBCgKCAQEA47xyXpCj2iH5gZ9PWJpN
oznu0A8dNGN2n4O8Vcc/kwOn/iDZwQrRGnjlKIPUZvW9gY0tKK4bAYqpId9YHVOy
Qh4/VVZ0eNti0y++buZqwcLq3HlIHv272B46iv48fc1Rngonfbj05yEr7zTfaRN+
aM0KeVzO0hnddkgC3N9qKyRFgxRUu5HSUbBB+1/CC0XMCrpArtvwcOttVEFmeHH7
IaKeSQ8/Bz/COBqCUGHPX3NoSmcHDZ1V4PMRXqaA+PuHGsvtRgZKaWeUGumcR7Ri
IgALq85Wkv3fnaoeyT1I+8ZXaWi5tTv3gxV2hhnJv375z/dB0WwRlm30BNo+h3Oh
VQIDAQAB
-----END PUBLIC KEY-----
"#;

    // Публичный RSA-ключ
    let public_key = RsaPublicKey::from_public_key_pem(pubkey_pem)
        .map_err(|e| JsValue::from_str(&format!("RSA-key import failed: {}", e)))?;

    // Шифрование данных
    let encdata = public_key
        .encrypt(&mut rng, Pkcs1v15Encrypt, &data)
        .map_err(|e| JsValue::from_str(&format!("Encryption failed: {}", e)))?;

    // log(&format!("{:?}", URL_SAFE_NO_PAD.encode(encdata.clone())));

    // Отправка данных на сервер
    let url = format!(
        "http://localhost:8000/?rid={}&sesskey={}",
        rid,
        URL_SAFE_NO_PAD.encode(encdata)
    );

    let client = reqwest::Client::new();
    let response = client
        .get(url)
        .send()
        .await
        .map_err(|e| JsValue::from_str(&format!("Request error: {}", e)))?;

    if response.status().is_success() {
        log("Data sent successfully");
        JsValue::from_serde(&sesskey)
            .map_err(|e| JsValue::from_str(&format!("Serialization error: {}", e)))
    } else {
        log(response.status().as_str());
        Err(JsValue::from_str(&format!(
            "Server error: {}",
            response.status()
        )))
    }
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    pub fn log(s: &str);
}
