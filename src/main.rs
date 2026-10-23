mod config;
mod core;
mod hardware;
mod modules;
mod profiles;
mod tools;

use anyhow::Result;
use std::io::{self, Write};

use config::optimization::OptimizationConfig;
use core::privileges::ensure_admin;

fn pause() {
    print!("\nPress Enter to continue...");
    let _ = io::stdout().flush();
    let mut buf = String::new();
    let _ = io::stdin().read_line(&mut buf);
}

fn main() -> Result<()> {
    if let Err(e) = ensure_admin() {
        eprintln!("[!] {e}");
        pause();
        return Err(e);
    }

    let config = OptimizationConfig::default();

    loop {
        println!();
        println!("======================================");
        println!("         ASTRO TWEAKS 2.0");
        println!("======================================");
        println!("1. Esports");
        println!("2. Streaming");
        println!("3. Extreme");
        println!("4. Exit");
        println!("======================================");

        print!("Select: ");
        io::stdout().flush()?;

        let mut input = String::new();
        if io::stdin().read_line(&mut input)? == 0 {
            println!("Input stream closed. Exiting Astro Tweaks.");
            break;
        }

        let result = match input.trim() {
            "1" => profiles::esports::apply(&config),
            "2" => profiles::streaming::apply(),
            "3" => profiles::extreme::apply(),
            "4" => break,
            _ => {
                println!("Invalid option. Enter 1, 2, 3, or 4.");
                continue;
            }
        };

        if let Err(e) = result {
            eprintln!();
            eprintln!("======================================");
            eprintln!("[!] Profile failed:");
            eprintln!("{e:#}");
            eprintln!("======================================");
            pause();
        }
    }

    Ok(())
}
