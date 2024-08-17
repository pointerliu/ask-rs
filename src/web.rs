use reqwest::{header, Client};
use serde::{Deserialize, Serialize};

pub struct WebSearcher {
    pub query: String,
    pub response: String,
    #[allow(dead_code)]
    search_engine: String,
    format: String,
}

#[derive(Serialize, Deserialize, Debug, Default)]
struct Usage {
    tokens: u32,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct WebData {
    text: String,
    title: String,
    description: String,
    usage: Usage,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct WebResponse {
    code: u32,
    status: u32,
    data: Vec<WebData>,
}

impl WebSearcher {
    pub fn new(query: String) -> Self {
        WebSearcher {
            query,
            response: "".to_string(),
            search_engine: "jina.ai".to_string(),
            format: "text".to_string(),
        }
    }

    pub async fn search(&mut self) {
        let client = Client::new();

        let response = client.get(format!("https://s.jina.ai/{}", self.query))
            .header(header::ACCEPT, "application/json")
            // .header(header::AUTHORIZATION, "Bearer jina_58dea1b1a09b4fd5ade8e1619f21552deELpfFEOzo3jgqFY3Y-Otmnzdhsd")
            .header("X-Return-Format", self.format.clone())
            .send()
            .await
            .unwrap();
        let res = response
            .text()
            .await
            .map_or("".to_string(), |x| x);
        let json_ds: WebResponse = serde_json::from_str(&res).map_or(WebResponse::default(), |x| x);
        dbg!(&json_ds.data[0]);
        self.response = res
    }
}