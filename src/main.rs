use rand::rng;
use rand::prelude::IndexedRandom;
use rand::prelude::SliceRandom;
mod args;
use args::Args;
use colored::*;

use clap::Parser;
fn main() {
    let args = Args::parse();
    let mut charset = args.characters.clone();
    if let Some(extra) = args.special_chars {
        charset.push_str(&extra);
    }

    let char_pool: Vec<char> = charset.chars().collect();
    let total_needed = args.blocks * args.block_size;

    if char_pool.is_empty() {
        println!("{}", "[Error] Passgen requires at least one character in the character set.".red());
        std::process::exit(1);
    }
    if total_needed == 0 {
        println!("{}", "[Warning] Passgen generated an empty password.".yellow());
        std::process::exit(2);
    }

    let digits:  Vec<char> = char_pool.iter().filter(|c| c.is_ascii_digit()).cloned().collect();
    let upper:   Vec<char> = char_pool.iter().filter(|c| c.is_ascii_uppercase()).cloned().collect();
    let lower:   Vec<char> = char_pool.iter().filter(|c| c.is_ascii_lowercase()).cloned().collect();
    let symbols: Vec<char> = char_pool.iter().filter(|c| !c.is_ascii_alphanumeric()).cloned().collect();
    
    let mut rng = rng(); 

    let mut results = Vec::new();
    for category in [&digits, &upper, &lower, &symbols] {
        if let Some(c) = category.choose(&mut rng) {
            results.push(*c);
        }
    }

    if results.len() > total_needed {
        eprintln!("{}", "Error: Password too short to guarantee all character types found in the pool.".red());
        return;
    }

    while results.len() < total_needed {
        results.push(*char_pool.choose(&mut rng).unwrap());
    }
    results.shuffle(&mut rng);

    let password = results
        .chunks(args.block_size)
        .map(|chunk| chunk.iter().collect::<String>())
        .collect::<Vec<String>>();

    if args.colorize {
        print_colorized(&password, &args.delimiter);
    } else {
        println!("{}", password.join(&args.delimiter));
    }
    if args.extra_info { 
        let pool_size = char_pool.len() as f64;
        let total_length = (args.blocks * args.block_size) as f64;

        let total_combinations = pool_size.powf(total_length);
        let entropy = total_length * pool_size.log2();
        println!();
        println!("Entropy.............. {} bits", format!("{:.2}", entropy).bold()); 
        println!("Total Combinations... {} ", format!("{:.2e}", total_combinations).bold()); 
    }
}

fn print_colorized(password: &Vec<String>, delimiter: &str) {
        let max_i = password.len() - 1;
        for (i, block) in password.iter().enumerate() {
            let colored_block = if i % 2 == 0 {
                block.yellow().to_string()
            } else {
                block.green().to_string()
            };
            print!("{}", colored_block);
            if i < max_i {
                print!("{}", delimiter.dimmed());
            }
            else {
                println!();
            }
        }
}
