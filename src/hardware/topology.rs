use anyhow::Result;

pub fn logical_processor_count() -> Result<usize> {
    crate::hardware::cpu::logical_processor_count()
}