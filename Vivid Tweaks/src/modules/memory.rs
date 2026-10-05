use anyhow::Result;

pub fn apply() -> Result<()> {
    crate::core::memory::trim_working_sets()
}