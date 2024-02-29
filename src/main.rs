use clap::Parser;
use num_format::{Locale, ToFormattedString};
use inquire::Select;

mod cost; // Import the module from the cost folder, looking for mod.rs file
mod prices; // Import the module from the prices.rs file

use cost::CostResponse;
use prices::get_model_prices; // Exposes the get_model_prices function from the prices module
use prices::MODELS;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    #[arg(short, long, default_value = "", help = "The model to use as reference")]
    model: String,
}

fn main() {
    let args = Args::parse();
    let mut input = String::new();
    let mut output = String::new();
    let mut num_request = String::new();

    let model: String;

    if args.model.is_empty() {
        let answer = Select::new("What model are you using?", MODELS.to_vec()).prompt();
        model = answer.unwrap().to_string();
    } else {
        model = args.model;
    }

    println!("Using model: {}", model.to_string());
    let (_price_input_1k, _price_output_1k) = get_model_prices(&model);

    println!("Please enter the amount of average prompt tokens");
    std::io::stdin().read_line(&mut input).unwrap();
    let input_tokens: i32 = input.trim().parse::<i32>().unwrap();

    println!("Please enter the amount of average completition tokens");
    std::io::stdin().read_line(&mut output).unwrap();
    let output_tokens: i32 = output.trim().parse::<i32>().unwrap();

    println!("Please enter the amount of requests");
    std::io::stdin().read_line(&mut num_request).unwrap();
    let requests_num: i32 = num_request.trim().parse::<i32>().unwrap();

    let input_cost_response = CostResponse::calculate_cost(input_tokens, requests_num, _price_input_1k);
    let output_cost_response = CostResponse::calculate_cost(output_tokens, requests_num, _price_output_1k);

    let input_total_tokens: String = (input_cost_response.num_tokens as i32).to_formatted_string(&Locale::en);
    let output_total_tokens: String = (output_cost_response.num_tokens as i32).to_formatted_string(&Locale::en);
    let total_tokens = (input_cost_response.num_tokens + output_cost_response.num_tokens) as i32;
    let total_cost = (input_cost_response.cost + output_cost_response.cost) as i32;

    println!("The prompt tokens will cost: {} USD, for {} tokens generated", input_cost_response.cost, input_total_tokens);
    println!("The completition tokens will cost: {} USD, for {} tokens generated", output_cost_response.cost, output_total_tokens);
    println!("Total tokens: {}", total_tokens.to_formatted_string(&Locale::en));
    println!("Total cost with OpenAI: {} USD", total_cost.to_formatted_string(&Locale::en));
}
