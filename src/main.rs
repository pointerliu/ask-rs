use std::error::Error;
use std::io::stdin;
use clap::{arg, Parser};
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
    #[arg(short, long)]
    command_only: bool,
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
        args.command_only
    );
    let mut prompt = Prompt::new()
        .add_source(stdin())
        .add_source(args.prompt.map_or("".to_owned(), |x| x));

    if args.verbose {
        println!("Prompt = {}", prompt.message);
    }
    let resp = engine.generate(stdout(), prompt).await?;
    println!("{}", resp);
    Ok(())
}