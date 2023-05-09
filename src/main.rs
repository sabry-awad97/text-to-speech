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

#[derive(Debug)]
enum TtsError {
    RequestError(reqwest::Error),
    SerdeError(serde_urlencoded::ser::Error),
    IoError(std::io::Error),
}

impl Error for TtsError {}

impl std::fmt::Display for TtsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TtsError::RequestError(e) => write!(f, "Request error: {}", e),
            TtsError::SerdeError(e) => write!(f, "Serde error: {}", e),
            TtsError::IoError(e) => write!(f, "IO error: {}", e),
        }
    }
}

impl From<reqwest::Error> for TtsError {
    fn from(error: reqwest::Error) -> Self {
        TtsError::RequestError(error)
    }
}

impl From<serde_urlencoded::ser::Error> for TtsError {
    fn from(error: serde_urlencoded::ser::Error) -> Self {
        TtsError::SerdeError(error)
    }
}

impl From<std::io::Error> for TtsError {
    fn from(error: std::io::Error) -> Self {
        TtsError::IoError(error)
    }
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

    fn build_url(&self, text: &str) -> Result<String, TtsError> {
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

    async fn synthesize_text(&self, text: &str) -> Result<Response, TtsError> {
        let url = self.build_url(text)?;
        let response = self.client.get(&url).send().await?;
        Ok(response)
    }
}

struct AudioFile {
    file: File,
}

impl AudioFile {
    async fn new(path: &str) -> Result<Self, TtsError> {
        let file = File::create(path).await?;
        Ok(Self { file })
    }

    async fn write_chunk(&mut self, chunk: &[u8]) -> Result<(), TtsError> {
        self.file.write_all(chunk).await?;
        Ok(())
    }
}

struct TextToSpeech {
    client: GoogleTranslateClient,
    audio_file: AudioFile,
}

impl TextToSpeech {
    async fn new(language: &str, path: &str) -> Result<Self, TtsError> {
        let client = GoogleTranslateClient::new(language.to_owned());
        let audio_file = AudioFile::new(path).await?;

        Ok(Self { client, audio_file })
    }

    async fn synthesize_text(&mut self, text: &str) -> Result<(), TtsError> {
        let mut response = self.client.synthesize_text(text).await?;

        if response.status().is_success() {
            while let Some(chunk) = response.chunk().await? {
                self.audio_file.write_chunk(&chunk).await?;
            }
        }

        Ok(())
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let text = "Hello, world!";
    let language = "en-US";
    let path = "output.mp3";

    let mut tts = TextToSpeech::new(language, path).await?;
    tts.synthesize_text(text).await?;
    Ok(())
}
