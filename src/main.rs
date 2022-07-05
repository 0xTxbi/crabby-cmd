use std::io;

fn main() {
    // Prompt for user's input
    println!("Input degree value to convert:");

    // User's input
    let mut user_input = String::new();
    io::stdin().read_line(&mut user_input).unwrap();

    // Parse string input to float
    let parsed_user_input: f32 = user_input.trim().parse().unwrap();

    println!("input: {}", user_input);
    let user_degree = radian_calculator(parsed_user_input);
    println!("{} deg would be: {}rad", parsed_user_input, user_degree);
}

fn radian_calculator(deg_to_convert: f32) -> f32 {
    (deg_to_convert * 3.142) / 180.0
}
