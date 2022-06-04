fn main() {
    println!(
        "Your weight on Mars would be: {}kg",
        mars_weight_calculator(75.0)
    );
}

fn mars_weight_calculator(weight_on_earth: f32) -> f32 {
    (weight_on_earth / 9.81) * 3.711
}
