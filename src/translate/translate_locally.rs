use serde_derive::{Deserialize, Serialize};
use std::{
    io::{Read, Write},
    process::{Command, Stdio},
};

#[derive(Serialize, Deserialize, Debug)]
pub struct TranslateRequest {
    pub id: i32,
    pub command: String,
    pub data: TranslateRequestData,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TranslateRequestData {
    pub src: String,
    pub trg: String,
    pub text: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TranslateResponse {
    pub id: i32,
    pub success: bool,
    pub data: Option<TranslateResponseData>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TranslateResponseData {
    pub target: TranslateResponseTargetData,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TranslateResponseTargetData {
    pub text: String,
}

use crate::utils::command_exists;

use super::TranslateEngine;

pub struct TranslateLocally {}

// https://github.com/marek-g/marek_translate
impl TranslateEngine for TranslateLocally {
    fn translate(
        &self,
        text: &str,
        lang_from: &str,
        lang_to: &str,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let mut command = Command::new("translateLocally")
            .arg("-p")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()?;

        let mut stdin = command.stdin.take().ok_or("Failed to open stdin")?;
        let mut stdout = command.stdout.take().ok_or("Failed to open stdout")?;

        let request = TranslateRequest {
            id: 1,
            command: "Translate".to_string(),
            data: TranslateRequestData {
                src: lang_from[..2].to_string(),
                trg: lang_to[..2].to_string(),
                text: text.to_string(),
            },
        };

        let request_bytes = serde_json::to_vec(&request)?;
        let length = (request_bytes.len() as u32).to_ne_bytes();

        stdin.write_all(&length)?;
        stdin.write_all(&request_bytes)?;

        // Fermer stdin pour indiquer que nous avons fini d'écrire
        drop(stdin);

        let mut response_len = [0u8; 4];
        stdout.read_exact(&mut response_len)?;
        let response_len = u32::from_ne_bytes(response_len);

        let mut response_bytes = vec![0u8; response_len as usize];
        stdout.read_exact(&mut response_bytes)?;

        let response = serde_json::from_slice::<TranslateResponse>(&response_bytes)?;

        // Attendre que le processus se termine proprement au lieu de le tuer
        let _ = command.wait()?;

        if let Some(data) = response.data {
            return Ok(data.target.text);
        }

        Err("Response does not contain data".into())
    }

    fn is_available(&self) -> bool {
        command_exists("translateLocally")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_translate_request_serialization() {
        let request = TranslateRequest {
            id: 1,
            command: "Translate".to_string(),
            data: TranslateRequestData {
                src: "en".to_string(),
                trg: "fr".to_string(),
                text: "Hello".to_string(),
            },
        };

        let serialized = serde_json::to_string(&request).unwrap();
        let expected =
            r#"{"id":1,"command":"Translate","data":{"src":"en","trg":"fr","text":"Hello"}}"#;
        assert_eq!(serialized, expected);
    }

    #[test]
    fn test_translate_response_deserialization() {
        let json_data = r#"
        {
            "id": 1,
            "success": true,
            "data": {
                "target": {
                    "text": "Bonjour"
                }
            }
        }"#;

        let response: TranslateResponse = serde_json::from_str(json_data).unwrap();
        assert_eq!(response.id, 1);
        assert!(response.success);
        assert_eq!(response.data.unwrap().target.text, "Bonjour");
    }

    #[test]
    fn test_translate_response_no_data() {
        let json_data = r#"
        {
            "id": 1,
            "success": false,
            "data": null
        }"#;

        let response: TranslateResponse = serde_json::from_str(json_data).unwrap();
        assert_eq!(response.id, 1);
        assert!(!response.success);
        assert!(response.data.is_none());
    }
}
