use tokio::io::{Stdout, AsyncWriteExt};
use ollama_rs::{IntoUrl, Ollama};
use ollama_rs::generation::completion::GenerationResponseStream;
use ollama_rs::generation::completion::request::GenerationRequest;
use tokio_stream::StreamExt;
use crate::error::PromptError;
use crate::prompt::Prompt;

pub trait ChatEngine {
    async fn prompt(&self, input: &Prompt) -> Result<GenerationResponseStream, PromptError>;
    async fn generate(&self, stdout: Stdout, prompt: Prompt) -> Result<String, Box<dyn std::error::Error>>;
    fn postprocess<'a>(&self, resp: &'a mut str) -> &'a str;
}

pub struct OllamaEngine {
    ollama: Ollama,
    model: String,
    command_only: bool,
}

impl OllamaEngine {
    pub fn new(host: impl IntoUrl, port: u16, model: &str, command_only: bool) -> Self {
        Self {
            ollama: Ollama::new(host, port),
            model: model.to_string(),
            command_only
        }
    }
}

impl ChatEngine for OllamaEngine {
    async fn prompt(&self, input: &Prompt) -> Result<GenerationResponseStream, PromptError> {
        let request = GenerationRequest::new(self.model.clone(), input.message.clone());
        let res = self.ollama.generate_stream(request).await;

        if let Err(_err) = res {
            Err(PromptError::new("Ollama server maybe not running."))
        } else {
            Ok(res.unwrap())
        }
    }

    async fn generate(&self, stdout: Stdout, prompt: Prompt) -> Result<String, Box<dyn std::error::Error>> {
        let mut stream = self.prompt(&prompt.into()).await.unwrap();
        let mut buffer = Vec::new();
        while let Some(Ok(res)) = stream.next().await {
            for ele in res {
                buffer.write_all(ele.response.as_bytes()).await?;
            }
        }
        let mut buffer = String::from_utf8(buffer)?;
        let buffer = self.postprocess(&mut buffer);
        Ok(buffer.to_string())
    }

    fn postprocess<'a>(&self, resp: &'a mut str) -> &'a str {
        resp
    }
}