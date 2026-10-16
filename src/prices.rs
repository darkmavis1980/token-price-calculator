pub fn get_google_model_prices(model: &str) -> (f32, f32) {
    match model {
        "gemini-4-argon" => (2.0, 10.0),
        "gemini-3.8-flash" => (0.75, 3.75),
        "gemini-3.6-flash" => (0.75, 3.75),
        "gemini-3.5-flash-lite" => (0.3, 2.5),
        "gemini-3.1-pro-preview" => (2.0, 12.0),
        "gemini-3.1-pro-preview-200k" => (4.0, 18.0),
        "gemini-3.1-flash-lite" => (0.25, 1.5),
        "gemini-3-flash-preview" => (0.5, 3.0),
        "gemini-2.5-pro" => (1.25, 10.0),
        "gemini-2.5-pro-200k" => (2.5, 15.0),
        "gemini-2.5-flash" => (0.3, 2.5),
        "gemini-2.5-flash-lite" => (0.1, 0.4),
        _ => (0.75, 3.75)
    }
}

pub fn get_perplexity_model_prices(model: &str) -> (f32, f32) {
    match model {
        "sonar" => (1.0, 1.0),
        "sonar-pro" => (3.0, 15.0),
        "sonar-reasoning-pro" => (2.0, 8.0),
        "sonar-deep-research" => (2.0, 8.0),
        _ => (1.0, 1.0)
    }
}

pub fn get_openai_model_prices(model: &str) -> (f32, f32) {
    match model {
        "gpt-6-astra" => (10.0, 50.0),
        "gpt-6-astra-272k" => (20.0, 75.0),
        "gpt-6.1-sol" => (2.0, 10.0),
        "gpt-6.1-sol-272k" => (4.0, 15.0),
        "gpt-6-sol" => (2.0, 10.0),
        "gpt-6-sol-272k" => (4.0, 15.0),
        "gpt-6-luna" => (0.1, 0.5),
        "gpt-6-luna-272k" => (0.2, 0.75),
        "gpt-5.6-sol" => (4.0, 20.0),
        "gpt-5.6-sol-272k" => (8.0, 30.0),
        "gpt-5.6-terra" => (2.0, 12.0),
        "gpt-5.6-terra-272k" => (4.0, 18.0),
        "gpt-5.6-luna" => (0.2, 1.2),
        "gpt-5.5" => (5.0, 30.0),
        "gpt-5.5-272k" => (10.0, 45.0),
        "gpt-5.5-pro" => (30.0, 180.0),
        "gpt-5.4" => (2.5, 15.0),
        "gpt-5.4-272k" => (5.0, 22.5),
        "gpt-5.4-pro" => (30.0, 180.0),
        "gpt-5.4-pro-272k" => (60.0, 270.0),
        "gpt-5.4-mini" => (0.75, 4.5),
        "gpt-5.4-nano" => (0.2, 1.25),
        "gpt-5.3-codex" => (1.75, 14.0),
        "gpt-5.2" => (1.75, 14.0),
        "gpt-5.2-pro" => (21.0, 168.0),
        "gpt-5.1" => (1.25, 10.0),
        "gpt-5" => (1.25, 10.0),
        "gpt-5-mini" => (0.25, 2.0),
        "gpt-5-nano" => (0.05, 0.4),
        "gpt-5-pro" => (15.0, 120.0),
        "gpt-4.1" => (2.0, 8.0),
        "gpt-4.1-mini" => (0.4, 1.6),
        "gpt-4o" => (2.5, 10.0),
        "gpt-4o-mini" => (0.15, 0.6),
        "o3" => (2.0, 8.0),
        "o3-pro" => (20.0, 80.0),
        _ => (2.0, 10.0)
    }
}

