use anyhow::Result;
use winreg::{
    enums::*,
    RegKey,
};

fn set_dword(
    root: &RegKey,
    path: &str,
    name: &str,
    value: u32,
) -> Result<()> {
    let (key, _) = root.create_subkey(path)?;
    key.set_value(name, &value)?;
    Ok(())
}

fn set_string(
    root: &RegKey,
    path: &str,
    name: &str,
    value: &str,
) -> Result<()> {
    let (key, _) = root.create_subkey(path)?;
    key.set_value(name, &value)?;
    Ok(())
}

pub fn apply() -> Result<()> {
    println!("[INPUT] Applying keyboard/mouse settings...");

    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);

    set_dword(
        &hklm,
        r"SYSTEM\CurrentControlSet\Services\kbdclass\Parameters",
        "KeyboardDataQueueSize",
        16,
    )?;

    set_dword(
        &hklm,
        r"SYSTEM\CurrentControlSet\Services\mouclass\Parameters",
        "MouseDataQueueSize",
        16,
    )?;

    set_string(
        &hkcu,
        r"Control Panel\Keyboard",
        "KeyboardDelay",
        "0",
    )?;

    set_string(
        &hkcu,
        r"Control Panel\Keyboard",
        "KeyboardSpeed",
        "31",
    )?;

    set_string(
        &hkcu,
        r"Control Panel\Mouse",
        "MouseSpeed",
        "0",
    )?;

    set_string(
        &hkcu,
        r"Control Panel\Mouse",
        "MouseSensitivity",
        "10",
    )?;

    set_dword(
        &hklm,
        r"SYSTEM\CurrentControlSet\Services\USB",
        "DisableSelectiveSuspend",
        1,
    )?;

    set_dword(
        &hklm,
        r"SYSTEM\CurrentControlSet\Services\USBXHCI",
        "IdleEnable",
        0,
    )?;

    set_dword(
        &hklm,
        r"SYSTEM\CurrentControlSet\Services\USBXHCI",
        "WaitWakeEnabled",
        0,
    )?;

    /*
     * USB MSI mode.
     */
    crate::core::command::powershell(
        r#"
Get-CimInstance Win32_USBController |
Where-Object {$_.PNPDeviceID -match "PCI\\VEN_"} |
ForEach-Object {

    $path =
        "HKLM:\SYSTEM\CurrentControlSet\Enum\$($_.PNPDeviceID)\Device Parameters\Interrupt Management"

    New-Item `
        "$path\MessageSignaledInterruptProperties" `
        -Force |
        Out-Null

    Set-ItemProperty `
        "$path\MessageSignaledInterruptProperties" `
        -Name MSISupported `
        -Value 1 `
        -Type DWord

    New-Item `
        "$path\Affinity Policy" `
        -Force |
        Out-Null

    Set-ItemProperty `
        "$path\Affinity Policy" `
        -Name DevicePriority `
        -Value 0 `
        -Type DWord
}
"#,
    )?;

    Ok(())
}