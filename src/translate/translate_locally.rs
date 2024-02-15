use serde_derive::{Deserialize, Serialize};
use std::{
    io::{Read, Write},
    process::{Command, Stdio},
};

#[derive(Serialize, Deserialize, Debug)]
pub struct TranslateRequest<'a> {
    pub id: i32,
    pub command: &'a str,
    pub data: TranslateRequestData<'a>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TranslateRequestData<'a> {
    pub src: &'a str,
    pub trg: &'a str,
    pub text: &'a str,
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

use super::TranslateEngine;

pub struct TranslateLocally {}

// https://github.com/marek-g/marek_translate
impl TranslateEngine for TranslateLocally {
    fn translate(&self, text: &str, lang_from: &str, lang_to: &str) -> String {
        let mut command = Command::new("translateLocally")
            .arg("-p")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()
            .unwrap();

        let mut stdin = command.stdin.take().unwrap();
        let mut stdout = command.stdout.take().unwrap();

        let request = TranslateRequest {
            id: 1,
            command: "Translate",
            data: TranslateRequestData {
                src: &lang_from[..2],
                trg: &lang_to[..2],
                text,
            },
        };

        let request = serde_json::to_vec(&request).unwrap();
        let length = (request.len() as u32).to_ne_bytes();

        let _ = stdin.write(&length);
        let _ = stdin.write(&request);

        let mut response_len = [0u8; 4];
        let _ = stdout.read(&mut response_len);
        let response_len = u32::from_ne_bytes(response_len);

        let mut res = vec![0u8; response_len as usize];
        let _ = stdout.read(&mut res);

        let response = serde_json::from_slice::<TranslateResponse>(&res).unwrap();

        if let Some(data) = response.data {
            return data.target.text;
        }

        panic!("la réponse ne contient pas de data");
    }
}
