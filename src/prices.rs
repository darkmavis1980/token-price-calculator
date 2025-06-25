pub fn get_google_model_prices(model: &str) -> (f32, f32) {
    match model {
        "gemini-1.5-pro" => (0.0035, 0.00105),
        "gemini-1.5-pro-128k" => (0.0007, 0.0021),
        "gemini-1.5-flash" => (0.000075, 0.0003),
        "gemini-1.5-flash-128k" => (0.00015, 0.0006),
        "gemini-2.0-flash" => (0.0001, 0.0004),
        "gemini-2.0-flash-lite" => (0.000075, 0.0003),
        "gemini-2.5-flash" => (0.0003, 0.0025),
        "gemini-2.5-flash-lite" => (0.0001, 0.0004),
        "gemini-2.5-pro" => (0.00125, 0.01),
        "gemini-2.5-pro-200k" => (0.0025, 0.015),
        _ => (0.0005, 0.0015)
    }
}

pub fn get_perplexity_model_prices(model: &str) -> (f32, f32) {
    match model {
        "sonar" => (0.001, 0.001),
        "sonar-pro" => (0.003, 0.015),
        "sonar-reasoning" => (0.001, 0.005),
        "sonar-reasoning-pro" => (0.002, 0.008),
        "sonar-deep-research" => (0.002, 0.008),
        "r1-1776" => (0.002, 0.008),
        _ => (0.001, 0.001)
    }
}

pub fn get_openai_model_prices(model: &str) -> (f32, f32) {
    match model {
        "gpt-4o" => (0.0025, 0.01),
        "gpt-4o-mini" => (0.00015, 0.0006),
        "gpt-4.1" => (0.002, 0.008),
        "gpt-4.1-mini" => (0.0004, 0.0016),
        "gpt-4.1-nano" => (0.0001, 0.0004),
        "o1" => (0.015, 0.06),
        "o1-pro" => (0.15, 0.6),
        "o3" => (0.002, 0.008),
        "o3-pro" => (0.02, 0.08),
        "o3-mini" => (0.0011, 0.0044),
        "o4-mini" => (0.0011, 0.0044),
        _ => (0.002, 0.008)
    }
}

pub fn get_groq_model_prices(model: &str) -> (f32, f32) {
    match model {
        "gemma2-9b-it" => (0.0002, 0.0002),
        "gemma-7b-it" => (0.00007, 0.00007),
        "llama-3.1-70b-versatile" => (0.00059, 0.00079),
        "llama-3.1-8b-instant" => (0.00005, 0.00008),
        "llama-3.3-70b-versatile" => (0.00059, 0.00079),
        "llama-4-scout" => (0.00011, 0.00034),
        "llama-4-maverick" => (0.0002, 0.0006),
        "llama-guard-4-12b" => (0.0002, 0.0002),
        _ => (0.00005, 0.00008)
    }
}

pub fn get_model_prices(provider: &str, model: &str) -> (f32, f32) {
    match provider {
        "openai" => get_openai_model_prices(model),
        "google" => get_google_model_prices(model),
        "perplexity" => get_perplexity_model_prices(model),
        "groq" => get_groq_model_prices(model),
        _ => (0.005, 0.015)
    }
}

pub fn get_perplexity_online_requests_cost(requests: i32) -> i32 {
    let cost_per_thousand_requests: i32 = 5;
    let cost = (requests as i32 * cost_per_thousand_requests) / 1000;
    cost
}

pub const PROVIDERS: [&str; 4] = ["openai", "google", "perplexity", "groq"];

