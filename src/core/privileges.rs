use anyhow::{bail, Result};
use std::env;
use std::io::{self, Write};

use windows::core::{w, PCWSTR};
use windows::Win32::Foundation::CloseHandle;
use windows::Win32::Security::{
    GetTokenInformation,
    TokenElevation,
    TOKEN_ELEVATION,
    TOKEN_QUERY,
};
use windows::Win32::System::Threading::{
    GetCurrentProcess,
    OpenProcessToken,
};
use windows::Win32::UI::Shell::ShellExecuteW;
use windows::Win32::UI::WindowsAndMessaging::SW_SHOWNORMAL;

pub fn is_admin() -> Result<bool> {
    unsafe {
        let mut token = Default::default();

        OpenProcessToken(
            GetCurrentProcess(),
            TOKEN_QUERY,
            &mut token,
        )?;

        let mut elevation = TOKEN_ELEVATION::default();
        let mut size = 0u32;

        GetTokenInformation(
            token,
            TokenElevation,
            Some(&mut elevation as *mut _ as *mut _),
            std::mem::size_of::<TOKEN_ELEVATION>() as u32,
            &mut size,
        )?;

        let _ = CloseHandle(token);

        Ok(elevation.TokenIsElevated != 0)
    }
}

pub fn ensure_admin() -> Result<()> {
    if is_admin()? {
        return Ok(());
    }

    // 1. Attempt automatic UAC elevation
    unsafe {
        if let Ok(exe_path) = env::current_exe() {
            if let Some(exe_str) = exe_path.to_str() {
                let wide_path: Vec<u16> = exe_str.encode_utf16().chain(Some(0)).collect();

                let result = ShellExecuteW(
                    None,
                    w!("runas"), // Invokes UAC "Run as Administrator"
                    PCWSTR(wide_path.as_ptr()),
                    None,
                    None,
                    SW_SHOWNORMAL,
                );

                // If user accepts UAC, the elevated process spawns; exit this un-elevated one
                if result.0 as usize > 32 {
                    std::process::exit(0);
                }
            }
        }
    }

    // 2. Fallback: If UAC was rejected or failed, prevent the window from closing instantly
    println!("==================================================");
    eprintln!("[!] ERROR: Administrator privileges are required.");
    eprintln!("[!] Right-click the executable and select 'Run as Administrator'.");
    println!("==================================================");
    print!("\nPress Enter to exit...");
    let _ = io::stdout().flush();

    let mut pause = String::new();
    let _ = io::stdin().read_line(&mut pause);

    bail!("Astro Tweaks must be run as Administrator.");
}
