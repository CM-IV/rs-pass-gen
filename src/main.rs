use rand::prelude::*;
use rand::distributions::Alphanumeric;
use color_eyre::eyre::Result;
use std::env;

fn main() -> Result<()> {
    
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("Usage: rs-pass-gen <password_length>");
        return Ok(());
    }

    let length: usize = args[1].parse().expect("Please enter a valid number");

    let password: String = thread_rng()
        .sample_iter(&Alphanumeric)
        .map(|c| c as char)
        .take(length)
        .collect();

    println!("Here is your password: {}", password);

    Ok(())
}