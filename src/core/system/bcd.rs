use anyhow::Result;

use crate::core::command::bcdedit;

/*
 * Keep the existing boot-configuration matrix in one place. BCD changes
 * can prevent booting or reduce platform security, so the matrix deliberately
 * excludes entries that disable early-launch anti-malware, DEP, VBS, or the
 * Windows hypervisor. Remaining entries are still best-effort and may vary
 * by Windows version and hardware.
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
    ("x2apicpolicy", "Enable"),
    ("configaccesspolicy", "Default"),
    ("MSI", "Default"),
    ("usephysicaldestination", "No"),
    ("usefirmwarepcisettings", "No"),
    ("pae", "ForceEnable"),
    ("highestmode", "Yes"),
    ("forcefipscrypto", "No"),
    ("noumex", "Yes"),
    ("uselegacyapicmode", "No"),
    ("ems", "No"),
    ("extendedinput", "Yes"),
    ("debug", "No"),
    ("quietboot", "yes"),
    ("bootmenupolicy", "Legacy"),
];

pub fn apply_matrix() -> Result<()> {
    println!("[BCD] Applying the retained boot configuration matrix.");
    println!("[BCD] Security-sensitive entries (ELAM, DEP, VBS, hypervisor) are intentionally excluded.");

    for &(name, value) in BCD_TWEAKS {
        if let Err(error) = bcdedit(&["/set", name, value]) {
            eprintln!("[BCD] {name}={value} -> {error:#}");
        }
    }

    Ok(())
}
