//! Authentication module for the Bybit SDK.
//!
//! Provides HMAC-SHA256 and RSA PKCS1v1.5 signing for both HTTP and WebSocket
//! authentication, matching the Python `generate_signature()` and `_auth()` logic.

use base64::Engine;
use hmac::{Hmac, Mac};
use reqwest::header::{HeaderMap, HeaderValue};
use rsa::pkcs8::DecodePrivateKey;
use rsa::Pkcs1v15Sign;
use sha2::Sha256;

use crate::config::{AuthMethod, ClientConfig};
use crate::error::{BybitError, Result};
use crate::helpers::generate_timestamp;

/// Compute HMAC-SHA256 and return the hex digest.
///
/// Equivalent to Python:
/// ```python
/// hmac.new(bytes(secret, "utf-8"), param_str.encode("utf-8"), hashlib.sha256).hexdigest()
/// ```
pub fn sign_hmac(secret: &str, payload: &str) -> String {
    let mut mac =
        Hmac::<Sha256>::new_from_slice(secret.as_bytes()).expect("HMAC accepts any key length");
    mac.update(payload.as_bytes());
    hex::encode(mac.finalize().into_bytes())
}

/// Compute RSA PKCS1v1.5 SHA-256 signature and return base64-encoded string.
///
/// Equivalent to Python:
/// ```python
/// hash = SHA256.new(param_str.encode("utf-8"))
/// base64.b64encode(PKCS1_v1_5.new(RSA.importKey(secret)).sign(hash)).decode()
/// ```
pub fn sign_rsa(private_key_pem: &str, payload: &str) -> Result<String> {
    let private_key = rsa::RsaPrivateKey::from_pkcs8_pem(private_key_pem)
        .map_err(|e| BybitError::Rsa(e.into()))?;

    let digest = {
        use sha2::Digest;
        let mut hasher = Sha256::new();
        hasher.update(payload.as_bytes());
        hasher.finalize()
    };

    let signature = private_key
        .sign(Pkcs1v15Sign::new::<Sha256>(), &digest)?;

    Ok(base64::engine::general_purpose::STANDARD.encode(signature))
}

/// Generate a signature using the configured authentication method.
///
/// Dispatches to `sign_hmac` or `sign_rsa` based on the `AuthMethod`.
pub fn generate_signature(auth_method: &AuthMethod, secret: &str, payload: &str) -> Result<String> {
    match auth_method {
        AuthMethod::Hmac => Ok(sign_hmac(secret, payload)),
        AuthMethod::Rsa => sign_rsa(secret, payload),
    }
}

/// Build authenticated HTTP headers for a Bybit API request.
///
/// Produces headers matching the Python `_prepare_headers()`:
/// ```text
/// X-BAPI-API-KEY: {api_key}
/// X-BAPI-SIGN: {signature}
/// X-BAPI-SIGN-TYPE: 2
/// X-BAPI-TIMESTAMP: {timestamp_ms}
/// X-BAPI-RECV-WINDOW: {recv_window_ms}
/// ```
///
/// The param string is: `{timestamp}{api_key}{recv_window}{payload}`
pub fn build_auth_headers(config: &ClientConfig, payload: &str) -> Result<HeaderMap> {
    let api_key = config.api_key.as_deref().ok_or(BybitError::Unauthorized)?;
    let api_secret = config.api_secret.as_deref().ok_or(BybitError::Unauthorized)?;

    let timestamp = generate_timestamp();
    let recv_window = config.recv_window;

    let param_str = format!("{}{}{}{}", timestamp, api_key, recv_window, payload);
    let signature = generate_signature(&config.auth_method, api_secret, &param_str)?;

    let mut headers = HeaderMap::new();
    headers.insert("Content-Type", HeaderValue::from_static("application/json"));
    headers.insert("X-BAPI-API-KEY", HeaderValue::from_str(api_key).unwrap());
    headers.insert("X-BAPI-SIGN", HeaderValue::from_str(&signature).unwrap());
    headers.insert("X-BAPI-SIGN-TYPE", HeaderValue::from_static("2"));
    headers.insert(
        "X-BAPI-TIMESTAMP",
        HeaderValue::from_str(&timestamp.to_string()).unwrap(),
    );
    headers.insert(
        "X-BAPI-RECV-WINDOW",
        HeaderValue::from_str(&recv_window.to_string()).unwrap(),
    );

    Ok(headers)
}

