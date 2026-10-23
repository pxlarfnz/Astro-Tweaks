use anyhow::Result;

use crate::config::optimization::OptimizationConfig;

pub fn apply(config: &OptimizationConfig) -> Result<()> {
    println!();
    println!("======================================");
    println!("       ASTRO ESPORTS PROFILE");
    println!("======================================");

    if config.input {
        crate::modules::input::apply()?;
    }

    if config.performance {
        crate::modules::performance::apply_all_performance_optimizations()?;
    }

    if config.gpu {
        crate::modules::gpu::apply_latency()?;
    }

    if config.network {
        crate::modules::network::apply()?;
    }

    if config.power {
        crate::modules::power::apply()?;
    }

    if config.privacy {
        crate::modules::privacy::apply_privacy_optimizations()?;
    }

    if config.shell {
        crate::modules::shell::apply()?;
    }

    crate::modules::qol::apply()?;

    // The legacy Defender module is preserved in the repository, but is not
    // automatically executed because it attempts to disable built-in security
    // protections and tamper with protected Windows files.
    println!("[SECURITY] Windows Defender changes were skipped.");

    println!();
    println!("[MEMORY] Trimming process working sets...");
    crate::modules::memory::apply()?;

    println!();
    println!("======================================");
    println!("       BROWSER INSTALLER");
    println!("======================================");
    crate::tools::browser::browser_menu()?;

    println!();
    println!("[ESPORTS] Complete.");
    println!("A reboot may be required for some settings to take effect.");

    Ok(())
}
