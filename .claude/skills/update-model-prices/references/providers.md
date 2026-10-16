# Provider pricing sources

Primary sources per provider, plus gotchas learned on previous runs. Update this file when a URL moves or a new trap shows up.

Last verified: 2026-10-16.

## OpenAI
- Pricing: https://developers.openai.com/api/docs/pricing (platform.openai.com/docs/pricing 301-redirects here)
- Per-model pages: https://developers.openai.com/api/docs/models/{id} — long-context rules are stated here, not always in the table
- Deprecations: https://developers.openai.com/api/docs/deprecations
- Gotchas:
  - Use the `"tier":"standard"` table embedded in the page data. Many cells are rendered client-side, so curl the raw HTML and parse it; WebFetch's summary omitted gpt-5.3-codex last time.
  - Long context (>272K input tokens) is typically 2x input / 1.5x output for the whole request, but only for models whose page says so.
  - Many models carry a shutdown date a few weeks out — check deprecations before keeping older ones.

## Anthropic
- Pricing: https://platform.claude.com/docs/en/about-claude/pricing (docs.claude.com redirects here); https://claude.com/pricing as a cross-check
- Models: https://platform.claude.com/docs/en/about-claude/models/overview
- Deprecations: https://platform.claude.com/docs/en/about-claude/model-deprecations
- Gotchas:
  - Only some models have a long-context tier, and the threshold varies (Haiku 5.5 tiers at 100K, not 200K).
  - Mythos models are limited-access — skip.
  - Claude 4.7+ uses a newer tokenizer (~30% more tokens for the same text); worth mentioning if the user asks about comparing costs across generations.

## Google Gemini
- Pricing: https://ai.google.dev/gemini-api/docs/pricing (use the paid tier, Standard, not Batch)
- Deprecations: https://ai.google.dev/gemini-api/docs/deprecations
- Gotchas:
  - Some models are tiered by prompt length (<=200k vs >200k); others have a dated introductory price that later doubles — these are different things.
  - In the raw HTML the "<= 200k" label can be dropped from tiered rows; the first price is the lower tier.
  - Live, Omni (video), and Robotics models also have text prices — skip them unless the user asks.

## Perplexity
- Pricing: https://docs.perplexity.ai/getting-started/pricing
- Changelog: https://docs.perplexity.ai/changelog
- Migration guide: https://docs.perplexity.ai/docs/agent-api/migrate-from-sonar/overview
- Gotchas:
  - Sonar Chat Completions is being replaced by the Agent API (support ended 2026-09-27). The Sonar table is still published; Agent API presets have no per-token price list yet. Re-check whether to switch models.
  - Token prices exclude per-request search fees and Deep Research extras (citation/reasoning tokens, search queries). The calculator only models tokens.

## Groq
- Models + prices: https://console.groq.com/docs/models (groq.com/pricing now redirects to the homepage with no table)
- Deprecations: https://console.groq.com/docs/deprecations
- Gotchas:
  - Catalog churns fast — most Llama models were shut down for self-serve in 2026; remaining ones are "Contact Sales" (skip).
  - Only include Production models; Preview models are excluded per the README.
