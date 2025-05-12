use base64::{
    engine::general_purpose::STANDARD, engine::general_purpose::URL_SAFE_NO_PAD, Engine as _,
};
use gloo_utils::format::JsValueSerdeExt;
use rand_core::RngCore;
use rsa::pkcs8::DecodePublicKey;
use rsa::{Pkcs1v15Encrypt, RsaPublicKey};
use serde_json::json;

use wasm_bindgen::prelude::*;

// Публичный RSA-ключ в PEM-формате
// ~~~~~~~~~ HARDCODED! ~~~~~~~~~~~
const PUBKEY_PEM: &str = r#"
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

/// Функция генерирует случайные 16 байт (сессионный ключ для E2E чатов),
/// кодирует их в CryptoKey и шифрует публичным RSA-ключом,
/// который предоставлен ДИБ в PEM-формате, после чего возвращает
/// совмещённый результат в одном объекте
#[wasm_bindgen]
pub async fn generate_aes_key(_rid: String) -> Result<JsValue, JsValue> {
    // Случайные данные для генерации сессионного ключа
    let mut rng = rand_core::OsRng;
    let mut data = [0u8; 16];
    rng.fill_bytes(&mut data);

    let public_key = RsaPublicKey::from_public_key_pem(PUBKEY_PEM)
        .map_err(|e| JsValue::from_str(&format!("RSA-key import failed: {}", e)))?;

    // Шифрование нового сессионного ключа публичным RSA-ключом
    let encdata = public_key
        .encrypt(&mut rng, Pkcs1v15Encrypt, &data)
        .map_err(|e| JsValue::from_str(&format!("Encryption failed: {}", e)))?;

    // Сессионный ключ в формате CryptoKey Object
    let sesskey = json!({
        // Бинарные 16 байт кодируются в Base64URI
        "k": format!("{}",URL_SAFE_NO_PAD.encode(data)), // {{
        "kty": "oct",                                    // CryptoKey
        "alg": "A128CBC",                                // JS
        "ext": true,                                     // Object
        "key_ops": ["encrypt", "decrypt"],               // }}
        // Зашифрованный сессионный ключ кодируется в Base64URI
        "ck": URL_SAFE_NO_PAD.encode(encdata)            // дополнение
    });

    // Идентификатор сгенерированного ключа формируется по методике,
    // которая получена путём реверс-инжиниринга кода RocketChat
    let key_string: String = STANDARD.encode(sesskey.to_string());
    let _key_id: String = key_string.chars().take(12).collect();

    // Отправка данных на сервер используется только при работе на клиенте.
    // Базовый сценарий: модуль в составе бэкенда, где за сохранение результата
    // в БД отвечает кастомный метод Meteor'a
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    // let url = format!(
    //     "http://localhost:8000/?rid={}&sesskey={}",
    //     rid,
    //     URL_SAFE_NO_PAD.encode(encdata)
    // );

    // let client = reqwest::Client::new();
    // let response = client
    //     .get(url)
    //     .send()
    //     .await
    //     .map_err(|e| JsValue::from_str(&format!("Request error: {}", e)))?;

    // if response.status().is_success() {
    //     log("Data sent successfully");
    // } else {
    //     log(response.status().as_str());
    //     Err(JsValue::from_str(&format!(
    //         "Server error: {}",
    //         response.status()
    //     )))
    // }
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

    // Возврат составного объекта
    JsValue::from_serde(&sesskey)
        .map_err(|e| JsValue::from_str(&format!("Serialization error: {}", e)))
}

/// Служебная функция для отладки
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    pub fn log(s: &str);
}
