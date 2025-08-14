pub struct CostResponse {
  pub num_tokens: i32,
  pub cost: f32,
}

impl CostResponse {
  pub fn calculate_cost(tokens: i32, requests: i32, pricing: f32) -> Self {
      let num_tokens: i32 = tokens * requests;
      let cost_per_million_requests = pricing / 1000000.0;
      let cost = num_tokens as f32 * cost_per_million_requests;

      CostResponse { num_tokens, cost }
  }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_cost() {
        let cost_response = CostResponse::calculate_cost(100, 10, 0.5);
        assert_eq!(cost_response.num_tokens, 1000);
        assert_eq!(cost_response.cost, 0.5);
    }

    #[test]
    fn test_calculate_cost_high() {
      let cost_response = CostResponse::calculate_cost(100, 10000, 0.5);
      assert_eq!(cost_response.num_tokens, 1000000);
      assert_eq!(cost_response.cost, 0.5);
    }

    #[test]
    fn test_calculate_cost_zero() {
        let cost_response = CostResponse::calculate_cost(0, 0, 0.5);
        assert_eq!(cost_response.num_tokens, 0);
        assert_eq!(cost_response.cost, 0.0);
    }
}