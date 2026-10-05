#[derive(Debug, Clone)]
pub struct OptimizationConfig {
    pub input: bool,
    pub performance: bool,
    pub gpu: bool,
    pub network: bool,
    pub power: bool,
    pub privacy: bool,
    pub shell: bool,
    pub debloat: bool,
    pub services: bool,
    pub bcd: bool,

    /*
     * Security mitigation disabling is intentionally not enabled.
     *
     * This is where the source-script inventory is represented without
     * automatically weakening Windows security.
     */
    pub advanced_security: bool,

    pub install_tools: bool,
}

impl Default for OptimizationConfig {
    fn default() -> Self {
        Self {
            input: true,
            performance: true,
            gpu: true,
            network: true,
            power: true,
            privacy: true,
            shell: true,
            debloat: true,
            services: false,
            bcd: false,
            advanced_security: false,
            install_tools: false,
        }
    }
}