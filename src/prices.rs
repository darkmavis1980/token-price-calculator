pub fn get_google_model_prices(model: &str) -> (f32, f32) {
    match model {
        "gemini-1.5-pro" => (3.5, 1.05),
        "gemini-1.5-pro-128k" => (0.7, 2.1),
        "gemini-1.5-flash" => (0.075, 0.3),
        "gemini-1.5-flash-128k" => (0.15, 0.6),
        "gemini-2.0-flash" => (0.1, 0.4),
        "gemini-2.0-flash-lite" => (0.075, 0.3),
        "gemini-2.5-flash" => (0.3, 2.5),
        "gemini-2.5-flash-lite" => (0.1, 0.4),
        "gemini-2.5-pro" => (1.25, 10.0),
        "gemini-2.5-pro-200k" => (2.5, 15.0),
        _ => (0.5, 1.5)
    }
}

pub fn get_perplexity_model_prices(model: &str) -> (f32, f32) {
    match model {
        "sonar" => (1.0, 1.0),
        "sonar-pro" => (3.0, 15.0),
        "sonar-reasoning" => (1.0, 5.0),
        "sonar-reasoning-pro" => (2.0, 8.0),
        "sonar-deep-research" => (2.0, 8.0),
        "r1-1776" => (2.0, 8.0),
        _ => (1.0, 1.0)
    }
}

pub fn get_openai_model_prices(model: &str) -> (f32, f32) {
    match model {
        "gpt-5" => (1.25, 10.0),
        "gpt-5-mini" => (0.25, 2.0),
        "gpt-4o" => (5.0, 20.0),
        "gpt-4o-mini" => (0.6, 2.4),
        "gpt-4.1" => (2.0, 8.0),
        "gpt-4.1-mini" => (0.4, 1.6),
        "gpt-4.1-nano" => (0.1, 0.4),
        "o1" => (15.0, 60.0),
        "o1-pro" => (15.0, 60.0),
        "o3" => (2.0, 8.0),
        "o3-pro" => (20.0, 80.0),
        "o3-mini" => (1.1, 4.4),
        "o4-mini" => (1.1, 4.4),
        _ => (2.0, 8.0)
    }
}