/// Build the WebSocket authentication JSON message.
///
/// Matches the Python `_auth()` in `_websocket_stream.py`:
/// ```python
/// expires = generate_timestamp() + (private_auth_expire * 1000)
/// param_str = f"GET/realtime{expires}"
/// signature = generate_signature(rsa_auth, api_secret, param_str)
/// {"op": "auth", "args": [api_key, expires, signature]}
/// ```
pub fn build_ws_auth_message(
    config: &ClientConfig,
    private_auth_expire_secs: u64,
) -> Result<String> {
    let api_key = config.api_key.as_deref().ok_or(BybitError::Unauthorized)?;
    let api_secret = config.api_secret.as_deref().ok_or(BybitError::Unauthorized)?;

    let expires = generate_timestamp() + (private_auth_expire_secs * 1000);
    let param_str = format!("GET/realtime{}", expires);
    let signature = generate_signature(&config.auth_method, api_secret, &param_str)?;

    let msg = serde_json::json!({
        "op": "auth",
        "args": [api_key, expires, signature]
    });

    Ok(msg.to_string())
}

/// Internal hex encoding (avoids pulling in the `hex` crate).
mod hex {
    pub fn encode(bytes: impl AsRef<[u8]>) -> String {
        bytes
            .as_ref()
            .iter()
            .fold(String::new(), |mut acc, b| {
                use std::fmt::Write;
                write!(acc, "{:02x}", b).unwrap();
                acc
            })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Known-good test vectors verified against Python's hmac + hashlib.
    // Python: hmac.new(b"my_secret", b"1234567890api_key5000symbol=BTCUSDT", hashlib.sha256).hexdigest()
    #[test]
    fn hmac_signature_matches_python() {
        let secret = "my_secret";
        let payload = "1234567890api_key5000symbol=BTCUSDT";
        let sig = sign_hmac(secret, payload);
        assert_eq!(
            sig,
            "f21fe087576c368e81fca370de448fd60682c3200fa1a6f184053b98474a9984"
        );
    }

    #[test]
    fn hmac_empty_payload() {
        // Python: hmac.new(b"secret", b"", hashlib.sha256).hexdigest()
        let sig = sign_hmac("secret", "");
        assert_eq!(
            sig,
            "f9e66e179b6747ae54108f82f8ade8b3c25d76fd30afde6c395822c530196169"
        );
    }

    #[test]
    fn generate_signature_dispatches_hmac() {
        let sig = generate_signature(&AuthMethod::Hmac, "secret", "payload").unwrap();
        assert_eq!(sig, sign_hmac("secret", "payload"));
    }

    #[test]
    fn build_auth_headers_requires_keys() {
        let config = ClientConfig::default(); // no keys
        let result = build_auth_headers(&config, "payload");
        assert!(matches!(result, Err(BybitError::Unauthorized)));
    }

    #[test]
    fn build_auth_headers_produces_all_required_headers() {
        let config = ClientConfig::testnet("my_api_key", "my_api_secret");
        let headers = build_auth_headers(&config, "symbol=BTCUSDT").unwrap();

        assert_eq!(
            headers.get("X-BAPI-API-KEY").unwrap().to_str().unwrap(),
            "my_api_key"
        );
        assert!(headers.get("X-BAPI-SIGN").is_some());
        assert_eq!(
            headers.get("X-BAPI-SIGN-TYPE").unwrap().to_str().unwrap(),
            "2"
        );
        assert!(headers.get("X-BAPI-TIMESTAMP").is_some());
        assert_eq!(
            headers
                .get("X-BAPI-RECV-WINDOW")
                .unwrap()
                .to_str()
                .unwrap(),
            "5000"
        );
        assert_eq!(
            headers.get("Content-Type").unwrap().to_str().unwrap(),
            "application/json"
        );
    }

    #[test]
    fn build_ws_auth_message_requires_keys() {
        let config = ClientConfig::default();
        let result = build_ws_auth_message(&config, 1);
        assert!(matches!(result, Err(BybitError::Unauthorized)));
    }

    #[test]
    fn build_ws_auth_message_structure() {
        let config = ClientConfig::testnet("ws_key", "ws_secret");
        let msg = build_ws_auth_message(&config, 1).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&msg).unwrap();

        assert_eq!(parsed["op"], "auth");
        let args = parsed["args"].as_array().unwrap();
        assert_eq!(args.len(), 3);
        assert_eq!(args[0].as_str().unwrap(), "ws_key");
        // args[1] is the expires timestamp (integer)
        assert!(args[1].as_u64().is_some());
        // args[2] is the HMAC signature (hex string)
        assert!(args[2].as_str().is_some());
    }

