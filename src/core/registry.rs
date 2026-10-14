use anyhow::{bail, Context, Result};
use winreg::{
    enums::*,
    RegKey,
};

/// Parse a full registry path like `HKLM\SOFTWARE\...` or `HKCU\...`
/// and return the root hive + the subkey path.
fn parse_path(full_path: &str) -> Result<(RegKey, &str)> {
    let full_path = full_path.trim_start_matches('\\');

    if let Some(rest) = full_path
        .strip_prefix("HKLM\\")
        .or_else(|| full_path.strip_prefix("HKEY_LOCAL_MACHINE\\"))
    {
        return Ok((RegKey::predef(HKEY_LOCAL_MACHINE), rest));
    }

    if let Some(rest) = full_path
        .strip_prefix("HKCU\\")
        .or_else(|| full_path.strip_prefix("HKEY_CURRENT_USER\\"))
    {
        return Ok((RegKey::predef(HKEY_CURRENT_USER), rest));
    }

    if let Some(rest) = full_path
        .strip_prefix("HKCR\\")
        .or_else(|| full_path.strip_prefix("HKEY_CLASSES_ROOT\\"))
    {
        return Ok((RegKey::predef(HKEY_CLASSES_ROOT), rest));
    }

    if let Some(rest) = full_path
        .strip_prefix("HKU\\")
        .or_else(|| full_path.strip_prefix("HKEY_USERS\\"))
    {
        return Ok((RegKey::predef(HKEY_USERS), rest));
    }

    bail!(
        "Unsupported registry root in path: {}. Expected HKLM, HKCU, HKCR or HKU.",
        full_path
    );
}

/// Create (or open) a registry key and set a DWORD value.
pub fn set_dword(full_path: &str, name: &str, value: u32) -> Result<()> {
    let (root, subkey) = parse_path(full_path)?;
    let (key, _) = root
        .create_subkey(subkey)
        .with_context(|| format!("Failed to create/open registry key: {}", full_path))?;
    key.set_value(name, &value)
        .with_context(|| format!("Failed to set DWORD {}\\{}", full_path, name))?;
    Ok(())
}

/// Create (or open) a registry key and set a string (REG_SZ) value.
pub fn set_string(full_path: &str, name: &str, value: &str) -> Result<()> {
    let (root, subkey) = parse_path(full_path)?;
    let (key, _) = root
        .create_subkey(subkey)
        .with_context(|| format!("Failed to create/open registry key: {}", full_path))?;
    key.set_value(name, &value)
        .with_context(|| format!("Failed to set string {}\\{}", full_path, name))?;
    Ok(())
}

/// Read a DWORD value if it exists.
pub fn get_dword(full_path: &str, name: &str) -> Result<Option<u32>> {
    let (root, subkey) = parse_path(full_path)?;
    match root.open_subkey(subkey) {
        Ok(key) => match key.get_value::<u32, _>(name) {
            Ok(v) => Ok(Some(v)),
            Err(_) => Ok(None),
        },
        Err(_) => Ok(None),
    }
}

/// Read a string value if it exists.
pub fn get_string(full_path: &str, name: &str) -> Result<Option<String>> {
    let (root, subkey) = parse_path(full_path)?;
    match root.open_subkey(subkey) {
        Ok(key) => match key.get_value::<String, _>(name) {
            Ok(v) => Ok(Some(v)),
            Err(_) => Ok(None),
        },
        Err(_) => Ok(None),
    }
}
