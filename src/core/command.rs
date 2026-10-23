use anyhow::{anyhow, bail, Context, Result};
use std::{
    io::Read,
    process::{Command, Output, Stdio},
    thread,
    time::Duration,
};
use wait_timeout::ChildExt;

const DEFAULT_COMMAND_TIMEOUT: Duration = Duration::from_secs(120);

/// Run a child process with a finite timeout while draining stdout and stderr
/// concurrently so a full pipe cannot deadlock the child.
pub fn run_with_timeout(
    program: &str,
    args: &[&str],
    timeout: Duration,
) -> Result<Output> {
    let mut child = Command::new(program)
        .args(args)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .with_context(|| format!("Failed to start {program}"))?;

    let mut stdout = child.stdout.take().context("Could not capture command stdout")?;
    let mut stderr = child.stderr.take().context("Could not capture command stderr")?;

    let stdout_reader = thread::spawn(move || {
        let mut bytes = Vec::new();
        stdout.read_to_end(&mut bytes).map(|_| bytes)
    });
    let stderr_reader = thread::spawn(move || {
        let mut bytes = Vec::new();
        stderr.read_to_end(&mut bytes).map(|_| bytes)
    });

    let status = match child.wait_timeout(timeout) {
        Ok(Some(status)) => status,
        Ok(None) => {
            let _ = child.kill();
            let _ = child.wait();
            // Do not block the main thread indefinitely if a descendant process
            // inherited a pipe handle and kept it open after the timed-out child.
            drop(stdout_reader);
            drop(stderr_reader);
            bail!(
                "{program} exceeded the {} second timeout and was terminated",
                timeout.as_secs()
            );
        }
        Err(error) => {
            let _ = child.kill();
            let _ = child.wait();
            drop(stdout_reader);
            drop(stderr_reader);
            return Err(error).with_context(|| format!("Failed while waiting for {program}"));
        }
    };

    let stdout = stdout_reader
        .join()
        .map_err(|_| anyhow!("stdout reader for {program} panicked"))?
        .with_context(|| format!("Failed to read stdout from {program}"))?;
    let stderr = stderr_reader
        .join()
        .map_err(|_| anyhow!("stderr reader for {program} panicked"))?
        .with_context(|| format!("Failed to read stderr from {program}"))?;

    Ok(Output { status, stdout, stderr })
}

pub fn run(program: &str, args: &[&str]) -> Result<Output> {
    let output = run_with_timeout(program, args, DEFAULT_COMMAND_TIMEOUT)?;
    if !output.status.success() {
        bail!(
            "{program} exited with {}: {}",
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

    let output = run_with_timeout(program, args, DEFAULT_COMMAND_TIMEOUT)
        .with_context(|| format!("Command failed or timed out: {program}"))?;

    if !output.stdout.is_empty() {
        let text = String::from_utf8_lossy(&output.stdout).trim().to_string();
        if !text.is_empty() && text.to_lowercase() != "null" {
            println!("{text}");
        }
    }

    if !output.status.success() {
        let err = String::from_utf8_lossy(&output.stderr).trim().to_string();
        if !err.is_empty() && err.to_lowercase() != "null" {
            eprintln!("[WARN] {} -> {}", program, err);
        } else {
            eprintln!("[WARN] {} exited with {}", program, output.status);
        }
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
    let output = run_with_timeout(
        "powershell.exe",
        &[
            "-NoProfile",
            "-ExecutionPolicy",
            "Bypass",
            "-Command",
            script,
        ],
        DEFAULT_COMMAND_TIMEOUT,
    )
    .context("Failed to run PowerShell or the command timed out")?;

    if !output.status.success() {
        let err = String::from_utf8_lossy(&output.stderr).trim().to_string();
        if !err.is_empty() && err.to_lowercase() != "null" {
            eprintln!("[WARN] powershell: {err}");
        }
    }

    Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
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
    thread::sleep(Duration::from_millis(100));
}