    #[test]
    fn build_ws_auth_message_expire_offset() {
        let config = ClientConfig::testnet("k", "s");
        let before = generate_timestamp();
        let msg = build_ws_auth_message(&config, 5).unwrap();
        let after = generate_timestamp();

        let parsed: serde_json::Value = serde_json::from_str(&msg).unwrap();
        let expires = parsed["args"][1].as_u64().unwrap();

        // expires should be ~5000ms in the future
        assert!(expires >= before + 5000);
        assert!(expires <= after + 5000);
    }

    // 2048-bit RSA test key in PKCS8 PEM format, generated for testing only.
    const TEST_RSA_PRIVATE_KEY: &str = "\
-----BEGIN PRIVATE KEY-----\n\
MIIEvgIBADANBgkqhkiG9w0BAQEFAASCBKgwggSkAgEAAoIBAQDfjQgPb7vsp6um\n\
ZT6P1u6+PikvM50VzAgWXGj6BuKf8XOUOL5da6mnADbM00WUBkL5RVmIRbIrE4BV\n\
0g1/FILQ7fglF4qv7T2TQYmYqp8nTFXGQGGFusS3emUKhPi0CFpRQ4THY2seqmJO\n\
B41bqzmJ4pU/56fDt+8ZfClI5XvzqKbTSW5YppxZexEOVLE/xnKkV49NyMSzAJFH\n\
xoiiNcR0DeGonXUl+4OKvfM7SjZF8/9jnjsiy9umS8F+Cu0BQl3v/ZCx7E0r8RoI\n\
D8T/cvgZRopgF01mk89V4YCevj6uxaLr6xwxQR7KERZcsU88buxjv3lXKUlp4I9f\n\
BWMfwh1ZAgMBAAECggEAacNxkswMHtTc8uZniOnpHfyOw5Qpt4xGLz4dNF1Zjs5P\n\
HksSVWSAaVBCnPk/q96ihQh4aW5/8r1RNEkitYLguTQ8+F+A7+bQAQE6tg3TKmaF\n\
FjyTZw+LzjqL30Q13gcto9DL2wGARovZ4ozUJhCGMmQOtKNGzeQRrP7/EUvxLr8z\n\
UA7dIXg1xtaCetX54A8P8rlo2cdnAhVEtHW5azP66k9oFGxaWjeWbpopMx60QZF5\n\
IygM5MuolszZ7t3t88dvSyhqWyhnfXYnPsGYeLkgkvDycFaCfbKR/XhzwRV6ioy+\n\
XGHk8egyvPBskF7N0ybafhiLRVzhKXRib5R+XnicGQKBgQDljD5NUADccriYuqMy\n\
3L1UPO5UbjUKlCvonJLE8tSfUTQD7n1VRpBE8XEiOpa3rVdBkRTMYzZEaGmLwTfi\n\
QwRdG6MztLgFEFysXs1AjqEXqlm7hVucg9Hpo1lj9c8O9J6f5hCFnAUr+Og3PpRw\n\
fSGB62NUAyDPwW9Q55N+mrXPTwKBgQD5T+BeA2bbquGitFs1L0r456On/1zD92VB\n\
kHKdFjGnI9sNROnB7QrnGGDyg29RWTRwosFP3LmgiOPWsQTOIGZvPyDFJ/ujD+Q+\n\
/PT6nwKDWW3cmV39Xzkfirchc9XGsCsAG3eZRrSrUrofmBUfud0yatw96rIkKvKj\n\
5+PqNW5e1wKBgE3NdAx2UmRlyeWSVOBXayCTKmCZngbA5AUKlMCGYcTCHDQ88GQn\n\
75w+jY8BdrY43XtOtlL0SZ90GFoTGmLpPiBkzrMmzLyCqDaQR1dbQISp65IgiEWL\n\
dGmRQSodVZoHFYCy1OTEVghqbCLVZjCeaO4FH3yJNuTT9mvgoeBmhbsHAoGBAK+w\n\
woR+ETYsLeXjA4NvWH4ffw0cv9q5tPmHOb3JriQC+JNZRQIMcep5G5jLVt8U5gea\n\
KnCFM6sVzLm29ZPs8ULkUcueaY533Hkm+dzDwZmeo/UZf0rWmALKVlOUtcnWy/a/\n\
ZID3espg+hjZq8k3ydGFzL81bWRff65R9CNQk/7xAoGBAOVWU/C0oOpMu5QOzXZ7\n\
h7Df6Vm0RwMAwrDdiQZFvmUDDIRPdG/U6DwFuXnFdgL01ZSxZYrQC/2LD21Z0ZSf\n\
Pr0P4IMvZOfaMaKG9DLYtLqmj2LC4wbuBozW94E8Nt5pw1hSJ6v/54eXXTPcKB6t\n\
IWcQcrLZhs0giB2/3QzDqp67\n\
-----END PRIVATE KEY-----";

