use reqwest::{Client, Response};
use serde::Serialize;
use std::error::Error;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;

#[derive(Debug, Serialize)]
struct SynthesizeTextRequest {
    #[serde(rename = "ie")]
    input_encoding: String,
    #[serde(rename = "q")]
    query: String,
    #[serde(rename = "tl")]
    target_language: String,
    total: i32,
    idx: i32,
    textlen: i32,
    client: String,
}

struct GoogleTranslateClient {
    client: Client,
    base_url: String,
    language: String,
}

impl GoogleTranslateClient {
    fn new(language: String) -> Self {
        let client = Client::new();
        let base_url = String::from("https://translate.google.fr/translate_tts");

        Self {
            client,
            base_url,
            language,
        }
    }

    fn build_url(&self, text: &str) -> Result<String, Box<dyn Error>> {
        let request = SynthesizeTextRequest {
            input_encoding: "UTF-8".to_owned(),
            query: text.to_owned(),
            target_language: self.language.to_owned(),
            total: 1,
            idx: 0,
            textlen: text.len() as i32,
            client: "tw-ob".to_owned(),
        };

        let query_string = serde_urlencoded::to_string(&request)?;
        Ok(format!("{}?{}", self.base_url, query_string))
    }
}
fn main() {
    println!("Hello, world!");
}
