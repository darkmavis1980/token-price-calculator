pub struct CostResponse {
  pub num_tokens: f32,
  pub cost: f32,
}

impl CostResponse {
  pub fn calculate_cost(tokens: f32, requests: f32, pricing: f32) -> Self {
      let num_tokens = tokens * requests;
      let cost_per_thousand_requests = pricing / 1000.0;
      let cost = num_tokens * cost_per_thousand_requests;

      CostResponse { num_tokens, cost }
  }
}