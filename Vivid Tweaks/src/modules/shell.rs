use anyhow::Result;
use winreg::{
    enums::*,
    RegKey,
};

pub fn apply() -> Result<()> {
    println!("[SHELL] Applying Explorer/UI settings...");

    let hkcu =
        RegKey::predef(HKEY_CURRENT_USER);

    let (desktop, _) =
        hkcu.create_subkey(
            r"Control Panel\Desktop"
        )?;

    desktop.set_value(
        "MenuShowDelay",
        &"0",
    )?;

    let (advanced, _) =
        hkcu.create_subkey(
            r"SOFTWARE\Microsoft\Windows\CurrentVersion\Explorer\Advanced"
        )?;

    advanced.set_value(
        "DontPrettyPath",
        &1u32,
    )?;

    advanced.set_value(
        "DisallowShaking",
        &1u32,
    )?;

    let (visual, _) =
        hkcu.create_subkey(
            r"SOFTWARE\Microsoft\Windows\CurrentVersion\Explorer\VisualEffects"
        )?;

    visual.set_value(
        "VisualFXSetting",
        &3u32,
    )?;

    let effects = [
        "AnimateMinMax",
        "ComboBoxAnimation",
        "ControlAnimations",
        "CursorShadow",
        "DragFullWindows",
        "DropShadow",
        "DWMAeroPeekEnabled",
        "ListviewAlphaSelect",
        "ListviewShadow",
        "MenuAnimation",
        "SelectionFade",
        "TaskbarAnimations",
        "TooltipAnimation",
    ];

    for effect in effects {
        let path =
            format!(
                r"SOFTWARE\Microsoft\Windows\CurrentVersion\Explorer\VisualEffects\{}",
                effect
            );

        let (key, _) =
            hkcu.create_subkey(path)?;

        key.set_value(
            "DefaultApplied",
            &"0",
        )?;
    }

    let hklm =
        RegKey::predef(HKEY_LOCAL_MACHINE);

    let (filesystem, _) =
        hklm.create_subkey(
            r"SYSTEM\CurrentControlSet\Control\FileSystem"
        )?;

    filesystem.set_value(
        "LongPathsEnabled",
        &1u32,
    )?;

    let (notifications, _) =
        hkcu.create_subkey(
            r"SOFTWARE\Policies\Microsoft\Windows\CurrentVersion\PushNotifications"
        )?;

    notifications.set_value(
        "NoTileApplicationNotification",
        &1u32,
    )?;

    Ok(())
}