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
    // Ensure we are elevated. If not, privileges::ensure_admin will
    // attempt UAC relaunch and exit this process.
    if let Err(e) = ensure_admin() {
        eprintln!("[!] {}", e);
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
        println!("4. Memory Trim");
        println!("5. Security Audit");
        println!("6. Browser Installer");
        println!("7. Exit");
        println!("======================================");

        print!("Select: ");
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;

        let result = match input.trim() {
            "1" => profiles::esports::apply(&config),
            "2" => profiles::streaming::apply(),
            "3" => profiles::extreme::apply(),
            "4" => modules::memory::apply(),
            "5" => modules::security::audit(),
            "6" => tools::browser::browser_menu(),
            "7" => break,
            _ => {
                println!("Invalid option.");
                continue;
            }
        };

        // Don't let a single failure kill the whole process and close the window.
        // Print the error clearly and wait so the user can actually read it.
        if let Err(e) = result {
            eprintln!();
            eprintln!("======================================");
            eprintln!("[!] Profile / action failed:");
            eprintln!("{:#}", e);
            eprintln!("======================================");
            pause();
        }
    }

    Ok(())
}
