# ask-rs
Ask LLM directly from your terminal, written in Rust.

# Prerequisites

You need to deploy [Ollama](https://github.com/ollama/ollama) first.
```bash
# install ollama
curl -fsSL https://ollama.com/install.sh | sh
# start ollama server, 
# If you install ollama using above command, a ollama server has been running at http://127.0.0.1:11434.
# So do not start another server again.
ollama serve
# pull a model from ollama, e.g. llama3.1:8b
ollama pull llama3.1:8b
```
You also need to download rust first.

# Compile & Install

```bash
# enter the project dir and execute
cargo install --path .
```

# How to use

## Ask a question
```bash
ask-rs "hello, who are you ?"
```

## Piping
```bash
ls . | ask-rs "how many files in current dir ?"
```

## Command-only
```bash
ask-rs "write me a simple python program" -c
```

## Web searcher
```bash
ask-rs -s "why the sky is blue ?"
```

# Config

config.toml is located at `~/.config/ask-rs/config.toml`
```toml
[ollama]
host = "http://127.0.0.1"
port = 11434
model = "llama3.1:8b"
```

# License
borrow examples from [ollama-rs](https://github.com/pepperoni21/ollama-rs)

Inspired by [shell-ask](https://github.com/egoist/shell-ask)