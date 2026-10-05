use anyhow::Result;
use std::process::Command;

use crate::core::registry;

pub fn apply_notification_optimizations() -> Result<()> {
    println!();
    println!("Applying notification optimizations...");

    registry::set_string(
        r"HKCU\SOFTWARE\Microsoft\Windows\CurrentVersion\CapabilityAccessManager\ConsentStore\userNotificationListener",
        "Value",
        "Deny",
    )?;

    registry::set_dword(
        r"HKCU\SOFTWARE\Microsoft\Windows\CurrentVersion\Notifications\Settings",
        "NOC_GLOBAL_SETTING_ALLOW_NOTIFICATION_SOUND",
        0,
    )?;

    registry::set_dword(
        r"HKLM\SOFTWARE\Microsoft\Windows\CurrentVersion\PushNotifications",
        "ToastEnabled",
        0,
    )?;

    registry::set_dword(
        r"HKLM\SOFTWARE\Policies\Microsoft\Windows\CurrentVersion\PushNotifications",
        "NoCloudApplicationNotification",
        1,
    )?;

    registry::set_dword(
        r"HKLM\SOFTWARE\Policies\Microsoft\Windows\Explorer",
        "DisableNotificationCenter",
        1,
    )?;

    disable_notification_services()?;

    println!("Notification optimizations complete.");

    Ok(())
}

fn run_sc(arguments: &[&str]) {
    let _ = Command::new("sc")
        .args(arguments)
        .status();
}

fn disable_notification_services() -> Result<()> {
    run_sc(&[
        "config",
        "WpnService",
        "start=",
        "disabled",
    ]);

    run_sc(&[
        "stop",
        "WpnService",
    ]);

    run_sc(&[
        "config",
        "WpnUserService",
        "start=",
        "disabled",
    ]);

    run_sc(&[
        "stop",
        "WpnUserService",
    ]);

    let output = Command::new("reg")
        .args([
            "query",
            r"HKLM\SYSTEM\CurrentControlSet\Services",
        ])
        .output()?;

    let text = String::from_utf8_lossy(&output.stdout);

    for line in text.lines() {
        let Some(service_name) =
            line.strip_prefix(
                r"HKEY_LOCAL_MACHINE\SYSTEM\CurrentControlSet\Services\",
            )
        else {
            continue;
        };

        let service_name = service_name.trim();

        if !service_name.starts_with("WpnUserService_")
            || service_name.contains('\\')
        {
            continue;
        }

        run_sc(&[
            "config",
            service_name,
            "start=",
            "disabled",
        ]);

        run_sc(&[
            "stop",
            service_name,
        ]);
    }

    Ok(())
}