pub fn get_provider_models(provider: &str) -> Vec<&str> {
    match provider {
        "openai" => vec![
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
    fn test_get_model_prices_gpt_3_5_turbo() {
        let (input_price, output_price) = get_model_prices("openai", "gpt-3.5-turbo");
        assert_eq!(input_price, 0.0005);
        assert_eq!(output_price, 0.0015);
    }

    #[test]
    fn test_get_model_prices_gpt_3_5_turbo_instruct() {
        let (input_price, output_price) = get_model_prices("openai", "gpt-3.5-turbo-instruct");
        assert_eq!(input_price, 0.0015);
        assert_eq!(output_price, 0.002);
    }

    #[test]
    fn test_get_model_prices_gpt_4o() {
        let (input_price, output_price) = get_model_prices("openai", "gpt-4o");
        assert_eq!(input_price, 0.0025);
        assert_eq!(output_price, 0.01);
    }

    #[test]
    fn test_get_model_prices_o1_mini() {
        let (input_price, output_price) = get_model_prices("openai", "o1-mini");
        assert_eq!(input_price, 0.0011);
        assert_eq!(output_price, 0.0044);
    }

    #[test]
    fn test_get_model_prices_o3_mini() {
        let (input_price, output_price) = get_model_prices("openai", "o3-mini");
        assert_eq!(input_price, 0.0011);
        assert_eq!(output_price, 0.0044);
    }

    #[test]
    fn test_get_model_prices_gpt_4o_mini() {
        let (input_price, output_price) = get_model_prices("openai", "gpt-4o-mini");
        assert_eq!(input_price, 0.00015);
        assert_eq!(output_price, 0.0006);
    }

    #[test]
    fn test_get_model_prices_gpt_4() {
        let (input_price, output_price) = get_model_prices("openai", "gpt-4");
        assert_eq!(input_price, 0.03);
        assert_eq!(output_price, 0.06);
    }

    #[test]
    fn test_get_model_prices_o1() {
        let (input_price, output_price) = get_model_prices("openai", "o1");
        assert_eq!(input_price, 0.015);
        assert_eq!(output_price, 0.06);
    }

    #[test]
    fn test_get_model_prices_gpt_4_32k() {
        let (input_price, output_price) = get_model_prices("openai", "gpt-4-32k");
        assert_eq!(input_price, 0.06);
        assert_eq!(output_price, 0.12);
    }

    #[test]
    fn test_get_model_prices_gpt_4_turbo() {
        let (input_price, output_price) = get_model_prices("openai", "gpt-4-turbo");
        assert_eq!(input_price, 0.01);
        assert_eq!(output_price, 0.03);
    }

    #[test]
    fn test_get_model_prices_unknown() {
        let (input_price, output_price) = get_model_prices("unknown", "unknown");
        assert_eq!(input_price, 0.005);
        assert_eq!(output_price, 0.015);
    }

    #[test]
    fn test_get_model_prices_gemini_1_pro() {
        let (input_price, output_price) = get_model_prices("google", "gemini-1-pro");
        assert_eq!(input_price, 0.0005);
        assert_eq!(output_price, 0.0015);
    }

    #[test]
    fn test_get_model_prices_gemini_1_5_pro() {
        let (input_price, output_price) = get_model_prices("google", "gemini-1.5-pro");
        assert_eq!(input_price, 0.0035);
        assert_eq!(output_price, 0.00105);
    }

    #[test]
    fn test_get_model_prices_gemini_1_5_pro_128k() {
        let (input_price, output_price) = get_model_prices("google", "gemini-1.5-pro-128k");
        assert_eq!(input_price, 0.0007);
        assert_eq!(output_price, 0.0021);
    }

    #[test]
    fn test_get_model_prices_gemini_1_5_flash() {
        let (input_price, output_price) = get_model_prices("google", "gemini-1.5-flash");
        assert_eq!(input_price, 0.000075);
        assert_eq!(output_price, 0.0003);
    }

    #[test]
    fn test_get_model_prices_gemini_1_5_flash_128k() {
        let (input_price, output_price) = get_model_prices("google", "gemini-1.5-flash-128k");
        assert_eq!(input_price, 0.00015);
        assert_eq!(output_price, 0.0006);
    }

    #[test]
    fn test_get_model_prices_gemini_2_0_flash() {
        let (input_price, output_price) = get_model_prices("google", "gemini-2.0-flash");
        assert_eq!(input_price, 0.0001);
        assert_eq!(output_price, 0.0004);
    }

    #[test]
    fn test_get_model_prices_gemini_2_0_flash_lite_preview_02_05() {
        let (input_price, output_price) = get_model_prices("google", "gemini-2.0-flash-lite-preview-02-05");
        assert_eq!(input_price, 0.000075);
        assert_eq!(output_price, 0.0003);
    }

    #[test]
    fn test_get_model_prices_perplexity_sonar_reasoning_pro() {
        let (input_price, output_price) = get_model_prices("perplexity", "sonar-reasoning-pro");
        assert_eq!(input_price, 0.002);
        assert_eq!(output_price, 0.008);
    }

    #[test]
    fn test_get_model_prices_perplexity_sonar_reasoning() {
        let (input_price, output_price) = get_model_prices("perplexity", "sonar-reasoning");
        assert_eq!(input_price, 0.001);
        assert_eq!(output_price, 0.005);
    }

    #[test]
    fn test_get_model_prices_perplexity_sonar_pro() {
        let (input_price, output_price) = get_model_prices("perplexity", "sonar-pro");
        assert_eq!(input_price, 0.003);
        assert_eq!(output_price, 0.015);
    }

    #[test]
    fn test_get_model_prices_perplexity_sonar() {
        let (input_price, output_price) = get_model_prices("perplexity", "sonar");
        assert_eq!(input_price, 0.001);
        assert_eq!(output_price, 0.001);
    }

    #[test]
    fn test_get_model_prices_perplexity_sonar_deep_research() {
        let (input_price, output_price) = get_model_prices("perplexity", "sonar-deep-research");
        assert_eq!(input_price, 0.002);
        assert_eq!(output_price, 0.008);
    }

    #[test]
    fn test_get_model_prices_perplexity_r1_1776() {
        let (input_price, output_price) = get_model_prices("perplexity", "r1-1776");
        assert_eq!(input_price, 0.002);
        assert_eq!(output_price, 0.008);
    }
    
    
}
