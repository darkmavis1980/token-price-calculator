pub fn get_model_prices(model: &str) -> (f32, f32) {
  match model {
      "gpt-3.5-turbo" => (0.0005, 0.0015),
      "gpt-4" => (0.03, 0.06),
      "gpt-4-32k" => (0.06, 0.12),
      "gpt-4-turbo" => (0.01, 0.03),
      _ => (0.0005, 0.0015)
  }
}