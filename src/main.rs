/// The line `use std::env;` in Rust is a way to bring the `env` module into scope, allowing you to use
/// functions and types defined in that module without having to fully qualify them with the module name
/// each time.
// use std::env;
use clap::Parser;

struct CostResponse {
    num_tokens: f32,
    cost: f32,
}

impl CostResponse {
    fn calculate_cost(tokens: f32, requests: f32, pricing: f32) -> Self {
        let num_tokens = tokens * requests;
        let cost_per_thousand_requests = pricing / 1000.0;
        let cost = num_tokens * cost_per_thousand_requests;

        CostResponse { num_tokens, cost }
    }
}

fn get_model_prices(model: &str) -> (f32, f32) {
    match model {
        "gpt-3.5-turbo" => (0.0005, 0.0015),
        "gpt-4" => (0.03, 0.06),
        "gpt-4-32k" => (0.06, 0.12),
        "gpt-4-turbo" => (0.01, 0.03),
        _ => (0.0005, 0.0015)
    }
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    #[arg(short, long, default_value = "gpt-3.5-turbo", help = "The model to use as reference")]
    model: String,
}

fn main() {
    let args = Args::parse();

    println!("Using model: {}", args.model);

    let mut input = String::new();
    let mut output = String::new();
    let mut num_request = String::new();

    let (_price_input_1k, _price_output_1k) = get_model_prices(&args.model);

    println!("Please enter the amount of average prompt tokens");
    std::io::stdin().read_line(&mut input).unwrap();
    let input_tokens: f32 = input.trim().parse::<f32>().unwrap();

    println!("Please enter the amount of average completition tokens");
    std::io::stdin().read_line(&mut output).unwrap();
    let output_tokens: f32 = output.trim().parse::<f32>().unwrap();

    println!("Please enter the amount of requests");
    std::io::stdin().read_line(&mut num_request).unwrap();
    let requests_num: f32 = num_request.trim().parse::<f32>().unwrap();

    let input_cost_response = CostResponse::calculate_cost(input_tokens, requests_num, _price_input_1k);
    let output_cost_response = CostResponse::calculate_cost(output_tokens, requests_num, _price_output_1k);

    println!("The prompt tokens will cost: {}USD, for {} tokens generated", input_cost_response.cost, input_cost_response.num_tokens);
    println!("The completition tokens will cost: {}USD, for {} tokens generated", output_cost_response.cost, output_cost_response.num_tokens);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_cost() {
        let cost_response = CostResponse::calculate_cost(1000.0, 10.0, 0.0005);
        assert_eq!(cost_response.num_tokens, 10000.0);
        assert_eq!(cost_response.cost, 0.005);
    }

    #[test]
    fn test_calculate_cost_zero() {
        let cost_response = CostResponse::calculate_cost(0.0, 0.0, 0.0005);
        assert_eq!(cost_response.num_tokens, 0.0);
        assert_eq!(cost_response.cost, 0.0);
    }
}