pub fn get_anthropic_model_prices(model: &str) -> (f32, f32) {
    match model {
        "claude-fable-5-1" => (10.0, 50.0),
        "claude-fable-5" => (10.0, 50.0),
        "claude-opus-5-5" => (4.0, 20.0),
        "claude-opus-5" => (5.0, 25.0),
        "claude-opus-4-8" => (5.0, 25.0),
        "claude-opus-4-7" => (5.0, 25.0),
        "claude-opus-4-6" => (5.0, 25.0),
        "claude-opus-4-5" => (5.0, 25.0),
        "claude-sonnet-5-5" => (2.0, 10.0),
        "claude-sonnet-5" => (2.0, 10.0),
        "claude-sonnet-4-6" => (3.0, 15.0),
        "claude-sonnet-4-5" => (3.0, 15.0),
        "claude-haiku-5-5" => (0.1, 0.5),
        "claude-haiku-5-5-100k" => (0.5, 2.5),
        "claude-haiku-4-5" => (1.0, 5.0),
        _ => (2.0, 10.0)
    }
}
pub fn get_groq_model_prices(model: &str) -> (f32, f32) {
    match model {
        "openai/gpt-oss-120b" => (0.15, 0.6),
        "openai/gpt-oss-20b" => (0.075, 0.3),
        _ => (0.075, 0.3)
    }
}

pub fn get_model_prices(provider: &str, model: &str) -> (f32, f32) {
    match provider {
        "openai" => get_openai_model_prices(model),
        "google" => get_google_model_prices(model),
        "perplexity" => get_perplexity_model_prices(model),
        "groq" => get_groq_model_prices(model),
        "anthropic" => get_anthropic_model_prices(model),
        _ => (0.002, 0.008)
    }
}

pub fn get_perplexity_online_requests_cost(requests: i32) -> i32 {
    let cost_per_million_requests: i32 = 5;
    let cost = (requests as i32 * cost_per_million_requests) / 1000000;
    cost
}

pub const PROVIDERS: [&str; 5] = ["openai", "google", "perplexity", "groq", "anthropic"];

