use tokio::io::{Stdout, AsyncWriteExt};
use ollama_rs::{IntoUrl, Ollama};
use ollama_rs::generation::completion::GenerationResponseStream;
use ollama_rs::generation::completion::request::GenerationRequest;
use tokio_stream::StreamExt;
use crate::error::PromptError;
use crate::prompt::Prompt;

pub trait ChatEngine {
    async fn prompt(&self, input: &Prompt) -> Result<GenerationResponseStream, PromptError>;
    async fn generate(&self, stdout: Stdout, prompt: Prompt) -> Result<(), Box<dyn std::error::Error>>;
}

pub struct OllamaEngine {
    ollama: Ollama,
    model: String,
}

impl OllamaEngine {
    pub fn new(host: impl IntoUrl, port: u16, model: &str) -> Self {
        Self {
            ollama: Ollama::new(host, port),
            model: model.to_string(),
        }
    }
}

impl ChatEngine for OllamaEngine {
    async fn prompt(&self, input: &Prompt) -> Result<GenerationResponseStream, PromptError> {
        let request = GenerationRequest::new(self.model.clone(), input.message.clone());
        let res = self.ollama.generate_stream(request).await;

        if let Err(err) = res {
            Err(PromptError::new("Ollama server maybe not running."))
        } else {
            Ok(res.unwrap())
        }
    }

    async fn generate(&self, stdout: Stdout, prompt: Prompt) -> Result<(), Box<dyn std::error::Error>> {
        let mut stdout = stdout;
        let mut stream = self.prompt(&prompt.into()).await.unwrap();
        while let Some(Ok(res)) = stream.next().await {
            for ele in res {
                stdout.write_all(ele.response.as_bytes()).await?;
                stdout.flush().await?;
            }
        }
        Ok(())
    }
}