use anyhow::Result;
use colored::Colorize;

pub fn apply() -> Result<()> {
    println!();
    println!("{}", "======================================".bright_yellow());
    println!("{}", "      ASTRO STREAMING PROFILE".bright_yellow().bold());
    println!("{}", "======================================".bright_yellow());
    println!();

    println!("[1/12] Input optimizations...");
    crate::modules::input::apply()?;

    println!("[2/12] CPU/MMCSS optimizations...");
    crate::modules::performance::apply_cpu_scheduling()?;
    crate::modules::performance::apply_additional_performance_optimizations()?;

    println!("[3/12] GPU quality optimizations...");
    crate::modules::gpu::apply_quality()?;

    println!("[4/12] Network optimizations...");
    crate::modules::network::apply()?;

    println!("[5/12] Power optimizations...");
    crate::modules::power::apply()?;

    println!("[6/12] Privacy optimizations...");
    crate::modules::privacy::apply_privacy_optimizations()?;

    println!("[7/12] Debloat optimizations...");
    crate::modules::debloat::apply_debloat_optimizations()?;

    println!("[8/12] Notification optimizations...");
    crate::modules::notifications::apply_notification_optimizations()?;

    println!("[9/12] Quality of Life pack...");
    crate::modules::qol::apply()?;

    println!("[10/12] Memory trim...");
    crate::modules::memory::apply()?;

    println!("[11/12] Miscellaneous utilities...");
    crate::modules::misc::apply_misc_system_utilities()?;

    println!();
    println!("[12/12] Browser installer...");
    println!("{}", "======================================".bright_yellow());
    println!("{}", "       BROWSER INSTALLER".bright_yellow().bold());
    println!("{}", "======================================".bright_yellow());
    crate::tools::browser::browser_menu()?;

    println!();
    println!("Streaming profile completed.");
    println!("⚠️  Reboot is strongly recommended.");

    Ok(())
}
