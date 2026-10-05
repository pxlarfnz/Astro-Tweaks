use anyhow::Result;
use winreg::{enums::*, RegKey};

use crate::core::registry;
use crate::core::command;

/// Latency-focused GPU tweaks (Esports + Extreme)
pub fn apply_latency() -> Result<()> {
    println!("[GPU] Applying latency-focused graphics settings...");

    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);

    // Hardware-accelerated GPU Scheduling
    let (graphics, _) = hklm.create_subkey(
        r"SYSTEM\CurrentControlSet\Control\GraphicsDrivers",
    )?;
    graphics.set_value("HwSchMode", &2u32)?;
    graphics.set_value("DpiMapIommuContiguous", &1u32)?;

    // Disable Game DVR / Background recording (frees GPU resources)
    let (game, _) = hkcu.create_subkey(r"System\GameConfigStore")?;
    game.set_value("GameDVR_Enabled", &0u32)?;

    registry::set_dword(
        r"HKCU\SOFTWARE\Microsoft\Windows\CurrentVersion\GameDVR",
        "AppCaptureEnabled",
        0,
    )?;

    registry::set_dword(
        r"HKLM\SOFTWARE\Policies\Microsoft\Windows\GameDVR",
        "AllowGameDVR",
        0,
    )?;

    // Force MSI mode + high priority on actual GPU devices
    command::powershell(
        r#"
Get-CimInstance Win32_VideoController |
Where-Object { $_.PNPDeviceID -match "PCI\\VEN_" } |
ForEach-Object {
    $base = "HKLM:\SYSTEM\CurrentControlSet\Enum\$($_.PNPDeviceID)\Device Parameters\Interrupt Management"

    New-Item "$base\MessageSignaledInterruptProperties" -Force | Out-Null
    Set-ItemProperty "$base\MessageSignaledInterruptProperties" -Name MSISupported -Value 1 -Type DWord

    New-Item "$base\Affinity Policy" -Force | Out-Null
    Set-ItemProperty "$base\Affinity Policy" -Name DevicePriority -Value 0 -Type DWord
}
"#,
    )?;

    Ok(())
}

/// Quality-focused GPU tweaks (Streaming profile)
pub fn apply_quality() -> Result<()> {
    println!("[GPU] Applying quality-focused graphics settings for streaming...");

    // Keep Hardware Scheduling (still good for consistency)
    registry::set_dword(
        r"HKLM\SYSTEM\CurrentControlSet\Control\GraphicsDrivers",
        "HwSchMode",
        2,
    )?;

    // Disable Game DVR so the GPU can focus on the game + encoder
    registry::set_dword(
        r"HKCU\System\GameConfigStore",
        "GameDVR_Enabled",
        0,
    )?;

    registry::set_dword(
        r"HKCU\SOFTWARE\Microsoft\Windows\CurrentVersion\GameDVR",
        "AppCaptureEnabled",
        0,
    )?;

    // Prefer higher quality texture filtering behaviour where possible
    // (NVIDIA/AMD control panel still has the biggest impact here)

    // Slightly more relaxed preemption for better frame consistency under encoding load
    let _ = registry::set_dword(
        r"HKLM\SYSTEM\CurrentControlSet\Control\GraphicsDrivers\Scheduler",
        "EnablePreemption",
        1,
    );

    // MSI mode is still beneficial
    command::powershell(
        r#"
Get-CimInstance Win32_VideoController |
Where-Object { $_.PNPDeviceID -match "PCI\\VEN_" } |
ForEach-Object {
    $base = "HKLM:\SYSTEM\CurrentControlSet\Enum\$($_.PNPDeviceID)\Device Parameters\Interrupt Management"

    New-Item "$base\MessageSignaledInterruptProperties" -Force | Out-Null
    Set-ItemProperty "$base\MessageSignaledInterruptProperties" -Name MSISupported -Value 1 -Type DWord
}
"#,
    )?;

    Ok(())
}

/// Backward-compatible entry point (calls latency version)
pub fn apply() -> Result<()> {
    apply_latency()
}