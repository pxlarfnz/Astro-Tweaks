
use anyhow::{Context, Result};
use std::env;
use std::path::PathBuf;

use crate::core::command;
use crate::core::registry;

/// Apply the full Quality of Life + Interface pack.
pub fn apply() -> Result<()> {
    println!("[QOL] Applying Astro Tweaks Quality of Life pack...");

    apply_wallpaper()?;
    disable_annoying_features()?;
    configure_explorer()?;
    configure_taskbar_and_start()?;
    configure_visual_effects()?;
    disable_copilot_and_chat()?;
    apply_input_and_accessibility()?;
    apply_misc_qol()?;

    println!("[QOL] Quality of Life pack applied.");
    Ok(())
}

/// Locate astro_background.jpg.
fn get_wallpaper_path() -> Option<PathBuf> {
    // First: assets folder beside the executable.
    if let Ok(exe) = env::current_exe() {
        if let Some(dir) = exe.parent() {
            let candidate = dir.join("assets").join("astro_background.jpg");
            if candidate.is_file() {
                return Some(candidate);
            }

            // Second: beside the executable.
            let candidate = dir.join("astro_background.jpg");
            if candidate.is_file() {
                return Some(candidate);
            }
        }
    }

    // Development fallbacks.
    let candidate = PathBuf::from("assets/astro_background.jpg");
    if candidate.is_file() {
        return Some(candidate);
    }

    let candidate = PathBuf::from("astro_background.jpg");
    if candidate.is_file() {
        return Some(candidate);
    }

    None
}

/// Apply the wallpaper and configure the current user's wallpaper policy.
fn apply_wallpaper() -> Result<()> {
    let source = match get_wallpaper_path() {
        Some(path) => path,
        None => {
            println!(
                "[QOL] astro_background.jpg was not found. \
                 Wallpaper settings were not changed."
            );
            return Ok(());
        }
    };

    println!("[QOL] Applying Astro wallpaper and policy...");

    // Use a persistent location so the wallpaper doesn't depend
    // on the original executable or working directory.
    let program_data = env::var_os("PROGRAMDATA")
        .map(PathBuf::from)
        .context("Could not locate the ProgramData directory")?;

    let wallpaper_dir = program_data.join("AstroTweaks");

    std::fs::create_dir_all(&wallpaper_dir)
        .context("Could not create the AstroTweaks wallpaper directory")?;

    let wallpaper = wallpaper_dir.join("astro_background.jpg");

    // Copy if missing, or if the source file size has changed.
    let should_copy = !wallpaper.is_file()
        || std::fs::metadata(&source)?.len()
            != std::fs::metadata(&wallpaper)
                .map(|metadata| metadata.len())
                .unwrap_or(0);

    if should_copy {
        std::fs::copy(&source, &wallpaper)
            .context("Could not copy astro_background.jpg to ProgramData")?;
    }

    let wallpaper_path = wallpaper.to_string_lossy().into_owned();

    // Windows policy registry paths for the current user.
    let system_policy =
        r"HKCU\Software\Microsoft\Windows\CurrentVersion\Policies\System";

    let desktop_policy =
        r"HKCU\Software\Microsoft\Windows\CurrentVersion\Policies\ActiveDesktop";

    // Set the wallpaper path enforced by Windows policy.
    registry::set_string(
        system_policy,
        "Wallpaper",
        &wallpaper_path,
    )?;

    // Wallpaper style: 10 = Fill.
    registry::set_string(
        system_policy,
        "WallpaperStyle",
        "10",
    )?;

    // Enable "Prevent changing desktop background".
    registry::set_dword(
        desktop_policy,
        "NoChangingWallPaper",
        1,
    )?;

    // Set the wallpaper immediately for the current session.
    // Escape single quotes before inserting the path into PowerShell.
    let escaped_path = wallpaper_path.replace('\'', "''");

    let script = format!(
        r#"
$ErrorActionPreference = 'Stop'

Add-Type -TypeDefinition @"
using System;
using System.Runtime.InteropServices;

public class AstroWallpaper {{
    [DllImport("user32.dll", CharSet = CharSet.Auto, SetLastError = true)]
    public static extern bool SystemParametersInfo(
        int action,
        int param,
        string path,
        int flags
    );
}}
"@

$result = [AstroWallpaper]::SystemParametersInfo(
    20, 0, '{}', 3
)

if (-not $result) {{
    throw "SystemParametersInfo failed to apply the wallpaper."
}}
"#,
        escaped_path
    );

    command::powershell(&script)
        .context("Failed to apply the desktop wallpaper")?;

    // Preserve the original lock-screen setting.
    registry::set_string(
        r"HKCU\Software\Microsoft\Windows\CurrentVersion\ContentDeliveryManager",
        "RotatingLockScreenEnabled",
        "0",
    )?;

    println!("[QOL] Wallpaper applied: {}", wallpaper_path);
    println!("[QOL] Wallpaper-change policy enabled for the current user.");

    Ok(())
}

