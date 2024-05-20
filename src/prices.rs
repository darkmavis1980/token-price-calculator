pub fn get_model_prices(model: &str) -> (f32, f32) {
  match model {
      "gpt-3.5-turbo" => (0.0005, 0.0015),
      "gpt-3.5-turbo-0125" => (0.0005, 0.0015),
      "gpt-3.5-turbo-instruct" => (0.0015, 0.002),
      "gpt-4" => (0.03, 0.06),
      "gpt-4-32k" => (0.06, 0.12),
      "gpt-4-turbo" => (0.01, 0.03),
      "gtp-4o" => (0.0005, 0.0015),
      _ => (0.0005, 0.0015)
  }
}

pub const MODELS: [&str; 7] = [
  "gpt-3.5-turbo",
  "gpt-3.5-turbo-0125",
  "gpt-3.5-turbo-instruct",
  "gpt-4",
  "gpt-4-32k",
  "gpt-4-turbo",
  "gpt-4o",
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_model_prices_gpt_3_5_turbo() {
        let (input_price, output_price) = get_model_prices("gpt-3.5-turbo");
        assert_eq!(input_price, 0.0005);
        assert_eq!(output_price, 0.0015);
    }

    #[test]
    fn test_get_model_prices_gpt_3_5_turbo_0125() {
        let (input_price, output_price) = get_model_prices("gpt-3.5-turbo-0125");
        assert_eq!(input_price, 0.0005);
        assert_eq!(output_price, 0.0015);
    }

    #[test]
    fn test_get_model_prices_gpt_3_5_turbo_instruct() {
        let (input_price, output_price) = get_model_prices("gpt-3.5-turbo-instruct");
        assert_eq!(input_price, 0.0015);
        assert_eq!(output_price, 0.002);
    }

    #[test]
    fn test_get_model_prices_gpt_4o() {
        let (input_price, output_price) = get_model_prices("gpt-4o");
        assert_eq!(input_price, 0.0005);
        assert_eq!(output_price, 0.0015);
    }

    #[test]
    fn test_get_model_prices_gpt_4() {
        let (input_price, output_price) = get_model_prices("gpt-4");
        assert_eq!(input_price, 0.03);
        assert_eq!(output_price, 0.06);
    }

    #[test]
    fn test_get_model_prices_gpt_4_32k() {
        let (input_price, output_price) = get_model_prices("gpt-4-32k");
        assert_eq!(input_price, 0.06);
        assert_eq!(output_price, 0.12);
    }

    #[test]
    fn test_get_model_prices_gpt_4_turbo() {
        let (input_price, output_price) = get_model_prices("gpt-4-turbo");
        assert_eq!(input_price, 0.01);
        assert_eq!(output_price, 0.03);
    }

    #[test]
    fn test_get_model_prices_unknown() {
        let (input_price, output_price) = get_model_prices("unknown");
        assert_eq!(input_price, 0.0005);
        assert_eq!(output_price, 0.0015);
    }
}