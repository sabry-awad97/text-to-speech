# Text-To-Speech Rust Client for Google Translate TTS

This is a Rust client for Google Translate's TTS (Text-To-Speech) service. It allows you to synthesize text to speech using Google Translate and write the output to an audio file.

## Getting Started

### Prerequisites

- Rust (1.41 or later)
- Cargo (1.41 or later)
- `reqwest` and `tokio` crates.

### Installation

1. Clone the repository:

   ```bash
   git clone https://github.com/sabry-awad97/text-to-speech.git
   cd text-to-speech
   ```

2. Build the project:

   ```bash
   cargo build --release
   ```

### Usage

The basic usage of the Text-To-Speech Rust client is as follows:

```rust
use std::error::Error;
use text_to_speech::TextToSpeech;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let text = "Hello, world!";
    let language = "en-US";
    let path = "output.mp3";

    let mut tts = TextToSpeech::new(language, path).await?;
    tts.synthesize_text(text).await?;
    Ok(())
}
```

- `text` is the input text that you want to synthesize.
- `language` is the language in which you want to synthesize the text.
- `path` is the output audio file path.

## Contributing

Bug reports and pull requests are welcome on GitHub at <https://github.com/sabry-awad97/text-to-speech>. This project is intended to be a safe, welcoming space for collaboration, and contributors are expected to adhere to the [Contributor Covenant](https://www.contributor-covenant.org/) code of conduct.

## License

The Text-To-Speech Rust client is available as open source under the terms of the [MIT License](https://opensource.org/licenses/MIT).
