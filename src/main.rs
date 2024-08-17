use std::error::Error;
use std::io::stdin;
use clap::{arg, Parser};
use crate::config::SETTINGS;
use crate::llm::{ChatEngine, OllamaEngine};
use crate::prompt::Prompt;
use crate::web::WebSearcher;

mod config;
mod llm;
mod error;
mod prompt;
mod web;

#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long)]
    verbose: bool,
    #[arg(short, long)]
    command_only: bool,
    #[arg(short, long)]
    search: bool,
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

    let arg_prompt = args.prompt.map_or("".to_owned(), |x| x);
    let mut ws = WebSearcher::new(arg_prompt.clone());
    if args.search {
        ws.search().await;
    }

    let prompt = Prompt::new()
        .add_source(stdin())
        .add_source(ws)
        .add_source(arg_prompt);

    if args.verbose {
        println!("Prompt = {}", prompt.message);
    }
    let resp = engine.generate(prompt).await?;
    println!("{}", resp);
    Ok(())
}