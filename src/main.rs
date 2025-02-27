#![allow(unused)]
use reqwest::{self, Client};
use serde_json::{Value,json};
use std::{fmt::format, fs::File, io::Write};

#[tokio::main]
async fn main()->Result<(),Box<dyn std::error::Error>> {
    let api_key = std::env::var("OPENAI_API_KEY").expect("api key not found");

    let client = Client::new();
    let prompt = "Generate a python script that prints *hello world*..";

    let response = client.post("https://api.openai.com/v1/chat/completions")
        .header("Authorization", format!("Bearer {}",api_key))
        .header("Content-type", "application/json")
        .json(&json!({
            "model": "gpt-4",
            "messages": [{"role": "user", "content": prompt}],
            "max_tokens": 100
        }))
        .send()
        .await?;

    let response_json:Value = response.json().await?;
    let script = response_json["choices"][0]["message"]["content"]
        .as_str()
        .unwrap_or("print('No response from ai')");

    let mut file = File::create("script.py")?;
    file.write_all(script.as_bytes())?;
    println!("Python script generated: script.py"); 

    Ok(())
}
