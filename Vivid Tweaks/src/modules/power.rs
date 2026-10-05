use anyhow::Result;
use winreg::{
    enums::*,
    RegKey,
};

pub fn apply() -> Result<()> {
    println!("[POWER] Applying power configuration...");

    /*
     * Preserve the power-related settings from the supplied scripts.
     */
    let commands: &[&[&str]] = &[
        &["/h", "off"],

        &[
            "/setacvalueindex",
            "scheme_current",
            "SUB_SLEEP",
            "STANDBYIDLE",
            "0",
        ],

        &[
            "/setacvalueindex",
            "scheme_current",
            "SUB_VIDEO",
            "VIDEOIDLE",
            "0",
        ],

        &[
            "/setacvalueindex",
            "scheme_current",
            "SUB_DISK",
            "DISKIDLE",
            "0",
        ],

        &[
            "/setacvalueindex",
            "scheme_current",
            "SUB_USB",
            "USBSELECTIVE",
            "0",
        ],

        &[
            "/setacvalueindex",
            "scheme_current",
            "SUB_PCIEXPRESS",
            "ASPM",
            "0",
        ],
    ];

    for command in commands {
        if let Err(error) =
            crate::core::command::powercfg(command)
        {
            eprintln!(
                "[POWERCFG] {:?} -> {}",
                command,
                error
            );
        }
    }

    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);

    let (power, _) =
        hklm.create_subkey(
            r"SYSTEM\CurrentControlSet\Control\Power"
        )?;

    power.set_value(
        "HiberbootEnabled",
        &0u32,
    )?;

    power.set_value(
        "PowerThrottlingOff",
        &1u32,
    )?;

    /*
     * PCIe ASPM opt-out.
     */
    let (aspm, _) =
        hklm.create_subkey(
            r"SYSTEM\CurrentControlSet\Control\Power\PowerSettings\501a4d13-42af-4429-9fd1-a8218c268e20"
        )?;

    aspm.set_value(
        "Attributes",
        &0u32,
    )?;

    /*
     * Processor power settings from the source inventory.
     */
    for service in [
        r"SYSTEM\CurrentControlSet\Services\intelppm",
        r"SYSTEM\CurrentControlSet\Services\amdppm",
    ] {
        if let Ok((key, _)) =
            hklm.create_subkey(service)
        {
            let _ = key.set_value(
                "Latency",
                &0u32,
            );
        }
    }

    Ok(())
}