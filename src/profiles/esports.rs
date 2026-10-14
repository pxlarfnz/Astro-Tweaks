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
    crate::modules::defender::apply()?;

    // Memory trim (included with every profile)
    println!();
    println!("[MEMORY] Trimming process working sets...");
    crate::modules::memory::apply()?;

    // Browser installer (included with every profile)
    println!();
    println!("======================================");
    println!("       BROWSER INSTALLER");
    println!("======================================");
    crate::tools::browser::browser_menu()?;

    println!();
    println!("[ESPORTS] Complete.");
    println!("⚠️  Reboot is strongly recommended.");

    Ok(())
}
