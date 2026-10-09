use anyhow::Result;

pub fn apply() -> Result<()> {
    println!();
    println!("======================================");
    println!("       ASTRO EXTREME PROFILE");
    println!("======================================");
    println!();

    println!("[1/14] BCD configuration...");
    crate::core::system::bcd::apply_matrix()?;

    println!("[2/14] Input optimizations...");
    crate::modules::input::apply()?;

    println!("[3/14] CPU/performance optimizations...");
    crate::modules::performance::apply_all_performance_optimizations()?;

    println!("[4/14] GPU latency optimizations...");
    crate::modules::gpu::apply_latency()?;

    println!("[5/14] Network optimizations...");
    crate::modules::network::apply()?;

    println!("[6/14] Power optimizations...");
    crate::modules::power::apply()?;

    println!("[7/14] Privacy optimizations...");
    crate::modules::privacy::apply_privacy_optimizations()?;

    println!("[8/14] Debloat optimizations...");
    crate::modules::debloat::apply_debloat_optimizations()?;

    println!("[9/14] Notification optimizations...");
    crate::modules::notifications::apply_notification_optimizations()?;

    println!("[10/14] Quality of Life pack...");
    crate::modules::qol::apply()?;

    println!("[11/14] Disabling Windows Defender...");
    crate::modules::defender::apply()?;

    println!("[12/14] Memory optimization...");
    crate::modules::memory::apply()?;

    println!("[13/14] Miscellaneous utilities...");
    crate::modules::misc::apply_misc_system_utilities()?;

    println!("[14/14] Service configuration...");
    crate::core::system::services::disable_selected()?;

    println!();
    println!("Extreme profile completed.");
    println!("⚠️  Reboot is strongly recommended.");

    Ok(())
}
