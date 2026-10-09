use anyhow::{Context, Result};
use std::path::PathBuf;
use std::process::Command;
use std::env;

use crate::core::registry;
use crate::core::command;

/// Apply the full Quality of Life + Interface pack
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

fn get_wallpaper_path() -> PathBuf {
    // 1. Try next to the executable
    if let Ok(exe) = env::current_exe() {
        if let Some(dir) = exe.parent() {
            let candidate = dir.join("assets").join("image.png");
            if candidate.exists() {
                return candidate;
            }
            let candidate2 = dir.join("image.png");
            if candidate2.exists() {
                return candidate2;
            }
        }
    }

    // 2. Fallback – relative path (for development)
    PathBuf::from("assets/image.png")
}

fn apply_wallpaper() -> Result<()> {
    let wallpaper = get_wallpaper_path();

    if !wallpaper.exists() {
        println!("[QOL] Wallpaper not found at {:?}. Skipping wallpaper.", wallpaper);
        return Ok(());
    }

    println!("[QOL] Setting Astro wallpaper...");

    // Set desktop wallpaper
    let path_str = wallpaper.to_string_lossy();
    let script = format!(
        r#"
Add-Type -TypeDefinition @"
using System;
using System.Runtime.InteropServices;
public class Wallpaper {{
    [DllImport("user32.dll", CharSet = CharSet.Auto)]
    public static extern int SystemParametersInfo(int uAction, int uParam, string lpvParam, int fuWinIni);
}}
"@
[Wallpaper]::SystemParametersInfo(20, 0, "{}", 3)
"#,
        path_str
    );

    let _ = command::powershell(&script);

    // Also set as lock screen (best effort)
    let _ = registry::set_string(
        r"HKCU\Software\Microsoft\Windows\CurrentVersion\ContentDeliveryManager",
        "RotatingLockScreenEnabled",
        "0",
    );

    Ok(())
}

fn disable_annoying_features() -> Result<()> {
    // Accessibility shortcuts
    registry::set_string(r"HKCU\Control Panel\Accessibility\HighContrast", "Flags", "0")?;
    registry::set_string(r"HKCU\Control Panel\Accessibility\Keyboard Response", "Flags", "0")?;
    registry::set_string(r"HKCU\Control Panel\Accessibility\MouseKeys", "Flags", "0")?;
    registry::set_string(r"HKCU\Control Panel\Accessibility\StickyKeys", "Flags", "0")?;
    registry::set_string(r"HKCU\Control Panel\Accessibility\ToggleKeys", "Flags", "0")?;

    // Ease of Access sounds
    registry::set_dword(r"HKCU\Control Panel\Accessibility", "Warning Sounds", 0)?;
    registry::set_dword(r"HKCU\Control Panel\Accessibility", "Sound on Activation", 0)?;

    // Disable AutoRun
    registry::set_dword(
        r"HKCU\Software\Microsoft\Windows\CurrentVersion\Explorer\AutoplayHandlers",
        "DisableAutoplay",
        1,
    )?;

    // Disable Aero Shake
    registry::set_dword(
        r"HKCU\SOFTWARE\Microsoft\Windows\CurrentVersion\Explorer\Advanced",
        "DisallowShaking",
        1,
    )?;

    // Disable menu hover delay
    registry::set_string(r"HKCU\Control Panel\Desktop", "MenuShowDelay", "0")?;

    // Disable low disk space checks
    registry::set_dword(
        r"HKCU\SOFTWARE\Microsoft\Windows\CurrentVersion\Policies\Explorer",
        "NoLowDiskSpaceChecks",
        1,
    )?;

    Ok(())
}

fn configure_explorer() -> Result<()> {
    // Open to This PC
    registry::set_dword(
        r"HKCU\SOFTWARE\Microsoft\Windows\CurrentVersion\Explorer\Advanced",
        "LaunchTo",
        1,
    )?;

    // Show hidden files + extensions
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

    // Compact mode
    registry::set_dword(
        r"HKCU\Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced",
        "UseCompactMode",
        1,
    )?;

    // Hide recent / frequent
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

    // Hide Gallery (best effort via registry)
    registry::set_dword(
        r"HKCU\Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced",
        "ShowGallery",
        0,
    )?;

    // Full context menu
    registry::set_dword(
        r"HKCU\SOFTWARE\Microsoft\Windows\CurrentVersion\Explorer",
        "MultipleInvokePromptMinimum",
        100,
    )?;

    Ok(())
}

fn configure_taskbar_and_start() -> Result<()> {
    // Taskbar left
    registry::set_dword(
        r"HKCU\Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced",
        "TaskbarAl",
        0,
    )?;

    // Hide Task View
    registry::set_dword(
        r"HKCU\SOFTWARE\Microsoft\Windows\CurrentVersion\Explorer\Advanced",
        "ShowTaskViewButton",
        0,
    )?;

    // End Task in taskbar
    registry::set_dword(
        r"HKCU\SOFTWARE\Microsoft\Windows\CurrentVersion\Explorer\Advanced\TaskbarDeveloperSettings",
        "TaskbarEndTask",
        1,
    )?;

    // Disable Chat / Widgets icon
    registry::set_dword(
        r"HKCU\Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced",
        "TaskbarMn",
        0,
    )?;

    // Disable startup delay
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

    registry::set_string(r"HKCU\Control Panel\Desktop", "FontSmoothing", "2")?;
    registry::set_string(r"HKCU\Control Panel\Desktop\WindowMetrics", "MinAnimate", "0")?;

    // Disable some animations
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
    // Disable mouse acceleration
    registry::set_string(r"HKCU\Control Panel\Mouse", "MouseSpeed", "0")?;
    registry::set_string(r"HKCU\Control Panel\Mouse", "MouseThreshold1", "0")?;
    registry::set_string(r"HKCU\Control Panel\Mouse", "MouseThreshold2", "0")?;

    // Faster mouse hover
    registry::set_string(r"HKCU\Control Panel\Desktop", "MouseHoverTime", "20")?;

    Ok(())
}

fn apply_misc_qol() -> Result<()> {
    // Faster shutdown
    registry::set_string(r"HKCU\Control Panel\Desktop", "HungAppTimeout", "2000")?;
    registry::set_string(r"HKCU\Control Panel\Desktop", "WaitToKillAppTimeOut", "2000")?;
    registry::set_string(
        r"HKLM\SYSTEM\CurrentControlSet\Control",
        "WaitToKillServiceTimeout",
        "2000",
    )?;

    // Auto end tasks
    registry::set_string(r"HKCU\Control Panel\Desktop", "AutoEndTasks", "1")?;

    // Disable Windows Feedback
    registry::set_dword(
        r"HKCU\SOFTWARE\Microsoft\Siuf\Rules",
        "NumberOfSIUFInPeriod",
        0,
    )?;

    // Disable Spotlight
    registry::set_dword(
        r"HKCU\SOFTWARE\Policies\Microsoft\Windows\CloudContent",
        "DisableWindowsSpotlightFeatures",
        1,
    )?;

    Ok(())
}
