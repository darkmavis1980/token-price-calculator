# Tokens Price calculator

This takes in account only for some models with the current pricing as of mid-October 2026 (standard tier, USD per 1M tokens).

Models with long-context pricing have a separate entry with a suffix (e.g. `-200k`, `-272k`, `-100k`): pick that entry when your prompts exceed that size.

The AI vendors are the following:

- Google Gemini
- OpenAI
- Perplexity AI
- Anthropic
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

- `gemini-4-argon`: The Google Gemini 4 Argon model (introductory price until 2026-11-30, it doubles after).
- `gemini-3.8-flash`: The Google Gemini 3.8 Flash model (introductory price until 2026-12-31, it doubles after).
- `gemini-3.6-flash`: The Google Gemini 3.6 Flash model (introductory price until 2026-12-31, it doubles after).
- `gemini-3.5-flash-lite`: The Google Gemini 3.5 Flash Lite model.
- `gemini-3.1-pro-preview`: The Google Gemini 3.1 Pro Preview model.
- `gemini-3.1-pro-preview-200k`: The Google Gemini 3.1 Pro Preview model, prompts over 200k tokens.
- `gemini-3.1-flash-lite`: The Google Gemini 3.1 Flash Lite model.
- `gemini-3-flash-preview`: The Google Gemini 3 Flash Preview model.
- `gemini-2.5-pro`: The Google Gemini 2.5 Pro model.
- `gemini-2.5-pro-200k`: The Google Gemini 2.5 Pro model, prompts over 200k tokens.
- `gemini-2.5-flash`: The Google Gemini 2.5 Flash model.
- `gemini-2.5-flash-lite`: The Google Gemini 2.5 Flash Lite model.

[Pricing reference](https://ai.google.dev/gemini-api/docs/pricing)

### OpenAI

- `gpt-6-astra`: The OpenAI GPT-6 Astra model.
- `gpt-6.1-sol`: The OpenAI GPT-6.1 Sol model.
- `gpt-6-sol`: The OpenAI GPT-6 Sol model.
- `gpt-6-luna`: The OpenAI GPT-6 Luna model.
- `gpt-5.6-sol`: The OpenAI GPT-5.6 Sol model (promotional price through at least 2026-11-21).
- `gpt-5.6-terra`: The OpenAI GPT-5.6 Terra model.
- `gpt-5.6-luna`: The OpenAI GPT-5.6 Luna model.
- `gpt-5.5`: The OpenAI GPT-5.5 model.
- `gpt-5.5-pro`: The OpenAI GPT-5.5 Pro model.
- `gpt-5.4`: The OpenAI GPT-5.4 model.
- `gpt-5.4-pro`: The OpenAI GPT-5.4 Pro model.
- `gpt-5.4-mini`: The OpenAI GPT-5.4 Mini model.
- `gpt-5.4-nano`: The OpenAI GPT-5.4 Nano model.
- `gpt-5.3-codex`: The OpenAI GPT-5.3 Codex model.
- `gpt-5.2`: The OpenAI GPT-5.2 model.
- `gpt-5.2-pro`: The OpenAI GPT-5.2 Pro model.
- `gpt-5.1`: The OpenAI GPT-5.1 model.
- `gpt-5`: The OpenAI GPT-5 model.
- `gpt-5-mini`: The OpenAI GPT-5 Mini model.
- `gpt-5-nano`: The OpenAI GPT-5 Nano model.
- `gpt-5-pro`: The OpenAI GPT-5 Pro model.
- `gpt-4.1`: The OpenAI GPT-4.1 model.
- `gpt-4.1-mini`: The OpenAI GPT-4.1 Mini model.
- `gpt-4o`: The OpenAI GPT-4o model.
- `gpt-4o-mini`: The OpenAI GPT-4o Mini model.
- `o3`: The OpenAI O3 model.
- `o3-pro`: The OpenAI O3 Pro model.

Long-context (prompts over 272k tokens) entries: `gpt-6-astra-272k`, `gpt-6.1-sol-272k`, `gpt-6-sol-272k`, `gpt-6-luna-272k`, `gpt-5.6-sol-272k`, `gpt-5.6-terra-272k`, `gpt-5.5-272k`, `gpt-5.4-272k`, `gpt-5.4-pro-272k`.

[Pricing reference](https://developers.openai.com/api/docs/pricing)

### Perplexity AI

- `sonar`: The Perplexity AI Sonar model.
- `sonar-pro`: The Perplexity AI Sonar Pro model.
- `sonar-reasoning-pro`: The Perplexity AI Sonar Reasoning Pro model.
- `sonar-deep-research`: The Perplexity AI Sonar Deep Research model.

Prices are token costs only; per-request search fees and Deep Research citation/reasoning/search charges are not included.

[Pricing reference](https://docs.perplexity.ai/getting-started/pricing)

### Anthropic

- `claude-fable-5-1`: The Anthropic Claude Fable 5.1 model.
- `claude-fable-5`: The Anthropic Claude Fable 5 model.
- `claude-opus-5-5`: The Anthropic Claude Opus 5.5 model.
- `claude-opus-5`: The Anthropic Claude Opus 5 model.
- `claude-opus-4-8`: The Anthropic Claude Opus 4.8 model.
- `claude-opus-4-7`: The Anthropic Claude Opus 4.7 model.
- `claude-opus-4-6`: The Anthropic Claude Opus 4.6 model.
- `claude-opus-4-5`: The Anthropic Claude Opus 4.5 model.
- `claude-sonnet-5-5`: The Anthropic Claude Sonnet 5.5 model.
- `claude-sonnet-5`: The Anthropic Claude Sonnet 5 model.
- `claude-sonnet-4-6`: The Anthropic Claude Sonnet 4.6 model.
- `claude-sonnet-4-5`: The Anthropic Claude Sonnet 4.5 model (retires 2026-11-30).
- `claude-haiku-5-5`: The Anthropic Claude Haiku 5.5 model.
- `claude-haiku-5-5-100k`: The Anthropic Claude Haiku 5.5 model, prompts over 100k tokens.
- `claude-haiku-4-5`: The Anthropic Claude Haiku 4.5 model.

[Pricing reference](https://platform.claude.com/docs/en/about-claude/pricing)

### Groq

- `openai/gpt-oss-120b`: The Groq GPT-OSS 120B model.
- `openai/gpt-oss-20b`: The Groq GPT-OSS 20B model.

[Pricing reference](https://console.groq.com/docs/models)

To pass a model you can call the CLI with the `--model` flag, like this:

```sh
./target/release/calculator --provider openai --model gpt-4.1
```
