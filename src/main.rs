use std::io::stdin;
use clap::Parser;
use tokio::io::{stdout, AsyncWriteExt};
use tokio_stream::StreamExt;
use crate::config::SETTINGS;
use crate::llm::{ChatEngine, OllamaEngine};
use crate::prompt::Prompt;

mod config;
mod llm;
mod error;
mod prompt;

#[derive(Parser, Debug)]
struct Args {
    prompt: Option<String>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cfg = SETTINGS
        .read()
        .unwrap()
        .clone();
    println!("{cfg:?}");

    let args = Args::parse();

    let engine = OllamaEngine::new(
        cfg.ollama.host,
        cfg.ollama.port,
        &cfg.ollama.model,
    );

    let prompt: Prompt = if let Some(prompt) = args.prompt {
        prompt.into()
    } else {
        stdin().into()
    };

    dbg!(prompt.message.clone());

    engine.generate(stdout(), prompt).await
}