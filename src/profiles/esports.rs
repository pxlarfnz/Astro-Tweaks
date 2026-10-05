use anyhow::Result;

use crate::config::optimization::OptimizationConfig;

pub fn apply(config: &OptimizationConfig) -> Result<()> {
    println!();
    println!("======================================");
    println!("       VIVID ESPORTS PROFILE");
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

    // Quality of Life pack
    crate::modules::qol::apply()?;

    // Defender disable (as previously requested)
    crate::modules::defender::apply()?;

    println!();
    println!("[ESPORTS] Complete.");
    println!("⚠️  Reboot is strongly recommended.");

    Ok(())
}