# RocketChat Session Key Generator (WASM)

## Быстрый старт

На стенде должны быть установлены NodeJS >= 18 и Rust >= 1.76

```bash
rustup update                                 # обновление Rust
rustup target add wasm32-unknown-unknown      # установка целевой платформы
cargo install wasm-pack                       # установка сборщика
cargo install just                            # установка оболочки выполнения
git clone https://<this-repo-url>             # клонирование репозитория
cd <this-repo>
just g                                        # генерация ключей
```

Полученный публичный ключ (public_key.pem) необходимо вставить
в код библиотеки src/lib.rs при помощи обычного текстового редактора
(константа PUBLIC_KEY)

```bash
just b                                        # сборка WASM-модуля
```

## Тестирование

```bash
cd back-nodejs                                
npm install                                   # установка зависимостей
node server.js                                # запуск сервера
```

Откройте браузер и перейдите на http://localhost:3000/about.html
В терминале, где запущен сервер, должна появиться отладочная информация: JSON с
зашифрованным и незашифрованным сессионным ключом.

## Диаграмма последовательности

![](./keygen_seq.png)

```plantuml
@startuml
title RocketChat SessionKey Generator v.2 Sequence

DIB -> DIB: Generate RSA\nkey pair
DIB -> DIB: Store RSA\nprivate key
DIB --> RCTeam: Pass RSA public key
RCTeam --> Prod: Deploy WASM Crypto\nModule with public RSA key
Prod --> DIB: Export room keys encrypted with public RSA key
Prod --> DIB: Export messages encrypted with room keys
DIB -> DIB: Decrypt room key\nwith RSA private key
DIB -> DIB: Decrypt room messages\nwith decrypted room key

@enduml

```

## Состав компонентов

![](./keygen_objects.png)

```plantuml
@startuml

title RocketChat Session Key Generator v.2 Objects
package	"Rocket.Chat" {

	frame RocketChat-Client {
		object "Frontend Functions for E2E rooms" as FE {
			+generateAESKey()
			+importAESKey()
		}
	}

	frame RocketChat-Server {

		object "WASM Crypto Module" as CM {
			+returnRoomKey()
			+saveRoomKey()
		}

		note bottom of CM
		1. generates random 16 bytes
		2. return 16 bytes as CryptoKey Object
		3. encrypt 16 bytes for DIB public key
		4. put encrypted 16 bytes to store via Meteor's method
		endnote


		object "API Method" as SK {
			+saveRoomKey()
		}

		object ORM {
			+saveRoomKey()
		}

		SK .. ORM
	}

	FE --> CM
	CM -> SK: <room_id, encrypted_key>
}

@enduml
```
