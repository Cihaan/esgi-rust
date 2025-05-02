use rand::Rng;
use std::io;

fn main() {
    println!("Password Generator");

    println!("Enter the desired password length:");
    let mut length = String::new();
    let length: usize = loop {
        io::stdin()
            .read_line(&mut length)
            .expect("Failed to read line");
        match length.trim().parse() {
            Ok(num) if num > 0 => break num,
            _ => {
                println!("Invalid input. Please enter a positive number for the length:");
                length.clear();
            }
        }
    };

    let letters = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
    let numbers = "0123456789";
    let special_chars = "!@#$%^&*()_+=-`~[]\\{}|;':\",./<>?";

    println!("Enter characters to exclude (optional, e.g., l10):");
    let mut exclude = String::new();
    io::stdin()
        .read_line(&mut exclude)
        .expect("Failed to read line");
    let exclude: String = exclude.trim().to_string();

    let password = generate_password(length, letters, numbers, special_chars, &exclude);

    println!("Generated Password: {}", password);
}

fn generate_password(
    length: usize,
    letters: &str,
    numbers: &str,
    special_chars: &str,
    exclude: &str,
) -> String {
    let mut rng = rand::thread_rng();
    let combined_chars: String = letters.to_string() + numbers + special_chars;
    let mut password = String::new();

    let allowed_chars: String = combined_chars
        .chars()
        .filter(|c| !exclude.contains(*c))
        .collect();

    if allowed_chars.is_empty() {
        println!("Error: No characters available to generate password after exclusions.");
        return String::new();
    }

    for _i in 0..length {
        let idx = rng.gen_range(0..allowed_chars.len());

        password.push(allowed_chars.chars().nth(idx).unwrap());
    }

    password
}
