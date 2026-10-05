use anyhow::Result;
use std::process::Command;

use crate::core::registry;

pub fn optimize_ntfs() -> Result<()> {
    println!("Optimizing NTFS...");

    let _ = Command::new("fsutil")
        .args([
            "behavior",
            "set",
            "disablelastaccess",
            "1",
        ])
        .status();

    let _ = Command::new("fsutil")
        .args([
            "behavior",
            "set",
            "disable8dot3",
            "1",
        ])
        .status();

    Ok(())
}

pub fn disable_background_apps() -> Result<()> {
    println!("Disabling background applications...");

    registry::set_dword(
        r"HKCU\SOFTWARE\Microsoft\Windows\CurrentVersion\BackgroundAccessApplications",
        "GlobalUserDisabled",
        1,
    )?;

    registry::set_dword(
        r"HKCU\SOFTWARE\Microsoft\Windows\CurrentVersion\Search",
        "BackgroundAppGlobalToggle",
        0,
    )?;

    Ok(())
}

pub fn disable_game_bar() -> Result<()> {
    println!("Disabling Xbox Game Bar and Game DVR...");

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

    registry::set_dword(
        r"HKCU\SOFTWARE\Microsoft\GameBar",
        "GamePanelStartupTipIndex",
        3,
    )?;

    registry::set_dword(
        r"HKCU\SOFTWARE\Microsoft\GameBar",
        "ShowStartupPanel",
        0,
    )?;

    registry::set_dword(
        r"HKCU\SOFTWARE\Microsoft\GameBar",
        "UseNexusForGameBarEnabled",
        0,
    )?;

    registry::set_dword(
        r"HKLM\SOFTWARE\Microsoft\WindowsRuntime\ActivatableClassId\Windows.Gaming.GameBar.PresenceServer.Internal.PresenceWriter",
        "ActivationType",
        0,
    )?;

    registry::set_dword(
        r"HKLM\SOFTWARE\Policies\Microsoft\Windows\GameDVR",
        "AllowGameDVR",
        0,
    )?;

    registry::set_dword(
        r"HKLM\SOFTWARE\Microsoft\PolicyManager\default\ApplicationManagement\AllowGameDVR",
        "value",
        0,
    )?;

    Ok(())
}

pub fn disable_sleep_study() -> Result<()> {
    println!("Disabling Sleep Study diagnostics...");

    let logs = [
        "Microsoft-Windows-SleepStudy/Diagnostic",
        "Microsoft-Windows-Kernel-Processor-Power/Diagnostic",
        "Microsoft-Windows-UserModePowerService/Diagnostic",
    ];

    for log in logs {
        let _ = Command::new("wevtutil")
            .args([
                "set-log",
                log,
                "/e:false",
            ])
            .status();
    }

    let _ = Command::new("schtasks")
        .args([
            "/Change",
            "/TN",
            r"\Microsoft\Windows\Power Efficiency Diagnostics\AnalyzeSystem",
            "/Disable",
        ])
        .status();

    Ok(())
}

pub fn apply_cpu_scheduling() -> Result<()> {
    println!("Applying CPU scheduling optimizations...");

    registry::set_dword(
        r"HKLM\SYSTEM\CurrentControlSet\Control\PriorityControl",
        "Win32PrioritySeparation",
        38,
    )?;

    registry::set_dword(
        r"HKLM\SYSTEM\CurrentControlSet\Control",
        "SvcHostSplitThresholdInKB",
        u32::MAX,
    )?;

    registry::set_dword(
        r"HKLM\SYSTEM\CurrentControlSet\Control\Session Manager\Memory Management",
        "DisablePagingExecutive",
        1,
    )?;

    registry::set_dword(
        r"HKLM\SYSTEM\CurrentControlSet\Control\Session Manager",
        "FTHEnabled",
        0,
    )?;

    registry::set_dword(
        r"HKLM\SOFTWARE\Microsoft\Windows NT\CurrentVersion\Multimedia\SystemProfile",
        "SystemResponsiveness",
        10,
    )?;

    registry::set_dword(
        r"HKLM\SOFTWARE\Microsoft\Windows NT\CurrentVersion\Multimedia\SystemProfile",
        "NoLazyMode",
        1,
    )?;

    registry::set_dword(
        r"HKLM\SOFTWARE\Microsoft\Windows NT\CurrentVersion\Multimedia\SystemProfile",
        "NetworkThrottlingIndex",
        u32::MAX,
    )?;

    apply_mmcss_games()?;

    Ok(())
}

fn apply_mmcss_games() -> Result<()> {
    let base =
        r"HKLM\SOFTWARE\Microsoft\Windows NT\CurrentVersion\Multimedia\SystemProfile\Tasks";

    let tasks = [
        "Games",
        "Audio",
        "Pro Audio",
        "Playback",
        "Capture",
        "Window Manager",
    ];

    for task in tasks {
        let key = format!(r"{}\{}", base, task);

        registry::set_dword(&key, "GPU Priority", 8)?;
        registry::set_dword(&key, "Priority", 6)?;
        registry::set_string(&key, "Scheduling Category", "High")?;
        registry::set_string(&key, "SFIO Priority", "High")?;
        registry::set_string(&key, "Latency Sensitive", "True")?;
    }

    Ok(())
}

pub fn apply_filesystem_optimizations() -> Result<()> {
    optimize_ntfs()?;

    let _ = Command::new("fsutil")
        .args([
            "behavior",
            "set",
            "memoryusage",
            "2",
        ])
        .status();

    let _ = Command::new("fsutil")
        .args([
            "behavior",
            "set",
            "mftzone",
            "4",
        ])
        .status();

    let _ = Command::new("fsutil")
        .args([
            "behavior",
            "set",
            "disabledeletenotify",
            "0",
        ])
        .status();

    Ok(())
}

pub fn apply_additional_performance_optimizations() -> Result<()> {
    println!("Applying additional performance optimizations...");

    optimize_ntfs();
    disable_background_apps()?;
    disable_game_bar()?;
    disable_sleep_study()?;

    Ok(())
}

pub fn apply_all_performance_optimizations() -> Result<()> {
    apply_filesystem_optimizations()?;
    apply_cpu_scheduling()?;
    apply_additional_performance_optimizations()?;

    Ok(())
}