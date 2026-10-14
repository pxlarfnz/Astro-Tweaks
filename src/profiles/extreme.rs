use anyhow::Result;

pub fn apply() -> Result<()> {
    println!();
    println!("======================================");
    println!("       ASTRO EXTREME PROFILE");
    println!("======================================");
    println!();

    println!("[1/15] BCD configuration...");
    crate::core::system::bcd::apply_matrix()?;

    println!("[2/15] Input optimizations...");
    crate::modules::input::apply()?;

    println!("[3/15] CPU/performance optimizations...");
    crate::modules::performance::apply_all_performance_optimizations()?;

    println!("[4/15] GPU latency optimizations...");
    crate::modules::gpu::apply_latency()?;

    println!("[5/15] Network optimizations...");
    crate::modules::network::apply()?;

    println!("[6/15] Power optimizations...");
    crate::modules::power::apply()?;

    println!("[7/15] Privacy optimizations...");
    crate::modules::privacy::apply_privacy_optimizations()?;

    println!("[8/15] Debloat optimizations...");
    crate::modules::debloat::apply_debloat_optimizations()?;

    println!("[9/15] Notification optimizations...");
    crate::modules::notifications::apply_notification_optimizations()?;

    println!("[10/15] Quality of Life pack...");
    crate::modules::qol::apply()?;

    println!("[11/15] Disabling Windows Defender...");
    crate::modules::defender::apply()?;

    println!("[12/15] Memory trim...");
    crate::modules::memory::apply()?;

    println!("[13/15] Miscellaneous utilities...");
    crate::modules::misc::apply_misc_system_utilities()?;

    println!("[14/15] Service configuration...");
    crate::core::system::services::disable_selected()?;

    println!();
    println!("[15/15] Browser installer...");
    println!("======================================");
    println!("       BROWSER INSTALLER");
    println!("======================================");
    crate::tools::browser::browser_menu()?;

    println!();
    println!("Extreme profile completed.");
    println!("⚠️  Reboot is strongly recommended.");

    Ok(())
}
