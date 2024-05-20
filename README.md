# OpenAI Tokens calculator

This takes in account only for some models with the current pricing as Feb 2024.

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

The CLI will ask you for the model you want to use, you can choose between the followings:

- `gpt-3.5-turbo`: The default model.
- `gpt-3.5-turbo-instruct`: The GPT-3.5 Turbo Instruct model.
- `gpt-4-turbo`: The GPT-4 Turbo model.
- `gpt-4`: The GPT-4 model.
- `gpt-4-32k`: The GPT-4 32k model.
- `gtp-4o`: The GPT-4o model.

To pass a model you can call the CLI with the `--model` flag, like this:

```sh
./target/release/calculator --model gpt-4-turbo
```
