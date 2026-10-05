use anyhow::Result;

pub fn apply() -> Result<()> {
    println!();
    println!("======================================");
    println!("      VIVID STREAMING PROFILE");
    println!("======================================");
    println!();

    println!("[1/11] Input optimizations...");
    crate::modules::input::apply()?;

    println!("[2/11] CPU/MMCSS optimizations...");
    crate::modules::performance::apply_cpu_scheduling()?;
    crate::modules::performance::apply_additional_performance_optimizations()?;

    println!("[3/11] GPU quality optimizations...");
    crate::modules::gpu::apply_quality()?;

    println!("[4/11] Network optimizations...");
    crate::modules::network::apply()?;

    println!("[5/11] Power optimizations...");
    crate::modules::power::apply()?;

    println!("[6/11] Privacy optimizations...");
    crate::modules::privacy::apply_privacy_optimizations()?;

    println!("[7/11] Debloat optimizations...");
    crate::modules::debloat::apply_debloat_optimizations()?;

    println!("[8/11] Notification optimizations...");
    crate::modules::notifications::apply_notification_optimizations()?;

    println!("[9/11] Quality of Life pack...");
    crate::modules::qol::apply()?;

    println!("[10/11] Memory optimization...");
    crate::modules::memory::apply()?;

    println!("[11/11] Miscellaneous utilities...");
    crate::modules::misc::apply_misc_system_utilities()?;

    println!();
    println!("Streaming profile completed.");
    println!("⚠️  Reboot is strongly recommended.");

    Ok(())
}