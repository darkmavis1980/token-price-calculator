---
name: update-model-prices
description: Use when the user wants to refresh, update, or sync the model list or API prices in this token calculator — e.g. "update the prices", "add the new models", "the pricing is outdated", "add <new model> to <provider>", "check if any models were deprecated", or "add a new provider". Fetches current per-1M-token prices from each provider's official pricing page and updates src/prices.rs, its tests, and the README.
---

# Update model prices

This repo is a CLI that estimates API spend from token counts. Its value depends entirely on the price table in `src/prices.rs` being current, and provider catalogs change every few weeks (new models, price cuts, retirements). Your own knowledge of prices is almost certainly stale, so every number you write must come from a page you read in this session.

## 1. Read the current state

Read `src/prices.rs` and the model sections of `README.md`. Note which providers exist (`PROVIDERS`), the model ids per provider, and the fallback (`_ =>`) price in each match. Run `cargo test` once to confirm a green baseline.

If the user asked for a single provider or model, scope the rest of this workflow to just that.

## 2. Research prices — one subagent per provider, in parallel

Spawn one `general-purpose` agent per provider in a single message so they run concurrently. Research is the slow part, and per-provider agents keep raw page dumps out of your context. Give each agent:

- today's date and the primary source URLs from `references/providers.md` (tell it pages may have moved and to follow redirects)
- the current model ids in the repo for that provider, so it can say which are gone
- the ask: every currently-listed text model with API id, **standard-tier** input and output USD per 1M tokens, any prompt-length tiers, any scheduled price changes, and deprecation/shutdown dates
- explicit skips: cached input, batch, audio/image/video, realtime, embeddings, fine-tuning
- "Return a compact table plus the source URLs you actually read, flag anything not verified from a primary source, and do not edit files."

Ask agents to prefer raw HTML (curl) over WebFetch summaries when the page is a big table — summarizers have silently dropped rows before. Third-party aggregator sites are fine for discovering that a model exists, never for its price.

## 3. Decide what goes in

Apply these rules consistently; they keep the menu usable and the numbers honest:

- **Drop** models that are shut down, no longer priced, or shutting down within ~2 weeks. Keep models that are deprecated but still callable for a while, and note their retirement date in the README.
- **Skip** models with no public price ("Contact sales") and limited-access/allowlist-only models — the calculator can't give a meaningful answer for them.
- **Groq: stable/production models only** (the README promises this).
- **Use the provider's real API id** as the model name (e.g. `claude-opus-5-5`, `openai/gpt-oss-120b`), since that's what users will recognize from their code.
- **Prompt-length tiers** become a second entry with a suffix naming the threshold: `gemini-2.5-pro` + `gemini-2.5-pro-200k`. Only add a tier entry when the provider prints the price or states an explicit rule for that specific model; don't extrapolate a rule from a sibling model.
- **Scheduled price changes / promos:** use today's price and mention the change date in the README line for that model.
- If something is ambiguous (e.g. a whole API surface being deprecated, as happened with Perplexity Sonar), make the conservative call and surface it in your final summary rather than stalling.

## 4. Edit the code

In `src/prices.rs` each model appears in **two** places — the provider's `get_<provider>_model_prices` match arm and the `get_provider_models` vec. Keep them in the same order (newest/flagship first, tier entries right after their base model). Set each provider's `_ =>` fallback to a sensible mid-range current model rather than a retired one.

Tests:
- Remove tests for models you removed.
- Add or update a few spot-check tests per provider, including at least one tier entry.
- Keep `test_every_listed_model_has_explicit_price` and `test_listed_models_are_unique` — they catch the most common mistake (a model in the menu silently falling through to the default price). If they don't exist yet, add them.

Adding a new provider means: add to `PROVIDERS` (bump the array length), add a `get_<provider>_model_prices` fn, wire it into `get_model_prices` and `get_provider_models`, add README section and a source URL to `references/providers.md`.

## 5. Update the README

- Update the "pricing as of ..." date line at the top.
- Rewrite each changed provider section: one bullet per model with id and a short description, retirement/price-change notes inline, tier entries listed, and a `[Pricing reference](url)` link at the end.
- Update the vendor list at the top if providers changed.

## 6. Verify

- `cargo test` — all green.
- Smoke-run one model per changed provider, e.g. `printf '1000\n500\n1000\n' | cargo run -q -- --provider anthropic --model claude-opus-5-5`.
- If you learned a new source URL or gotcha (a page moved, a summarizer dropped rows), update `references/providers.md` so the next run benefits.

Don't commit, bump the version, or push unless the user asks.

## 7. Report back

Summarize per provider: added, removed (with reason), price changes on existing models, and anything not verified from a primary source or derived from a stated rule. Keep it short — the user mostly wants to know what moved and what to double-check.
