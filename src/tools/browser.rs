use anyhow::{bail, Result};
use std::io::{self, Write};
use std::process::Command;

#[derive(Debug, Clone, Copy)]
pub enum Browser {
    Brave,
    LibreWolf,
    Waterfox,
}

impl Browser {
    pub fn name(self) -> &'static str {
        match self {
            Self::Brave => "Brave",
            Self::LibreWolf => "LibreWolf",
            Self::Waterfox => "Waterfox",
        }
    }

    pub fn winget_id(self) -> &'static str {
        match self {
            Self::Brave => "Brave.Brave",
            Self::LibreWolf => "LibreWolf.LibreWolf",
            Self::Waterfox => "Waterfox.Waterfox",
        }
    }
}

pub fn install_browser(browser: Browser) -> Result<()> {
    println!();
    println!("Installing {}...", browser.name());

    let status = Command::new("winget")
        .args([
            "install",
            "--id",
            browser.winget_id(),
            "--exact",
            "--accept-package-agreements",
            "--accept-source-agreements",
        ])
        .status()?;

    if !status.success() {
        bail!(
            "winget failed while installing {}. Exit code: {:?}",
            browser.name(),
            status.code()
        );
    }

    println!("{} installation completed.", browser.name());

    Ok(())
}

pub fn browser_menu() -> Result<()> {
    loop {
        println!();
        println!("================================");
        println!("       VIVID BROWSER TOOL");
        println!("================================");
        println!("1. Install Brave");
        println!("2. Install LibreWolf");
        println!("3. Install Waterfox");
        println!("0. Back");
        println!();

        print!("Select an option: ");
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;

        let browser = match input.trim() {
            "1" => Browser::Brave,
            "2" => Browser::LibreWolf,
            "3" => Browser::Waterfox,
            "0" => return Ok(()),
            _ => {
                println!("Invalid selection.");
                continue;
            }
        };

        if let Err(error) = install_browser(browser) {
            eprintln!("Browser installation failed: {error:#}");
        }

        return Ok(());
    }
}