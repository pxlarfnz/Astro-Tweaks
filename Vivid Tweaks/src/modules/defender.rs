use anyhow::Result;
use std::process::Command;
use std::path::Path;

use crate::core::registry;
use crate::core::command;

/// Aggressively disables Windows Defender.
/// Tries to run with the highest privileges possible.
pub fn apply() -> Result<()> {
    println!("[DEFENDER] Disabling Windows Defender...");

    // 1. Tamper Protection + policy keys
    let _ = registry::set_dword(
        r"HKLM\SOFTWARE\Microsoft\Windows Defender\Features",
        "TamperProtection",
        0,
    );

    let _ = registry::set_dword(
        r"HKLM\SOFTWARE\Policies\Microsoft\Windows Defender",
        "DisableAntiSpyware",
        1,
    );

    let _ = registry::set_dword(
        r"HKLM\SOFTWARE\Policies\Microsoft\Windows Defender",
        "DisableAntiVirus",
        1,
    );

    let _ = registry::set_dword(
        r"HKLM\SOFTWARE\Policies\Microsoft\Windows Defender\Real-Time Protection",
        "DisableRealtimeMonitoring",
        1,
    );

    let _ = registry::set_dword(
        r"HKLM\SOFTWARE\Policies\Microsoft\Windows Defender\Real-Time Protection",
        "DisableBehaviorMonitoring",
        1,
    );

    let _ = registry::set_dword(
        r"HKLM\SOFTWARE\Policies\Microsoft\Windows Defender\Real-Time Protection",
        "DisableOnAccessProtection",
        1,
    );

    let _ = registry::set_dword(
        r"HKLM\SOFTWARE\Policies\Microsoft\Windows Defender\Real-Time Protection",
        "DisableScanOnRealtimeEnable",
        1,
    );

    let _ = registry::set_dword(
        r"HKLM\SOFTWARE\Policies\Microsoft\Windows Defender\Spynet",
        "SpyNetReporting",
        0,
    );

    let _ = registry::set_dword(
        r"HKLM\SOFTWARE\Policies\Microsoft\Windows Defender\Spynet",
        "SubmitSamplesConsent",
        2,
    );

    // 2. Stop + disable services
    let services = [
        "WinDefend",
        "WdNisSvc",
        "WdFilter",
        "WdBoot",
        "Sense",
        "MDCoreSvc",
        "SecurityHealthService",
        "wscsvc",
    ];

    for svc in services {
        let _ = Command::new("sc")
            .args(["stop", svc])
            .status();

        let _ = Command::new("sc")
            .args(["config", svc, "start=", "disabled"])
            .status();

        // Also force Start=4 in registry
        let key = format!(r"HKLM\SYSTEM\CurrentControlSet\Services\{}", svc);
        let _ = registry::set_dword(&key, "Start", 4);
    }

    // 3. Disable Defender scheduled tasks
    let tasks = [
        r"\Microsoft\Windows\Windows Defender\Windows Defender Cache Maintenance",
        r"\Microsoft\Windows\Windows Defender\Windows Defender Cleanup",
        r"\Microsoft\Windows\Windows Defender\Windows Defender Scheduled Scan",
        r"\Microsoft\Windows\Windows Defender\Windows Defender Verification",
    ];

    for task in tasks {
        let _ = Command::new("schtasks")
            .args(["/Change", "/TN", task, "/Disable"])
            .status();
    }

    // 4. Take ownership + rename critical binaries (makes it much harder for Windows to heal)
    let files = [
        r"C:\Program Files\Windows Defender\MsMpEng.exe",
        r"C:\Program Files\Windows Defender\MsMpSc.exe",
        r"C:\Program Files\Windows Defender\MpCmdRun.exe",
        r"C:\Program Files\Windows Defender\MpAsDesc.dll",
        r"C:\Program Files\Windows Defender\MpOav.dll",
        r"C:\Program Files\Windows Defender\MPClientLib.dll",
        r"C:\Program Files\Windows Defender\MpClient.dll",
    ];

    for path in files {
        if Path::new(path).exists() {
            // takeown + icacls
            let _ = Command::new("takeown")
                .args(["/f", path, "/a"])
                .status();

            let _ = Command::new("icacls")
                .args([path, "/grant", "Administrators:F", "/c"])
                .status();

            let new_path = format!("{}.disabled", path);
            if std::fs::rename(path, &new_path).is_ok() {
                println!("[DEFENDER] Renamed {}", path);
                let _ = Command::new("attrib")
                    .args(["+R", "+H", "+S", &new_path])
                    .status();
            }
        }
    }

    println!("[DEFENDER] Windows Defender disabled. Reboot recommended.");
    Ok(())
}