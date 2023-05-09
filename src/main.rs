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

fn main() {
    println!("Hello, world!");
}
