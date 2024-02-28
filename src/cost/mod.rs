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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_cost() {
        let cost_response = CostResponse::calculate_cost(100.0, 10.0, 0.0005);
        assert_eq!(cost_response.num_tokens, 1000.0);
        assert_eq!(cost_response.cost, 0.0005);
    }

    #[test]
    fn test_calculate_cost_zero() {
        let cost_response = CostResponse::calculate_cost(0.0, 0.0, 0.0005);
        assert_eq!(cost_response.num_tokens, 0.0);
        assert_eq!(cost_response.cost, 0.0);
    }
}