# Tokens Price calculator

This takes in account only for some models with the current pricing as June 2025.

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

- `gemini-1.5-pro`: The Google Gemini 1.5 Pro model.
- `gemini-1.5-pro-128k`: The Google Gemini 1.5 Pro 128k+ model.
- `gemini-1.5-flash`: The Google Gemini 1.5 Flash model.
- `gemini-1.5-flash-128k`: The Google Gemini 1.5 Flash 128k+ model.
- `gemini-2.0-flash`: The Google Gemini 2.0 Flash model.
- `gemini-2.0-flash-lite`: The Google Gemini 2.0 Flash Lite Preview 02-05 model.
- `gemini-2.5-flash`: The Google Gemini 2.5 Flash model.
- `gemini-2.5-flash-lite`: The Google Gemini 2.5 Flash Lite model.
- `gemini-2.5-pro`: The Google Gemini 2.5 Pro model.
- `gemini-2.5-pro-200k`: The Google Gemini 2.5 Pro 200k model.


### OpenAI

- `gtp-4o`: The OpenAI GPT-4o model, default model.
- `gtp-4o-mini`: The OpenAI GPT-4o-mini model.
- `gpt-4.1`: The OpenAI GPT-4.1 model.
- `gpt-4.1-mini`: The OpenAI GPT-4.1-mini model.
- `gpt-4.1-nano`: The OpenAI GPT-4.1-nano model.
- `o1`: The OpenAI O1 model.
- `o1-pro`: The OpenAI O1 Pro model.
- `o3`: The OpenAI O3 model.
- `o3-pro`: The OpenAI O3 Pro model.
- `o3-mini`: The OpenAI O3 Mini model.
- `o4-mini`: The OpenAI O4 Mini model.

### Perplexity AI

- `sonar`: The Perplexity AI Sonar model.
- `sonar-pro`: The Perplexity AI Sonar Pro model.
- `sonar-reasoning`: The Perplexity AI Sonar Reasoning model.
- `sonar-reasoning-pro`: The Perplexity AI Sonar Reasoning Pro model.
- `sonar-deep-research`: The Perplexity AI Sonar Deep Research model.
- `r1-1776`: The Perplexity AI R1 1776 model.

### Groq

- `gemma2-9b-it`: The Groq Gemma2 9B 8k model.
- `gemma-7b-it`: The Groq Gemma 7B 8k Instruct model.
- `llama-3.1-70b-versatile`: The Groq Llama 3.1 70B Versatile 128k model.
- `llama-3.1-8b-instant`: The Groq Llama 3.1 8B Instant 128k model.
- `llama-3.3-70b-versatile`: The Groq Llama 3.3 70B Versatile model.
- `llama-4-scout`: The Groq Llama 4 Scout model.
- `llama-4-maverick`: The Groq Llama 4 Maverick model.
- `llama-guard-4-12b`: The Groq Llama Guard 4 12B model.

To pass a model you can call the CLI with the `--model` flag, like this:

```sh
./target/release/calculator --provider openai --model gpt-4.1
```
