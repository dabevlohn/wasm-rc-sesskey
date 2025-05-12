use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine as _};
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
MIGfMA0GCSqGSIb3DQEBAQUAA4GNADCBiQKBgQCzapsex3ucgvKV5hr8EzE+FzvF
ULnQByYQ8ZwEkhutsy3/3rvGghlZaGXq6+3atQy5Zkh+FpCz+pOasulBrz6BGsuu
ED0gb+Tm4SdYj6oGaMejOaz1dM5yM9zEveWMNuVx30t6q8163s4rzDnBl2DhhvNX
E8hKDAIGWZUHUmE9CwIDAQAB
-----END PUBLIC KEY-----
"#;

/// Функция генерирует случайные 16 байт (сессионный ключ для E2E чатов),
/// кодирует их в CryptoKey и шифрует публичным RSA-ключом,
/// который предоставлен ДИБ в PEM-формате, после чего возвращает
/// совмещённый результат в одном объекте
#[wasm_bindgen]
pub async fn generate_aes_key(rid: String) -> Result<JsValue, JsValue> {
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
        "k": format!("{}", URL_SAFE_NO_PAD.encode(data)),    // {{
        "kty": "oct",                                        // CryptoKey
        "alg": "A128CBC",                                    // JS
        "ext": true,                                         // Object
        "key_ops": ["encrypt", "decrypt"],                   // }}
        // Зашифрованный сессионный ключ кодируется в Base64URI
        "ck": format!("{}", URL_SAFE_NO_PAD.encode(encdata)) // дополнение
    });

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
