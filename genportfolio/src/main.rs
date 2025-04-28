use rand::Rng;

fn generate_mock_number() -> String {
    let mut rng = rand::thread_rng();
    let mut number = String::new();
    
    // Generate 6 random digits
    for _ in 0..6 {
        let digit = rng.gen_range(0..10);
        number.push_str(&digit.to_string());
    }
    
    number
}

fn main() {
    println!("Generated mock number: {}", generate_mock_number());
}
