use anyhow::{bail, Context, Result};
use std::process::{Command, Output};
use std::time::Duration;

pub fn run(program: &str, args: &[&str]) -> Result<Output> {
    let output = Command::new(program)
        .args(args)
        .output()
        .with_context(|| format!("Failed to start {}", program))?;

    if !output.status.success() {
        bail!(
            "{} exited with {}: {}",
            program,
            output.status,
            String::from_utf8_lossy(&output.stderr).trim()
        );
    }

    Ok(output)
}

/// Run a command and log it. Non-zero exit is reported as a warning
/// instead of aborting the whole profile (many tweaks are best-effort).
pub fn run_logged(program: &str, args: &[&str]) -> Result<()> {
    println!("[CMD] {} {}", program, args.join(" "));

    let output = Command::new(program)
        .args(args)
        .output()
        .with_context(|| format!("Failed to start {}", program))?;

    if !output.stdout.is_empty() {
        let text = String::from_utf8_lossy(&output.stdout).trim().to_string();
        // Filter the literal "null" that PowerShell sometimes prints
        if !text.is_empty() && text.to_lowercase() != "null" {
            println!("{}", text);
        }
    }

    if !output.status.success() {
        let err = String::from_utf8_lossy(&output.stderr).trim().to_string();
        if !err.is_empty() && err.to_lowercase() != "null" {
            eprintln!("[WARN] {} -> {}", program, err);
        } else {
            eprintln!("[WARN] {} exited with {}", program, output.status);
        }
        // Do not bail — continue applying the rest of the profile.
    }

    Ok(())
}

pub fn powershell(script: &str) -> Result<()> {
    run_logged(
        "powershell.exe",
        &[
            "-NoProfile",
            "-ExecutionPolicy",
            "Bypass",
            "-Command",
            script,
        ],
    )
}

pub fn powershell_quiet(script: &str) -> Result<String> {
    let output = Command::new("powershell.exe")
        .args([
            "-NoProfile",
            "-ExecutionPolicy",
            "Bypass",
            "-Command",
            script,
        ])
        .output()
        .with_context(|| "Failed to start powershell.exe")?;

    let stdout = String::from_utf8_lossy(&output.stdout)
        .trim()
        .to_string();

    if !output.status.success() {
        let err = String::from_utf8_lossy(&output.stderr).trim().to_string();
        if !err.is_empty() && err.to_lowercase() != "null" {
            eprintln!("[WARN] powershell: {}", err);
        }
    }

    Ok(stdout)
}

pub fn powercfg(args: &[&str]) -> Result<()> {
    run_logged("powercfg.exe", args)
}

pub fn netsh(args: &[&str]) -> Result<()> {
    run_logged("netsh.exe", args)
}

pub fn bcdedit(args: &[&str]) -> Result<()> {
    run_logged("bcdedit.exe", args)
}

pub fn fsutil(args: &[&str]) -> Result<()> {
    run_logged("fsutil.exe", args)
}

pub fn delay() {
    std::thread::sleep(Duration::from_millis(100));
}