pub fn get_provider_models(provider: &str) -> Vec<&str> {
    match provider {
        "openai" => vec![
            "gpt-6-astra",
            "gpt-6-astra-272k",
            "gpt-6.1-sol",
            "gpt-6.1-sol-272k",
            "gpt-6-sol",
            "gpt-6-sol-272k",
            "gpt-6-luna",
            "gpt-6-luna-272k",
            "gpt-5.6-sol",
            "gpt-5.6-sol-272k",
            "gpt-5.6-terra",
            "gpt-5.6-terra-272k",
            "gpt-5.6-luna",
            "gpt-5.5",
            "gpt-5.5-272k",
            "gpt-5.5-pro",
            "gpt-5.4",
            "gpt-5.4-272k",
            "gpt-5.4-pro",
            "gpt-5.4-pro-272k",
            "gpt-5.4-mini",
            "gpt-5.4-nano",
            "gpt-5.3-codex",
            "gpt-5.2",
            "gpt-5.2-pro",
            "gpt-5.1",
            "gpt-5",
            "gpt-5-mini",
            "gpt-5-nano",
            "gpt-5-pro",
            "gpt-4.1",
            "gpt-4.1-mini",
            "gpt-4o",
            "gpt-4o-mini",
            "o3",
            "o3-pro",
        ],
        "google" => vec![
            "gemini-4-argon",
            "gemini-3.8-flash",
            "gemini-3.6-flash",
            "gemini-3.5-flash-lite",
            "gemini-3.1-pro-preview",
            "gemini-3.1-pro-preview-200k",
            "gemini-3.1-flash-lite",
            "gemini-3-flash-preview",
            "gemini-2.5-pro",
            "gemini-2.5-pro-200k",
            "gemini-2.5-flash",
            "gemini-2.5-flash-lite",
        ],
        "perplexity" => vec![
            "sonar",
            "sonar-pro",
            "sonar-reasoning-pro",
            "sonar-deep-research",
        ],
        "anthropic" => vec![
            "claude-fable-5-1",
            "claude-fable-5",
            "claude-opus-5-5",
            "claude-opus-5",
            "claude-opus-4-8",
            "claude-opus-4-7",
            "claude-opus-4-6",
            "claude-opus-4-5",
            "claude-sonnet-5-5",
            "claude-sonnet-5",
            "claude-sonnet-4-6",
            "claude-sonnet-4-5",
            "claude-haiku-5-5",
            "claude-haiku-5-5-100k",
            "claude-haiku-4-5",
        ],
        "groq" => vec![
            "openai/gpt-oss-120b",
            "openai/gpt-oss-20b",
        ],
        _ => vec![]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_every_listed_model_has_explicit_price() {
        // Models falling through to the `_` arm would silently get a default price
        for provider in PROVIDERS {
            let sentinel = get_model_prices(provider, "__unlisted_model__");
            for model in get_provider_models(provider) {
                let source = include_str!("prices.rs");
                assert!(
                    source.contains(&format!("\"{}\" => (", model)),
                    "{}/{} has no explicit price (would default to {:?})",
                    provider, model, sentinel
                );
            }
        }
    }

    #[test]
    fn test_listed_models_are_unique() {
        for provider in PROVIDERS {
            let models = get_provider_models(provider);
            let mut deduped = models.clone();
            deduped.sort();
            deduped.dedup();
            assert_eq!(models.len(), deduped.len(), "duplicate model in {}", provider);
        }
    }

    #[test]
    fn test_get_model_prices_unknown() {
        let (input_price, output_price) = get_model_prices("unknown", "unknown");
        assert_eq!(input_price, 0.002);
        assert_eq!(output_price, 0.008);
    }

    #[test]
    fn test_get_model_prices_gpt_6_astra() {
        assert_eq!(get_model_prices("openai", "gpt-6-astra"), (10.0, 50.0));
        assert_eq!(get_model_prices("openai", "gpt-6-astra-272k"), (20.0, 75.0));
    }

    #[test]
    fn test_get_model_prices_gpt_5_5() {
        assert_eq!(get_model_prices("openai", "gpt-5.5"), (5.0, 30.0));
        assert_eq!(get_model_prices("openai", "gpt-5.5-272k"), (10.0, 45.0));
    }

    #[test]
    fn test_get_model_prices_gpt_5() {
        assert_eq!(get_model_prices("openai", "gpt-5"), (1.25, 10.0));
        assert_eq!(get_model_prices("openai", "gpt-5-mini"), (0.25, 2.0));
        assert_eq!(get_model_prices("openai", "gpt-5-nano"), (0.05, 0.4));
    }

    #[test]
    fn test_get_model_prices_gpt_4o() {
        assert_eq!(get_model_prices("openai", "gpt-4o"), (2.5, 10.0));
        assert_eq!(get_model_prices("openai", "gpt-4o-mini"), (0.15, 0.6));
    }

    #[test]
    fn test_get_model_prices_gemini_3_1_pro_preview() {
        assert_eq!(get_model_prices("google", "gemini-3.1-pro-preview"), (2.0, 12.0));
        assert_eq!(get_model_prices("google", "gemini-3.1-pro-preview-200k"), (4.0, 18.0));
    }

    #[test]
    fn test_get_model_prices_gemini_2_5_pro() {
        assert_eq!(get_model_prices("google", "gemini-2.5-pro"), (1.25, 10.0));
        assert_eq!(get_model_prices("google", "gemini-2.5-pro-200k"), (2.5, 15.0));
    }

    #[test]
    fn test_get_model_prices_perplexity_sonar() {
        assert_eq!(get_model_prices("perplexity", "sonar"), (1.0, 1.0));
        assert_eq!(get_model_prices("perplexity", "sonar-pro"), (3.0, 15.0));
        assert_eq!(get_model_prices("perplexity", "sonar-reasoning-pro"), (2.0, 8.0));
        assert_eq!(get_model_prices("perplexity", "sonar-deep-research"), (2.0, 8.0));
    }

    #[test]
    fn test_get_model_prices_anthropic_opus() {
        assert_eq!(get_model_prices("anthropic", "claude-opus-5-5"), (4.0, 20.0));
        assert_eq!(get_model_prices("anthropic", "claude-opus-4-6"), (5.0, 25.0));
    }

    #[test]
    fn test_get_model_prices_anthropic_haiku_5_5() {
        assert_eq!(get_model_prices("anthropic", "claude-haiku-5-5"), (0.1, 0.5));
        assert_eq!(get_model_prices("anthropic", "claude-haiku-5-5-100k"), (0.5, 2.5));
    }

    #[test]
    fn test_get_model_prices_groq_gpt_oss() {
        assert_eq!(get_model_prices("groq", "openai/gpt-oss-120b"), (0.15, 0.6));
        assert_eq!(get_model_prices("groq", "openai/gpt-oss-20b"), (0.075, 0.3));
    }
}