fn disable_annoying_features() -> Result<()> {
    // Accessibility shortcuts.
    registry::set_string(
        r"HKCU\Control Panel\Accessibility\HighContrast",
        "Flags",
        "0",
    )?;
    registry::set_string(
        r"HKCU\Control Panel\Accessibility\Keyboard Response",
        "Flags",
        "0",
    )?;
    registry::set_string(
        r"HKCU\Control Panel\Accessibility\MouseKeys",
        "Flags",
        "0",
    )?;
    registry::set_string(
        r"HKCU\Control Panel\Accessibility\StickyKeys",
        "Flags",
        "0",
    )?;
    registry::set_string(
        r"HKCU\Control Panel\Accessibility\ToggleKeys",
        "Flags",
        "0",
    )?;

    // Ease of Access sounds.
    registry::set_dword(
        r"HKCU\Control Panel\Accessibility",
        "Warning Sounds",
        0,
    )?;
    registry::set_dword(
        r"HKCU\Control Panel\Accessibility",
        "Sound on Activation",
        0,
    )?;

    // Disable AutoRun.
    registry::set_dword(
        r"HKCU\Software\Microsoft\Windows\CurrentVersion\Explorer\AutoplayHandlers",
        "DisableAutoplay",
        1,
    )?;

    // Disable Aero Shake.
    registry::set_dword(
        r"HKCU\SOFTWARE\Microsoft\Windows\CurrentVersion\Explorer\Advanced",
        "DisallowShaking",
        1,
    )?;

    // Disable menu hover delay.
    registry::set_string(
        r"HKCU\Control Panel\Desktop",
        "MenuShowDelay",
        "0",
    )?;

    // Disable low disk space checks.
    registry::set_dword(
        r"HKCU\SOFTWARE\Microsoft\Windows\CurrentVersion\Policies\Explorer",
        "NoLowDiskSpaceChecks",
        1,
    )?;

    Ok(())
}

fn configure_explorer() -> Result<()> {
    // Open to This PC.
    registry::set_dword(
        r"HKCU\SOFTWARE\Microsoft\Windows\CurrentVersion\Explorer\Advanced",
        "LaunchTo",
        1,
    )?;

    // Show hidden files and extensions.
    registry::set_dword(
        r"HKCU\SOFTWARE\Microsoft\Windows\CurrentVersion\Explorer\Advanced",
        "Hidden",
        1,
    )?;
    registry::set_dword(
        r"HKCU\SOFTWARE\Microsoft\Windows\CurrentVersion\Explorer\Advanced",
        "HideFileExt",
        0,
    )?;

    // Compact mode.
    registry::set_dword(
        r"HKCU\Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced",
        "UseCompactMode",
        1,
    )?;

    // Hide recent and frequent items.
    registry::set_dword(
        r"HKCU\SOFTWARE\Microsoft\Windows\CurrentVersion\Explorer",
        "ShowRecent",
        0,
    )?;
    registry::set_dword(
        r"HKCU\SOFTWARE\Microsoft\Windows\CurrentVersion\Explorer",
        "ShowFrequent",
        0,
    )?;

    // Hide Gallery (best effort via registry).
    registry::set_dword(
        r"HKCU\Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced",
        "ShowGallery",
        0,
    )?;

    // Context menu setting from the original QOL pack.
    registry::set_dword(
        r"HKCU\SOFTWARE\Microsoft\Windows\CurrentVersion\Explorer",
        "MultipleInvokePromptMinimum",
        100,
    )?;

    Ok(())
}

