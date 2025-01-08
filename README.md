# Tokens Price calculator

This takes in account only for some models with the current pricing as September 2024.

The AI vendors are the following:

- Google Gemini
- OpenAI
- Perplexity AI
- Groq (only stable models)

## Requirements

1. Make sure you have Rust installed on your system. You can check the official installation guide here: https://www.rust-lang.org/tools/install
2. Make sure that you also have cargo installed correctly, just run `cargo --version`.

## Run the code

You can either running with cargo, using `cargo run`, or you can build the binary file with:

```sh
cargo build --release
```

Then you can run the binary file with:

```sh
./target/release/calculator
```

## Set the model

The CLI will ask you for the provider, for example 'openai', and the model you want to use, you can choose between the followings:

### Google

- `gemini-1-pro`: The Google Gemini 1 Pro model.
- `gemini-1.5-pro`: The Google Gemini 1.5 Pro model.
- `gemini-1.5-pro-128k`: The Google Gemini 1.5 Pro 128k+ model.
- `gemini-1.5-flash`: The Google Gemini 1.5 Flash model.
- `gemini-1.5-flash-128k`: The Google Gemini 1.5 Flash 128k+ model.

### OpenAI

- `gtp-4o`: The OpenAI GPT-4o model, default model.
- `gtp-4o-mini`: The OpenAI GPT-4o-mini model.
- `o1`: The OpenAI O1 model.
- `o1-mini`: The OpenAI O1 Mini model.
- `gpt-3.5-turbo`: The OpenAI GPT-3.5 model.
- `gpt-3.5-turbo-instruct`: The OpenAI GPT-3.5 Turbo Instruct model.
- `gpt-4-turbo`: The OpenAI GPT-4 Turbo model.
- `gpt-4`: The OpenAI GPT-4 model.
- `gpt-4-32k`: The OpenAI GPT-4 32k model.

### Perplexity AI

- `llama-3.1-sonar-small-128k-online`: The Perplexity AI Llama 3.1 Sonar Small 128k Online model.
- `llama-3.1-sonar-large-128k-online`: The Perplexity AI Llama 3.1 Sonar Large 128k Online model.
- `llama-3.1-sonar-huge-128k-online`: The Perplexity AI Llama 3.1 Sonar Huge 128k Online model.

### Groq

- `gemma2-9b-it`: The Groq Gemma2 9B 8k model.
- `gemma-7b-it`: The Groq Gemma 7B 8k Instruct model.
- `llama-3.1-70b-versatile`: The Groq Llama 3.1 70B Versatile 128k model.
- `llama-3.1-8b-instant`: The Groq Llama 3.1 8B Instant 128k model.

To pass a model you can call the CLI with the `--model` flag, like this:

```sh
./target/release/calculator --provider openai --model gpt-4-turbo
```
