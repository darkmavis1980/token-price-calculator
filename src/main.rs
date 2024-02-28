/// The line `use std::env;` in Rust is a way to bring the `env` module into scope, allowing you to use
/// functions and types defined in that module without having to fully qualify them with the module name
/// each time.
// use std::env;
use clap::Parser;

mod cost; // Import the module from the cost folder, looking for mod.rs file
mod prices; // Import the module from the prices.rs file

use cost::CostResponse;
use prices::get_model_prices; // Exposes the get_model_prices function from the prices module

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
