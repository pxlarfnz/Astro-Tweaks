use anyhow::Result;\nuse colored::Colorize;

pub fn apply() -> Result<()> {
    println!();
    println!("{}", "======================================".bright_yellow());
    println!("{}", "       ASTRO EXTREME PROFILE".bright_yellow().bold());
    println!("{}", "======================================".bright_yellow());
    println!();

    // The matrix is retained, with boot-security-disabling entries excluded.
    println!("[1/13] Boot configuration matrix...");
    crate::core::system::bcd::apply_matrix()?;

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

    // Keep the legacy source file, but do not automatically disable Defender
    // or tamper with protected Windows security files.
    println!("{}", "[SECURITY] Windows Defender changes were skipped.".bright_yellow());

    println!("[11/13] Memory trim...");
    crate::modules::memory::apply()?;

    println!("[12/13] Miscellaneous utilities...");
    crate::modules::misc::apply_misc_system_utilities()?;

    println!("[13/13] Browser installer...");
    println!("{}", "======================================".bright_yellow());
    println!("{}", "       BROWSER INSTALLER".bright_yellow().bold());
    println!("{}", "======================================".bright_yellow());
    crate::tools::browser::browser_menu()?;

    println!();
    println!("{}", "Extreme profile completed.".bright_yellow());
    println!("Review boot changes carefully; a reboot may be required.");

    Ok(())
}