fn configure_taskbar_and_start() -> Result<()> {
    // Align taskbar to the left.
    registry::set_dword(
        r"HKCU\Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced",
        "TaskbarAl",
        0,
    )?;

    // Hide Task View.
    registry::set_dword(
        r"HKCU\SOFTWARE\Microsoft\Windows\CurrentVersion\Explorer\Advanced",
        "ShowTaskViewButton",
        0,
    )?;

    // Enable End Task in the taskbar.
    registry::set_dword(
        r"HKCU\SOFTWARE\Microsoft\Windows\CurrentVersion\Explorer\Advanced\TaskbarDeveloperSettings",
        "TaskbarEndTask",
        1,
    )?;

    // Disable Chat taskbar icon.
    registry::set_dword(
        r"HKCU\Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced",
        "TaskbarMn",
        0,
    )?;

    // Disable startup delay.
    registry::set_dword(
        r"HKCU\SOFTWARE\Microsoft\Windows\CurrentVersion\Explorer\Serialize",
        "StartupDelayInMSec",
        0,
    )?;

    Ok(())
}

fn configure_visual_effects() -> Result<()> {
    registry::set_dword(
        r"HKCU\SOFTWARE\Microsoft\Windows\CurrentVersion\Explorer\VisualEffects",
        "VisualFXSetting",
        3,
    )?;

    registry::set_string(
        r"HKCU\Control Panel\Desktop",
        "FontSmoothing",
        "2",
    )?;

    registry::set_string(
        r"HKCU\Control Panel\Desktop\WindowMetrics",
        "MinAnimate",
        "0",
    )?;

    // Disable some animations.
    registry::set_dword(
        r"HKCU\SOFTWARE\Microsoft\Windows\CurrentVersion\Explorer\Advanced",
        "TaskbarAnimations",
        0,
    )?;

    Ok(())
}

fn disable_copilot_and_chat() -> Result<()> {
    registry::set_dword(
        r"HKCU\Software\Policies\Microsoft\Windows\WindowsCopilot",
        "TurnOffWindowsCopilot",
        1,
    )?;

    registry::set_dword(
        r"HKLM\SOFTWARE\Policies\Microsoft\Windows\Windows Chat",
        "ChatIcon",
        3,
    )?;

    Ok(())
}

fn apply_input_and_accessibility() -> Result<()> {
    // Disable mouse acceleration.
    registry::set_string(
        r"HKCU\Control Panel\Mouse",
        "MouseSpeed",
        "0",
    )?;
    registry::set_string(
        r"HKCU\Control Panel\Mouse",
        "MouseThreshold1",
        "0",
    )?;
    registry::set_string(
        r"HKCU\Control Panel\Mouse",
        "MouseThreshold2",
        "0",
    )?;

    // Faster mouse hover.
    registry::set_string(
        r"HKCU\Control Panel\Desktop",
        "MouseHoverTime",
        "20",
    )?;

    Ok(())
}

fn apply_misc_qol() -> Result<()> {
    // Faster shutdown.
    registry::set_string(
        r"HKCU\Control Panel\Desktop",
        "HungAppTimeout",
        "2000",
    )?;
    registry::set_string(
        r"HKCU\Control Panel\Desktop",
        "WaitToKillAppTimeOut",
        "2000",
    )?;
    registry::set_string(
        r"HKLM\SYSTEM\CurrentControlSet\Control",
        "WaitToKillServiceTimeout",
        "2000",
    )?;

    // Auto-end tasks.
    registry::set_string(
        r"HKCU\Control Panel\Desktop",
        "AutoEndTasks",
        "1",
    )?;

    // Disable Windows Feedback.
    registry::set_dword(
        r"HKCU\SOFTWARE\Microsoft\Siuf\Rules",
        "NumberOfSIUFInPeriod",
        0,
    )?;

    // Disable Spotlight features.
    registry::set_dword(
        r"HKCU\SOFTWARE\Policies\Microsoft\Windows\CloudContent",
        "DisableWindowsSpotlightFeatures",
        1,
    )?;

    Ok(())
}
