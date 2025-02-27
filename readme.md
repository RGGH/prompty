# OpenAI Python Script Generator [![Rust](https://github.com/RGGH/prompty/actions/workflows/rust.yml/badge.svg)](https://github.com/RGGH/prompty/actions/workflows/rust.yml)

This Rust program interacts with OpenAI's API to generate a Python script that fetches the latest Bitcoin price from the Gemini API. It then writes the script to a file and executes it automatically.

## Features
- Uses OpenAI's GPT-4 model to generate a Python script dynamically
- Writes the script to `script.py`
- Executes the generated Python script
- Handles errors gracefully

## Prerequisites
- Rust (with `tokio` and `reqwest` dependencies)
- OpenAI API key
- Python installed (`python` or `python3` command available)

## Installation
1. Clone the repository:
   ```sh
   git clone https://github.com/RGGH/prompty.git
   cd prompty
   ```
2. Set up the OpenAI API key as an environment variable:
   ```sh
   export OPENAI_API_KEY=your_api_key_here
   ```
3. Build and run the program:
   ```sh
   cargo run
   ```

## How It Works
1. Fetches the OpenAI API key from the environment.
2. Sends a request to OpenAI's API to generate a Python script.
3. Writes the response to `script.py`.
4. Determines whether `python` or `python3` is available.
5. Executes the script and prints the output.

## Example Output
```sh
Python script generated: script.py
Python script output: 85000.32
```

## Dependencies
- [tokio](https://crates.io/crates/tokio)
- [reqwest](https://crates.io/crates/reqwest)
- [serde_json](https://crates.io/crates/serde_json)

## License
This project is licensed under the MIT License.

## Todo
Take user input to use as prompt

## Contributing
Pull requests are welcome! Feel free to submit issues or feature requests.

## Author
[RGGH](https://github.com/RGGH)


