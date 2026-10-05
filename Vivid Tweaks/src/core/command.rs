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

pub fn run_logged(program: &str, args: &[&str]) -> Result<()> {
    println!("[CMD] {} {}", program, args.join(" "));

    let output = run(program, args)?;

    if !output.stdout.is_empty() {
        println!("{}", String::from_utf8_lossy(&output.stdout).trim());
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
        .output()?;

    if !output.status.success() {
        bail!("{}", String::from_utf8_lossy(&output.stderr));
    }

    Ok(String::from_utf8_lossy(&output.stdout)
        .trim()
        .to_string())
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