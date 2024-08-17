use std::fmt::Debug;
use std::sync::RwLock;
use config::{Config, File};
use serde::Deserialize;
use homedir::my_home;

#[derive(Debug, Deserialize, Clone)]
#[allow(unused)]
pub struct Ollama {
    pub host: String,
    pub port: u16,
    pub model: String,
}

impl Default for Ollama {
    fn default() -> Self {
        Self {
            host: "http://127.0.0.1".to_owned(),
            port: 11434,
            model: "qwen2:0.5b".to_owned(),
        }
    }
}

#[derive(Debug, Deserialize, Default, Clone)]
#[allow(unused)]
pub struct AskConfig {
    pub ollama: Ollama,
}

lazy_static::lazy_static! {
    pub static ref SETTINGS: RwLock<AskConfig> = RwLock::new({
        let cfg = {
            let home = my_home().unwrap().unwrap().to_owned();
            let home = home.to_str().unwrap();
            let cfg_path = format!("{}/.config/ask-rs/config.toml", home);
            let builder = Config::builder()
            .add_source(
                File::with_name(cfg_path.as_str())
                .required(false)
            ).build().expect(&format!("Error when build config file at {}", cfg_path));
            builder.try_deserialize()
        };
        if let Ok(cfg) = cfg {
            cfg
        } else {
            AskConfig::default()
        }
    });
}