# Provider pricing sources

Primary sources per provider, plus gotchas learned on previous runs. Update this file when a URL moves or a new trap shows up.

Last verified: 2026-11-10.

## OpenAI
- Pricing: https://developers.openai.com/api/docs/pricing (platform.openai.com/docs/pricing 301-redirects here)
- Per-model pages: https://developers.openai.com/api/docs/models/{id} — long-context rules are stated here, not always in the table
- Deprecations: https://developers.openai.com/api/docs/deprecations
- Gotchas:
  - Use the `"tier":"standard"` table embedded in the page data. Many cells are rendered client-side, so curl the raw HTML and parse it; WebFetch's summary omitted gpt-5.3-codex last time.
  - The ">272K" prices are not in the server-rendered HTML; they live in a hardcoded `m.standard` table in the pricing page's JS bundle (`/_astro/pricing.*.js`). Some models (e.g. gpt-5.5-pro) have a long-context price there but no rule on their model page.
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
- Pricing: https://ai.google.dev/gemini-api/docs/pricing → now redirects to https://aistudio.google.com/docs/pricing (`.md` version also works) (use the paid tier, Standard, not Batch)
- Deprecations: https://ai.google.dev/gemini-api/docs/deprecations
- Gotchas:
  - Some models are tiered by prompt length (<=200k vs >200k); others have a dated introductory price that later doubles — these are different things.
  - In the raw HTML the "<= 200k" label can be dropped from tiered rows; the first price is the lower tier.
  - One curl of the redirected pricing URL once returned an unrelated (OpenAI) page — check the body contains "gemini" before parsing, and refetch if not.
  - Live, Omni (video), and Robotics models also have text prices — skip them unless the user asks.

## Perplexity
- Pricing: https://docs.perplexity.ai/docs/getting-started/pricing (no longer has Sonar or per-model token prices)
- Agent API models + per-token prices: https://docs.perplexity.ai/docs/agent-api/models (append `.md` for raw)
- Legacy Sonar prices: https://docs.perplexity.ai/docs/legacy/sonar/models/{id} (not in llms.txt/sitemap)
- Changelog: https://docs.perplexity.ai/changelog
- Migration guide: https://docs.perplexity.ai/docs/agent-api/migrate-from-sonar/overview
- Gotchas:
  - Sonar Chat Completions support ended 2026-09-27; sync/streaming requests are reformulated as Agent API requests (billing rate undocumented). As of 2026-11-10 the repo keeps legacy `sonar*` plus Perplexity-hosted `perplexity/*` Agent API models; resold OpenAI/Anthropic/Google/xAI models are skipped as duplicates. Re-check whether to drop legacy Sonar.
  - Agent API effort levels (formerly presets) have no own price; they bill at the underlying model's rate.
  - Token prices exclude per-request search fees and Deep Research extras (citation/reasoning tokens, search queries). The calculator only models tokens.

## Groq
- Models + prices: https://console.groq.com/docs/models (groq.com/pricing now redirects to the homepage with no table)
- Deprecations: https://console.groq.com/docs/deprecations
- Gotchas:
  - Catalog churns fast — most Llama models were shut down for self-serve in 2026; remaining ones are "Contact Sales" (skip).
  - Only include Production models; Preview models are excluded per the README.
