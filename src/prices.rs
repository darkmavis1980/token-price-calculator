pub fn get_google_model_prices(model: &str) -> (f32, f32) {
    match model {
        "gemini" => (0.00035, 0.00105),
        "gemini_128k" => (0.0007, 0.0021),
        _ => (0.00035, 0.00105)
    }
}

pub fn get_perplexity_model_prices(model: &str) -> (f32, f32) {
    match model {
        "llama-3.1-sonar-small-128k-chat" => (0.0002, 0.0002),
        "llama-3.1-sonar-large-128k-chat" => (0.001, 0.001),
        _ => (0.0002, 0.0002)
    }
}

pub fn get_openai_model_prices(model: &str) -> (f32, f32) {
    match model {
        "gpt-3.5-turbo" => (0.0005, 0.0015),
        "gpt-3.5-turbo-instruct" => (0.0015, 0.002),
        "gpt-4" => (0.03, 0.06),
        "gpt-4-32k" => (0.06, 0.12),
        "gpt-4-turbo" => (0.01, 0.03),
        "gpt-4o" => (0.005, 0.015),
        "gpt-4o-mini" => (0.00015, 0.0006),
        _ => (0.005, 0.015)
    }
}

pub fn get_model_prices(provider: &str, model: &str) -> (f32, f32) {
    match provider {
        "openai" => get_openai_model_prices(model),
        "google" => get_google_model_prices(model),
        "perplexity" => get_perplexity_model_prices(model),
        _ => (0.005, 0.015)
    }
}

pub fn get_perplexity_online_requests_cost(requests: i32) -> i32 {
    let cost_per_thousand_requests: i32 = 5;
    let cost = (requests as i32 * cost_per_thousand_requests) / 1000;
    cost
}

pub const PROVIDERS: [&str; 3] = ["openai", "google", "perplexity"];

pub fn get_provider_models(provider: &str) -> Vec<&str> {
    match provider {
        "openai" => vec![
            "gpt-3.5-turbo",
            "gpt-3.5-turbo-instruct",
            "gpt-4",
            "gpt-4-32k",
            "gpt-4-turbo",
            "gpt-4o",
            "gpt-4o-mini",
        ],
        "google" => vec!["gemini", "gemini_128k"],
        "perplexity" => vec![
            "llama-3.1-sonar-small-128k-chat",
            "llama-3.1-sonar-large-128k-chat",
            "llama-3.1-sonar-small-128k-online",
            "llama-3.1-sonar-large-128k-online",
            "llama-3.1-sonar-huge-128k-online",
        ],
        _ => vec![]
    }
}

// pub const MODELS: [&str; 8] = [
//     "gemini",
//     "gpt-4o",
//     "gpt-4o-mini",
//     "gpt-3.5-turbo",
//     "gpt-3.5-turbo-instruct",
//     "gpt-4",
//     "gpt-4-32k",
//     "gpt-4-turbo",
// ];

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
        assert_eq!(input_price, 0.005);
        assert_eq!(output_price, 0.015);
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
    fn test_get_model_prices_gemini() {
        let (input_price, output_price) = get_model_prices("google", "gemini");
        assert_eq!(input_price, 0.00035);
        assert_eq!(output_price, 0.00105);
    }

    #[test]
    fn test_get_model_prices_gemini_128k() {
        let (input_price, output_price) = get_model_prices("google", "gemini_128k");
        assert_eq!(input_price, 0.0007);
        assert_eq!(output_price, 0.0021);
    }
}