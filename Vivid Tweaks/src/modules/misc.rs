use anyhow::Result;
use std::process::Command;

pub fn add_music_and_videos_to_home() -> Result<()> {
    println!("Adding Music and Videos to File Explorer Home...");

    let script = r#"
$o = New-Object -ComObject shell.application
$home = $o.Namespace('shell:::{679f85cb-0220-4080-b29b-5540cc05aab6}')

if ($null -ne $home) {
    $currentPins = $home.Items() |
        ForEach-Object { $_.Path }

    foreach ($path in @(
        [Environment]::GetFolderPath('MyVideos'),
        [Environment]::GetFolderPath('MyMusic')
    )) {
        if ($currentPins -notcontains $path) {
            try {
                $o.Namespace($path).Self.InvokeVerb('pintohome')
            } catch {
            }
        }
    }
}
"#;

    let _ = Command::new("powershell.exe")
        .args([
            "-NoProfile",
            "-ExecutionPolicy",
            "Bypass",
            "-Command",
            script,
        ])
        .status();

    Ok(())
}

pub fn set_time_servers() -> Result<()> {
    println!("Configuring Windows Time servers...");

    let _ = Command::new("sc")
        .args([
            "start",
            "w32time",
        ])
        .status();

    let status = Command::new("w32tm")
        .args([
            "/config",
            "/syncfromflags:manual",
            "/manualpeerlist:0.pool.ntp.org 1.pool.ntp.org 2.pool.ntp.org 3.pool.ntp.org",
        ])
        .status()?;

    if !status.success() {
        println!("w32tm configuration returned an error.");
    }

    let _ = Command::new("w32tm")
        .args([
            "/config",
            "/update",
        ])
        .status();

    let _ = Command::new("w32tm")
        .arg("/resync")
        .status();

    Ok(())
}

pub fn reset_performance_counters() -> Result<()> {
    println!("Rebuilding performance counters...");

    let _ = Command::new("lodctr")
        .arg("/r")
        .status();

    let _ = Command::new("lodctr")
        .arg("/r")
        .status();

    let _ = Command::new("winmgmt")
        .arg("/resyncperf")
        .status();

    Ok(())
}

pub fn apply_misc_system_utilities() -> Result<()> {
    add_music_and_videos_to_home()?;
    set_time_servers()?;
    reset_performance_counters()?;

    Ok(())
}