pub fn get_anthropic_model_prices(model: &str) -> (f32, f32) {
    match model {
        "claude-3-5-haiku" => (0.8, 4.0),
        "claude-3-7-sonnet" => (3.0, 15.0),
        "claude-4-sonnet" => (3.0, 15.0),
        "claude-4-opus" => (15.0, 75.0),
        "claude-4.1-opus" => (15.0, 75.0),
        _ => (3.0, 15.0)
    }
}
pub fn get_groq_model_prices(model: &str) -> (f32, f32) {
    match model {
        "gemma2-9b-it" => (0.2, 0.2),
        "gemma-7b-it" => (0.07, 0.07),
        "llama-3.1-70b-versatile" => (0.59, 0.79),
        "llama-3.1-8b-instant" => (0.05, 0.08),
        "llama-3.3-70b-versatile" => (0.59, 0.79),
        "llama-4-scout" => (0.11, 0.34),
        "llama-4-maverick" => (0.2, 0.6),
        "llama-guard-4-12b" => (0.2, 0.2),
        _ => (0.05, 0.08)
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
            "gpt-5",
            "gpt-5-mini",
            "gpt-5-nano",
            "gpt-4o",
            "gpt-4o-mini",
            "gpt-4.1",
            "gpt-4.1-mini",
            "gpt-4.1-nano",
            "o1",
            "o1-pro",
            "o3",
            "o3-pro",
            "o3-mini",
            "o4-mini",
        ],
        "google" => vec![
            "gemini-1.5-pro",
            "gemini-1.5-pro-128k",
            "gemini-1.5-flash",
            "gemini-1.5-flash-128k",
            "gemini-2.0-flash",
            "gemini-2.0-flash-lite",
            "gemini-2.0-flash-lite",
            "gemini-2.5-flash",
            "gemini-2.5-flash-lite",
            "gemini-2.5-pro",
            "gemini-2.5-pro-200k",
        ],
        "perplexity" => vec![
            "sonar",
            "sonar-pro",
            "sonar-reasoning",
            "sonar-reasoning-pro",
            "sonar-deep-research",
            "r1-1776",
        ],
        "anthropic" => vec![
            "claude-3-5-haiku",
            "claude-3-7-sonnet",
            "claude-4-sonnet",
            "claude-4-opus",
            "claude-4.1-opus",
        ],
        "groq" => vec![
            "gemma2-9b-it",
            "gemma-7b-it",
            "llama-3.1-70b-versatile",
            "llama-3.1-8b-instant",
            "llama-3.3-70b-versatile",
            "llama-4-scout",
            "llama-4-maverick",
            "llama-guard-4-12b",
        ],
        _ => vec![]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_model_prices_gpt_4o() {
        let (input_price, output_price) = get_model_prices("openai", "gpt-4o");
        assert_eq!(input_price, 5.0);
        assert_eq!(output_price, 20.0);
    }

    #[test]
    fn test_get_model_prices_o3_mini() {
        let (input_price, output_price) = get_model_prices("openai", "o3-mini");
        assert_eq!(input_price, 1.1);
        assert_eq!(output_price, 4.4);
    }

    #[test]
    fn test_get_model_prices_o4_mini() {
        let (input_price, output_price) = get_model_prices("openai", "o4-mini");
        assert_eq!(input_price, 1.1);
        assert_eq!(output_price, 4.4);
    }

    #[test]
    fn test_get_model_prices_gpt_4o_mini() {
        let (input_price, output_price) = get_model_prices("openai", "gpt-4o-mini");
        assert_eq!(input_price, 0.6);
        assert_eq!(output_price, 2.4);
    }

    #[test]
    fn test_get_model_prices_gpt_4() {
        let (input_price, output_price) = get_model_prices("openai", "gpt-4.1");
        assert_eq!(input_price, 2.0);
        assert_eq!(output_price, 8.0);
    }

    #[test]
    fn test_get_model_prices_o1() {
        let (input_price, output_price) = get_model_prices("openai", "o1");
        assert_eq!(input_price, 15.0);
        assert_eq!(output_price, 60.0);
    }

    #[test]
    fn test_get_model_prices_unknown() {
        let (input_price, output_price) = get_model_prices("unknown", "unknown");
        assert_eq!(input_price, 0.002);
        assert_eq!(output_price, 0.008);
    }

    #[test]
    fn test_get_model_prices_gemini_1_5_pro() {
        let (input_price, output_price) = get_model_prices("google", "gemini-1.5-pro");
        assert_eq!(input_price, 3.5);
        assert_eq!(output_price, 1.05);
    }

    #[test]
    fn test_get_model_prices_gemini_1_5_pro_128k() {
        let (input_price, output_price) = get_model_prices("google", "gemini-1.5-pro-128k");
        assert_eq!(input_price, 0.7);
        assert_eq!(output_price, 2.1);
    }

    #[test]
    fn test_get_model_prices_gemini_1_5_flash() {
        let (input_price, output_price) = get_model_prices("google", "gemini-1.5-flash");
        assert_eq!(input_price, 0.075);
        assert_eq!(output_price, 0.3);
    }

    #[test]
    fn test_get_model_prices_gemini_1_5_flash_128k() {
        let (input_price, output_price) = get_model_prices("google", "gemini-1.5-flash-128k");
        assert_eq!(input_price, 0.15);
        assert_eq!(output_price, 0.6);
    }

    #[test]
    fn test_get_model_prices_gemini_2_0_flash() {
        let (input_price, output_price) = get_model_prices("google", "gemini-2.0-flash");
        assert_eq!(input_price, 0.1);
        assert_eq!(output_price, 0.4);
    }

    #[test]
    fn test_get_model_prices_gemini_2_0_flash_lite() {
        let (input_price, output_price) = get_model_prices("google", "gemini-2.0-flash-lite");
        assert_eq!(input_price, 0.075);
        assert_eq!(output_price, 0.3);
    }

    #[test]
    fn test_get_model_prices_perplexity_sonar_reasoning_pro() {
        let (input_price, output_price) = get_model_prices("perplexity", "sonar-reasoning-pro");
        assert_eq!(input_price, 2.0);
        assert_eq!(output_price, 8.0);
    }

    #[test]
    fn test_get_model_prices_perplexity_sonar_reasoning() {
        let (input_price, output_price) = get_model_prices("perplexity", "sonar-reasoning");
        assert_eq!(input_price, 1.0);
        assert_eq!(output_price, 5.0);
    }

    #[test]
    fn test_get_model_prices_perplexity_sonar_pro() {
        let (input_price, output_price) = get_model_prices("perplexity", "sonar-pro");
        assert_eq!(input_price, 3.0);
        assert_eq!(output_price, 15.0);
    }

    #[test]
    fn test_get_model_prices_perplexity_sonar() {
        let (input_price, output_price) = get_model_prices("perplexity", "sonar");
        assert_eq!(input_price, 1.0);
        assert_eq!(output_price, 1.0);
    }

    #[test]
    fn test_get_model_prices_perplexity_sonar_deep_research() {
        let (input_price, output_price) = get_model_prices("perplexity", "sonar-deep-research");
        assert_eq!(input_price, 2.0);
        assert_eq!(output_price, 8.0);
    }

    #[test]
    fn test_get_model_prices_perplexity_r1_1776() {
        let (input_price, output_price) = get_model_prices("perplexity", "r1-1776");
        assert_eq!(input_price, 2.0);
        assert_eq!(output_price, 8.0);
    }

    #[test]
    fn test_get_model_prices_anthropic_claude_3_5_haiku() {
        let (input_price, output_price) = get_model_prices("anthropic", "claude-3-5-haiku");
        assert_eq!(input_price, 0.8);
        assert_eq!(output_price, 4.0);
    }

    #[test]
    fn test_get_model_prices_anthropic_claude_3_7_sonnet() {
        let (input_price, output_price) = get_model_prices("anthropic", "claude-3-7-sonnet");
        assert_eq!(input_price, 3.0);
        assert_eq!(output_price, 15.0);
    }

    #[test]
    fn test_get_model_prices_anthropic_claude_4_sonnet() {
        let (input_price, output_price) = get_model_prices("anthropic", "claude-4-sonnet");
        assert_eq!(input_price, 3.0);
        assert_eq!(output_price, 15.0);
    }

    #[test]
    fn test_get_model_prices_anthropic_claude_4_opus() {
        let (input_price, output_price) = get_model_prices("anthropic", "claude-4-opus");
        assert_eq!(input_price, 15.0);
        assert_eq!(output_price, 75.0);
    }
}
