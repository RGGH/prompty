use reqwest::Client;
use serde_json::{json, Value};
use std::{env, fs::File, io::Write, process::Command, path::Path};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Fetch the OpenAI API key from the environment
    let api_key = env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY not set in environment");

    let client = Client::new();
    let prompt = "Generate a Python script that fetches the bitcoin price from Gemini API - just give the code, so it can be run, no smalltalk! - no comments like 'Here's a simple Python script that imports the `requests` library to send HTTP requests and fetch the latest Bitcoin price from the Gemini API:' - no backticks either ok, just code";

    let response = client
        .post("https://api.openai.com/v1/chat/completions")
        .header("Authorization", format!("Bearer {}", api_key))
        .header("Content-Type", "application/json")
        .json(&json!({
            "model": "gpt-4",
            "messages": [{"role": "user", "content": prompt}],
            "max_tokens": 100
        }))
        .send()
        .await?;

    let response_json: Value = response.json().await?;
    let script = response_json["choices"][0]["message"]["content"]
        .as_str()
        .unwrap_or("print('No response from AI')");

    // Write the Python script to a file
    let mut file = File::create("script.py")?;
    file.write_all(script.as_bytes())?;

    println!("Python script generated: script.py");

    // Check if the Python executable is available and use the correct command
    let python_command = if Command::new("python").output().is_ok() {
        "python"  // Use python if available
    } else {
        "python3"  // Otherwise use python3
    };

    // Get the absolute path of the script
    let script_path = Path::new("script.py")
        .canonicalize()?
        .to_str()
        .unwrap()
        .to_string();

    // Run the Python script
    let output = Command::new(python_command)
        .arg(script_path)
        .output()?;

    // Handle errors or print the output
    if !output.status.success() {
        eprintln!("Error running Python script: {:?}", output);
        eprintln!("stderr: {}", String::from_utf8_lossy(&output.stderr));
    } else {
        println!("Python script output: {}", String::from_utf8_lossy(&output.stdout));
    }

    Ok(())
}

