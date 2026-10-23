use anyhow::Result;

pub fn apply() -> Result<()> {
    println!();
    println!("======================================");
    println!("       ASTRO EXTREME PROFILE");
    println!("======================================");
    println!();

    // Do not apply the legacy BCD matrix automatically: it changes boot
    // security and hypervisor settings, including early-launch anti-malware.
    println!("[1/13] Skipping risky boot configuration changes.");

    println!("[2/13] Input optimizations...");
    crate::modules::input::apply()?;

    println!("[3/13] CPU/performance optimizations...");
    crate::modules::performance::apply_all_performance_optimizations()?;

    println!("[4/13] GPU latency optimizations...");
    crate::modules::gpu::apply_latency()?;

    println!("[5/13] Network optimizations...");
    crate::modules::network::apply()?;

    println!("[6/13] Power optimizations...");
    crate::modules::power::apply()?;

    println!("[7/13] Privacy optimizations...");
    crate::modules::privacy::apply_privacy_optimizations()?;

    println!("[8/13] Debloat optimizations...");
    crate::modules::debloat::apply_debloat_optimizations()?;

    println!("[9/13] Notification optimizations...");
    crate::modules::notifications::apply_notification_optimizations()?;

    println!("[10/13] Quality of Life pack...");
    crate::modules::qol::apply()?;

    // Preserve the existing module source but never automatically disable
    // Defender, its services, or protected Windows security files.
    println!("[SECURITY] Windows Defender changes were skipped.");

    println!("[11/13] Memory trim...");
    crate::modules::memory::apply()?;

    println!("[12/13] Miscellaneous utilities...");
    crate::modules::misc::apply_misc_system_utilities()?;

    println!("[13/13] Browser installer...");
    println!("======================================");
    println!("       BROWSER INSTALLER");
    println!("======================================");
    crate::tools::browser::browser_menu()?;

    println!();
    println!("Extreme profile completed.");
    println!("A reboot may be required for some settings to take effect.");

    Ok(())
}
