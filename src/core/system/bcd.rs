use anyhow::Result;

use crate::core::command::bcdedit;

/*
 * These are the BCD items supplied by the scripts.
 *
 * They are intentionally kept in one place because BCD changes affect
 * boot configuration rather than normal application performance.
 */

const BCD_TWEAKS: &[(&str, &str)] = &[
    ("useplatformclock", "No"),
    ("useplatformtick", "No"),
    ("disabledynamictick", "Yes"),
    ("tscsyncpolicy", "Enhanced"),
    ("firstmegabytepolicy", "UseAll"),
    ("avoidlowmemory", "0x8000000"),
    ("nolowmem", "Yes"),
    ("allowedinmemorysettings", "0x0"),
    ("isolatedcontext", "No"),
    ("vsmlaunchtype", "Off"),
    ("vm", "No"),
    ("x2apicpolicy", "Enable"),
    ("configaccesspolicy", "Default"),
    ("MSI", "Default"),
    ("usephysicaldestination", "No"),
    ("usefirmwarepcisettings", "No"),
    ("disableelamdrivers", "Yes"),
    ("pae", "ForceEnable"),
    ("nx", "optout"),
    ("highestmode", "Yes"),
    ("forcefipscrypto", "No"),
    ("noumex", "Yes"),
    ("uselegacyapicmode", "No"),
    ("ems", "No"),
    ("extendedinput", "Yes"),
    ("debug", "No"),
    ("hypervisorlaunchtype", "Off"),
    ("quietboot", "yes"),
    ("bootmenupolicy", "Legacy"),
];

pub fn apply_matrix() -> Result<()> {
    println!("[BCD] Applying BCD matrix...");

    for &(name, value) in BCD_TWEAKS {
        if let Err(error) = bcdedit(&["/set", name, value]) {
            eprintln!(
                "[BCD] {}={} -> {}",
                name,
                value,
                error
            );
        }
    }

    Ok(())
}