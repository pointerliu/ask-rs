use std::error::Error;
use std::io::stdin;
use clap::Parser;
use tokio::io::stdout;
use crate::config::SETTINGS;
use crate::llm::{ChatEngine, OllamaEngine};
use crate::prompt::Prompt;

mod config;
mod llm;
mod error;
mod prompt;

#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long)]
    verbose: bool,
    prompt: Option<String>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    let cfg = SETTINGS.read()?.clone();
    if args.verbose {
        println!("{:#?}", cfg);
    }

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

    engine.generate(stdout(), prompt).await
}