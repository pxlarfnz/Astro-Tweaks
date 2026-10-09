use anyhow::{Context, Result};
use winreg::{enums::*, RegKey};

pub fn set_dword(path: &str, name: &str, value: u32) -> Result<()> {
    let (hive, subkey) = split_path(path)?;
    let root = RegKey::predef(hive);
    let (key, _) = root.create_subkey(subkey)
        .with_context(|| format!("Failed to open/create {}", path))?;
    key.set_value(name, &value)
        .with_context(|| format!("Failed to set {}\\{}", path, name))?;
    Ok(())
}

pub fn set_string(path: &str, name: &str, value: &str) -> Result<()> {
    let (hive, subkey) = split_path(path)?;
    let root = RegKey::predef(hive);
    let (key, _) = root.create_subkey(subkey)
        .with_context(|| format!("Failed to open/create {}", path))?;
    key.set_value(name, &value)
        .with_context(|| format!("Failed to set {}\\{}", path, name))?;
    Ok(())
}

fn split_path(path: &str) -> Result<(isize, &str)> {
    if let Some(rest) = path.strip_prefix(r"HKLM\") {
        Ok((HKEY_LOCAL_MACHINE, rest))
    } else if let Some(rest) = path.strip_prefix(r"HKCU\") {
        Ok((HKEY_CURRENT_USER, rest))
    } else if let Some(rest) = path.strip_prefix(r"HKEY_LOCAL_MACHINE\") {
        Ok((HKEY_LOCAL_MACHINE, rest))
    } else if let Some(rest) = path.strip_prefix(r"HKEY_CURRENT_USER\") {
        Ok((HKEY_CURRENT_USER, rest))
    } else {
        anyhow::bail!("Unsupported registry path: {}", path)
    }
}
