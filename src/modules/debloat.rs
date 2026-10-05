use anyhow::Result;
use std::process::Command;

use crate::core::registry;

pub fn apply_content_delivery_optimizations() -> Result<()> {
    println!("Disabling Windows Content Delivery suggestions...");

    let key =
        r"HKCU\SOFTWARE\Microsoft\Windows\CurrentVersion\ContentDeliveryManager";

    let values = [
        "ContentDeliveryAllowed",
        "FeatureManagementEnabled",
        "SubscribedContentEnabled",
        "RemediationRequired",
        "OemPreInstalledAppsEnabled",
        "PreInstalledAppsEnabled",
        "PreInstalledAppsEverEnabled",
        "SilentInstalledAppsEnabled",
        "EnableAccountNotifications",
        "SubscribedContent-310093Enabled",
        "SubscribedContent-338393Enabled",
        "SubscribedContent-353694Enabled",
        "SubscribedContent-353696Enabled",
        "SubscribedContent-338388Enabled",
        "SubscribedContent-338387Enabled",
        "SubscribedContent-338389Enabled",
        "SystemPaneSuggestionsEnabled",
        "RotatingLockScreenOverlayEnabled",
        "SoftLandingEnabled",
    ];

    for value in values {
        registry::set_dword(
            key,
            value,
            0,
        )?;
    }

    registry::set_dword(
        r"HKCU\Software\Microsoft\Windows\CurrentVersion\SystemSettings\AccountNotifications",
        "EnableAccountNotifications",
        0,
    )?;

    Ok(())
}

pub fn apply_storage_sense_optimizations() -> Result<()> {
    println!("Configuring Storage Sense...");

    let key =
        r"HKCU\SOFTWARE\Microsoft\Windows\CurrentVersion\StorageSense\Parameters\StoragePolicy";

    for value in [
        "32",
        "02",
        "128",
        "08",
        "256",
    ] {
        registry::set_dword(
            key,
            value,
            0,
        )?;
    }

    registry::set_dword(
        key,
        "01",
        1,
    )?;

    registry::set_dword(
        key,
        "1024",
        1,
    )?;

    registry::set_dword(
        key,
        "04",
        1,
    )?;

    registry::set_dword(
        key,
        "2048",
        30,
    )?;

    // Trigger SilentCleanup.
    let _ = Command::new("schtasks")
        .args([
            "/Run",
            "/TN",
            r"\Microsoft\Windows\DiskCleanup\SilentCleanup",
        ])
        .status();

    Ok(())
}

pub fn disable_reserved_storage() -> Result<()> {
    println!("Disabling Reserved Storage...");

    let status = Command::new("dism.exe")
        .args([
            "/Online",
            "/Set-ReservedStorageState",
            "/State:Disabled",
        ])
        .status()?;

    if !status.success() {
        println!("Reserved Storage could not be changed.");
    }

    Ok(())
}

pub fn disable_debloat_scheduled_tasks() -> Result<()> {
    println!("Disabling selected telemetry/debloat scheduled tasks...");

    let tasks = [
        r"\Microsoft\Windows\Application Experience\PcaPatchDbTask",
        r"\Microsoft\Windows\AppxDeploymentClient\UCPD velocity",
        r"\Microsoft\Windows\Flighting\FeatureConfig\UsageDataReporting",
    ];

    for task in tasks {
        let _ = Command::new("schtasks")
            .args([
                "/Change",
                "/TN",
                task,
                "/Disable",
            ])
            .status();
    }

    Ok(())
}

pub fn disable_content_delivery_registry() -> Result<()> {
    registry::set_dword(
        r"HKCU\SOFTWARE\Microsoft\Windows\CurrentVersion\ContentDeliveryManager",
        "SilentInstalledAppsEnabled",
        0,
    )?;

    registry::set_dword(
        r"HKCU\SOFTWARE\Microsoft\Windows\CurrentVersion\ContentDeliveryManager",
        "SystemPaneSuggestionsEnabled",
        0,
    )?;

    registry::set_dword(
        r"HKCU\SOFTWARE\Microsoft\Windows\CurrentVersion\ContentDeliveryManager",
        "SubscribedContent-338393Enabled",
        0,
    )?;

    registry::set_dword(
        r"HKLM\SOFTWARE\Policies\Microsoft\Windows\CloudContent",
        "DisableWindowsConsumerFeatures",
        1,
    )?;

    registry::set_dword(
        r"HKLM\SOFTWARE\Policies\Microsoft\Windows\CloudContent",
        "DisableThirdPartySuggestions",
        1,
    )?;

    Ok(())
}

pub fn apply_debloat_optimizations() -> Result<()> {
    println!("Applying debloat optimizations...");

    disable_content_delivery_registry()?;
    apply_content_delivery_optimizations()?;
    apply_storage_sense_optimizations()?;
    disable_reserved_storage()?;
    disable_debloat_scheduled_tasks()?;

    println!("Debloat optimizations complete.");

    Ok(())
}