    #[test]
    fn rsa_signature_matches_python() {
        // Cross-verified with Python PyCryptodome:
        // PKCS1_v1_5.new(RSA.importKey(key_pem)).sign(SHA256.new(b"test_payload"))
        let sig = sign_rsa(TEST_RSA_PRIVATE_KEY, "test_payload").unwrap();
        assert_eq!(
            sig,
            "C3Nnfnw2qYsRU0NcG9YFrgdQO0BRvdS05fFEw9nhvmFWeZJFOnos8Cz0FvZHx+Mw2cRaa/ANU9AeWh8mIA+czK9fiT1gwbywDn+ubzGQT3tOoM70y7JmfAqGiL9UZ/1UQwgX2lW1zStA/W+n0RBg7+kwWBNu9X0cbviDYIdhocF8pbnO4Uvriu91Ix43U8dP8I2dQJ2WQk2WgenWKJz4QU1OLOprhMI2iG7SOcF9mk8lVb8T/pM/42YdZAMH1LubQWR3WBSrW7SnQ2wOxvm+9iKH2/r6Zb8EYd4JgYvIaAGPcqd9EoZszJsQuJFy250RqWBz1mKdY8TdtYz/kM6fug=="
        );
    }

    #[test]
    fn rsa_signature_is_base64() {
        let sig = sign_rsa(TEST_RSA_PRIVATE_KEY, "test_payload").unwrap();
        assert!(base64::engine::general_purpose::STANDARD.decode(&sig).is_ok());
        assert!(!sig.is_empty());
    }

