# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## What this is

A small interactive Rust CLI (binary `calculator`, crate `token_calculator`) that estimates LLM API spend. The user picks a provider and model (via `--provider` / `--model` flags, or `inquire` select menus for whichever flag is missing), then types three numbers on stdin: average prompt tokens, average completion tokens, and number of requests. All prices are hard-coded in `src/prices.rs` as USD per 1M tokens, so each release is effectively a price-table snapshot. There are no network calls, config files or persisted state.

`docs/mental-model.md` has the full architecture write-up and a list of known traps. Read it before non-trivial edits.

## Commands

```sh
cargo build                     # debug build
cargo build --release           # -> ./target/release/calculator
cargo test                      # all unit tests (inline #[cfg(test)] modules)
cargo test test_listed_models_are_unique   # single test by name substring
cargo run -- --provider openai --model gpt-4.1

# Non-interactive smoke run (stdin = prompt tokens, completion tokens, requests)
printf '1000\n500\n1000\n' | cargo run -q -- --provider anthropic --model claude-opus-5-5
```

CI (`.github/workflows/rust.yml`) runs `cargo build` + `cargo test` on push/PR to `main`. Pushing a `v*` tag triggers `release.yml`, which builds 5 targets via `build-artifacts.yml` (uses `--locked`, so keep `Cargo.lock` in sync), adds `SHA256SUMS` and creates a GitHub release. The `Cargo.toml` version, the tag, and the hard-coded version in the README download example are all bumped by hand.

## Architecture

- `src/main.rs`: clap args, menus, stdin parsing, output. Calls `get_model_prices`, then `CostResponse::calculate_cost` once for input and once for output.
- `src/prices.rs`: the whole data model. `PROVIDERS` array (`[&str; N]`), one `get_<provider>_model_prices` match per provider returning `(input, output)`, a `get_model_prices` dispatcher, and `get_provider_models` that returns the menu list per provider.
- `src/cost/mod.rs`: `CostResponse { num_tokens, cost }`.
- `src/helpers.rs`: float-to-string formatting.

### Price table conventions (tests depend on these)

- Every model appears **twice** in `src/prices.rs`: as a match arm in `get_<provider>_model_prices` and in the `get_provider_models` vec, in the same order (newest/flagship first, long-context tier entries right after their base model).
- `test_every_listed_model_has_explicit_price` uses `include_str!("prices.rs")` to check that each menu entry has a literal `"id" => (` arm. Keep each arm on one line in the form `"id" => (in, out),`. Reformatting (e.g. rustfmt wrapping) breaks the test. `test_listed_models_are_unique` guards against duplicates.
- Model names are the provider's real API ids. Long-context pricing is a separate, user-selected entry with a threshold suffix (`-200k`, `-272k`, `-100k`). Nothing switches tiers automatically.
- Each provider fn has a `_ =>` fallback price, and unknown model ids silently use it.
- Adding a provider means updating `PROVIDERS` (and its array length), adding a new `get_<provider>_model_prices`, adding an arm in both `get_model_prices` and `get_provider_models`, adding a README section, and adding a source URL in `.claude/skills/update-model-prices/references/providers.md`.

### Updating prices

Use the `update-model-prices` project skill (`.claude/skills/update-model-prices/SKILL.md`). It sets the inclusion rules (standard tier only, no contact-sales or limited-access models, Groq production models only, today's price during promos) and requires every price to come from a provider page read in the current session, never from memory. When prices change, update the README's "as of" date and the per-provider model lists together.

## Known traps

- **`calculate_cost` doesn't scale with token count.** It returns the per-1M price as the cost whenever tokens > 0, and the tests in `src/cost/mod.rs` assert this behaviour. A fix needs `num_tokens * price / 1_000_000` and updated tests.
- `tokens * requests` is `i32` and overflows at about 2.1B (it panics in debug builds).
- `_price_input_1k` / `_price_output_1k` in `main.rs` actually hold per-**1M** prices.
- An unknown provider with `--model` falls through to a stale `(0.002, 0.008)` in `get_model_prices`. Without `--model`, it gives an empty menu and panics.
- The Perplexity `ends_with("online")` surcharge path in `main.rs` is dead code: no current model matches it, and `get_perplexity_online_requests_cost` uses integer division.
- All input parsing uses `.unwrap()`, so invalid input panics.
