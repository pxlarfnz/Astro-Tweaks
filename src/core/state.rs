use anyhow::Result;
use std::{
    fs,
    path::Path,
};

const STATE_DIR: &str = r"C:\ProgramData\AstroTweaks\State";

pub fn write_state(feature: &str, state: &str) -> Result<()> {
    fs::create_dir_all(STATE_DIR)?;

    let path = Path::new(STATE_DIR).join(format!("{}.state", feature));

    fs::write(path, state)?;

    Ok(())
}

pub fn read_state(feature: &str) -> Option<String> {
    let path = Path::new(STATE_DIR).join(format!("{}.state", feature));

    fs::read_to_string(path).ok()
}
