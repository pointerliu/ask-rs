use tokio::io::AsyncWriteExt;
use ollama_rs::{IntoUrl, Ollama};
use ollama_rs::generation::completion::GenerationResponseStream;
use ollama_rs::generation::completion::request::GenerationRequest;
use tokio_stream::StreamExt;
use crate::error::PromptError;
use crate::prompt::Prompt;

pub trait ChatEngine {
    async fn prompt(&self, input: &Prompt) -> Result<GenerationResponseStream, PromptError>;
    async fn generate(&self, prompt: Prompt) -> Result<String, Box<dyn std::error::Error>>;
    fn postprocess(&self, resp: String) -> String;
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

    async fn generate(&self, prompt: Prompt) -> Result<String, Box<dyn std::error::Error>> {
        let mut stream = self.prompt(&prompt.into()).await.unwrap();
        let mut buffer = Vec::new();
        while let Some(Ok(res)) = stream.next().await {
            for ele in res {
                buffer.write_all(ele.response.as_bytes()).await?;
            }
        }
        let buffer = String::from_utf8(buffer)?;
        let buffer = self.postprocess(buffer);
        Ok(buffer)
    }

    fn postprocess(&self, resp: String) -> String {
        let mut buffer = Vec::new();
        let mut in_cblk = false;
        for line in resp.lines() {
            if line.starts_with("```") && !in_cblk {
                in_cblk = true;
                continue
            } else if line.starts_with("```") && in_cblk {
                in_cblk = false;
                continue
            }
            if in_cblk || !self.command_only {
                buffer.push(line);
            }
        }
        let buffer = buffer.join("\n");
        buffer
    }
}