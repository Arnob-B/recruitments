use argon2::{self, Config};
use std::env;

fn main() {
    // Get password from env or default to "admin"
    let password = env::var("PASSWORD").unwrap_or_else(|_| "admin".to_string());

    // Ideally use a random salt — but for this script, we'll hardcode it or derive it
    let salt = b"randomsalt"; // Replace this with proper salt generation in production

    let config = Config::default();

    let hash = argon2::hash_encoded(password.as_bytes(), salt, &config)
        .expect("Failed to hash password");

    println!("{}", hash);
}
