use anyhow::Result;

pub fn logical_processor_count() -> Result<usize> {
    Ok(std::thread::available_parallelism()?.get())
}