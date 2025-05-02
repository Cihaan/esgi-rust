use rand::Rng;
use std::io;

fn main() {
    println!("Password Generator");

    // 1. Get password length from user
    println!("Enter the desired password length:");
    let mut length = String::new();
    io::stdin()
        .read_line(&mut length)
        .expect("Failed to read line");
    let length: usize = length.trim().parse().expect("Please enter a valid number");

    // 2. Define character sets
    let letters = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
    let numbers = "0123456789";
    let special_chars = "!@#$%^&*()_+=-`~[]\\{}|;':\",./<>?";

    // 3. Allow user to specify characters to exclude (Bonus)
    println!("Enter characters to exclude (optional, e.g., l10):");
    let mut exclude = String::new();
    io::stdin()
        .read_line(&mut exclude)
        .expect("Failed to read line");
    let exclude: String = exclude.trim().to_string();

    // 4. Generate the password
    let password = generate_password(length, letters, numbers, special_chars, &exclude);

    // 5. Display the generated password
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

    for _i in 0..length {
        let idx = rng.gen_range(0..allowed_chars.len());
        password.push(allowed_chars.chars().nth(idx).unwrap());
    }

    password
}