    #[test]
    fn rsa_signature_deterministic() {
        let sig1 = sign_rsa(TEST_RSA_PRIVATE_KEY, "same_payload").unwrap();
        let sig2 = sign_rsa(TEST_RSA_PRIVATE_KEY, "same_payload").unwrap();
        assert_eq!(sig1, sig2);
    }

    #[test]
    fn rsa_different_payloads_different_sigs() {
        let sig1 = sign_rsa(TEST_RSA_PRIVATE_KEY, "payload_a").unwrap();
        let sig2 = sign_rsa(TEST_RSA_PRIVATE_KEY, "payload_b").unwrap();
        assert_ne!(sig1, sig2);
    }

    #[test]
    fn rsa_invalid_key_returns_error() {
        let result = sign_rsa("not-a-valid-pem", "payload");
        assert!(result.is_err());
    }

    #[test]
    fn generate_signature_dispatches_rsa() {
        let sig = generate_signature(&AuthMethod::Rsa, TEST_RSA_PRIVATE_KEY, "payload").unwrap();
        let expected = sign_rsa(TEST_RSA_PRIVATE_KEY, "payload").unwrap();
        assert_eq!(sig, expected);
    }

    #[test]
    fn build_auth_headers_signature_is_valid_hex() {
        let config = ClientConfig::testnet("key123", "secret456");
        let headers = build_auth_headers(&config, "category=linear&symbol=BTCUSDT").unwrap();

        let sig = headers.get("X-BAPI-SIGN").unwrap().to_str().unwrap();
        // HMAC-SHA256 hex signature is always 64 hex chars
        assert_eq!(sig.len(), 64);
        assert!(sig.chars().all(|c| c.is_ascii_hexdigit()));
    }

    #[test]
    fn build_auth_headers_custom_recv_window() {
        let config = ClientConfig::testnet("k", "s").with_recv_window(10000);
        let headers = build_auth_headers(&config, "").unwrap();
        assert_eq!(
            headers.get("X-BAPI-RECV-WINDOW").unwrap().to_str().unwrap(),
            "10000"
        );
    }

    #[test]
    fn build_auth_headers_timestamp_is_recent() {
        let before = generate_timestamp();
        let config = ClientConfig::testnet("k", "s");
        let headers = build_auth_headers(&config, "").unwrap();
        let after = generate_timestamp();

        let ts: u64 = headers
            .get("X-BAPI-TIMESTAMP")
            .unwrap()
            .to_str()
            .unwrap()
            .parse()
            .unwrap();
        assert!(ts >= before);
        assert!(ts <= after);
    }

    #[test]
    fn ws_auth_param_string_format() {
        // The WS auth param string is "GET/realtime{expires}" — verify structure
        let config = ClientConfig::testnet("k", "s");
        let before = generate_timestamp();
        let msg = build_ws_auth_message(&config, 2).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&msg).unwrap();

        let expires = parsed["args"][1].as_u64().unwrap();
        // 2 seconds = 2000ms offset from current time
        assert!(expires >= before + 2000);
        assert!(expires <= before + 2100); // small tolerance

        // Signature should match "GET/realtime{expires}" signed with secret
        let param_str = format!("GET/realtime{}", expires);
        let expected_sig = sign_hmac("s", &param_str);
        assert_eq!(parsed["args"][2].as_str().unwrap(), expected_sig);
    }

    #[test]
    fn ws_auth_with_rsa() {
        let config = ClientConfig {
            auth_method: AuthMethod::Rsa,
            api_key: Some("rsa_key".into()),
            api_secret: Some(TEST_RSA_PRIVATE_KEY.into()),
            ..ClientConfig::testnet("", "")
        };
        let msg = build_ws_auth_message(&config, 1).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&msg).unwrap();
        assert_eq!(parsed["op"], "auth");
        let sig_str = parsed["args"][2].as_str().unwrap();
        assert!(base64::engine::general_purpose::STANDARD.decode(sig_str).is_ok());
    }
}
