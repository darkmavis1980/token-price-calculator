fn main() {
    let mut input = String::new();
    let mut output = String::new();
    let mut num_request = String::new();
    let _price_input_1k: f32 = 0.0005;
    let _price_output_1k: f32 = 0.0015;
    println!("Please enter the amount of average prompt tokens");
    std::io::stdin().read_line(&mut input).unwrap();
    let input_tokens: f32 = input.trim().parse::<f32>().unwrap();

    println!("Please enter the amount of average completition tokens");
    std::io::stdin().read_line(&mut output).unwrap();
    let output_tokens: f32 = output.trim().parse::<f32>().unwrap();

    println!("Please enter the amount of requests");
    std::io::stdin().read_line(&mut num_request).unwrap();
    let requests_num: f32 = num_request.trim().parse::<f32>().unwrap();

    let input_requests: f32 =  input_tokens * requests_num;
    let input_cost: f32 = (input_requests / 1000.0) * _price_input_1k;

    let output_requests: f32 = output_tokens * requests_num;
    let output_cost: f32 = (output_requests / 1000.0) * _price_output_1k;

    println!("The prompt tokens will cost: {}USD, for {} tokens generated", input_cost, input_requests);
    println!("The completition tokens will cost: {}USD, for {} tokens generated", output_cost, output_requests);
}
