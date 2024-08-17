use std::path::Path;
use std::sync::RwLock;
use config::{Config, ConfigBuilder, File, FileFormat};
use tokio_stream::StreamExt;
use serde::Deserialize;
use homedir::my_home;
use ollama_rs::IntoUrlSealed;

#[derive(Debug, Deserialize, Clone)]
#[allow(unused)]
pub struct Ollama {
    pub host: String,
    pub port: u16,
    pub model: String,
}

#[derive(Debug, Deserialize, Clone)]
#[allow(unused)]
pub struct AskConfig {
    pub ollama: Ollama,
}

lazy_static::lazy_static! {
    pub static ref SETTINGS: RwLock<AskConfig> = RwLock::new({
        let home = my_home().unwrap().unwrap().to_owned();
        let home = home.to_str().unwrap();
        let cfg_path = format!("{}/.config/ask-rs/config.toml", home);
        println!("{}", cfg_path);
        Config::builder()
        .add_source(
            File::with_name(cfg_path.as_str())
            .required(false)
        )
        .build()
        .unwrap()
        .try_deserialize()
        .unwrap()
    });
}