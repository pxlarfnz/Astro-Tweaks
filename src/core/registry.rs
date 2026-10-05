```rust
use anyhow::{bail, Result};
use winreg::{
    enums::{HKEY_CURRENT_USER, HKEY_LOCAL_MACHINE},
    RegKey,
};

use windows::Win32::Foundation::CloseHandle;
use windows::Win32::Security::{
    GetTokenInformation,
    TokenElevation,
    TOKEN_ELEVATION,
    TOKEN_QUERY,
};
use windows::Win32::System::Threading::{
    GetCurrentProcess,
    OpenProcessToken,
};

/// Returns whether the current process is running with administrator privileges.
pub fn is_admin() -> Result<bool> {
    unsafe {
        let mut token = Default::default();

        OpenProcessToken(
            GetCurrentProcess(),
            TOKEN_QUERY,
            &mut token,
        )?;

        let mut elevation = TOKEN_ELEVATION::default();
        let mut size = 0u32;

        GetTokenInformation(
            token,
            TokenElevation,
            Some(&mut elevation as *mut _ as *mut _),
            std::mem::size_of::<TOKEN_ELEVATION>() as u32,
            &mut size,
        )?;

        CloseHandle(token);

        Ok(elevation.TokenIsElevated != 0)
    }
}

/// Ensures the program is running as Administrator.
pub fn ensure_admin() -> Result<()> {
    if !is_admin()? {
        bail!("Vivid Tweaks must be run as Administrator.");
    }

    Ok(())
}

/// Opens the requested Windows registry root and returns the subkey path.
///
/// Supported roots:
/// - HKCU
/// - HKLM
fn open_root(path: &str) -> Result<(RegKey, &str)> {
    if let Some(subkey) = path.strip_prefix(r"HKCU\") {
        Ok((RegKey::predef(HKEY_CURRENT_USER), subkey))
    } else if let Some(subkey) = path.strip_prefix(r"HKLM\") {
        Ok((RegKey::predef(HKEY_LOCAL_MACHINE), subkey))
    } else {
        bail!("Unsupported registry root: {}", path);
    }
}

/// Sets a REG_DWORD registry value.
///
/// Example:
/// set_dword(
///     r"HKCU\Software\Example",
///     "Enabled",
///     1,
/// )?;
pub fn set_dword(
    path: &str,
    name: &str,
    value: u32,
) -> Result<()> {
    let (root, subkey) = open_root(path)?;
    let (key, _) = root.create_subkey(subkey)?;

    key.set_value(name, &value)?;

    Ok(())
}

/// Sets a REG_SZ string registry value.
///
/// Example:
/// set_string(
///     r"HKLM\Software\Example",
///     "Priority",
///     "High",
/// )?;
pub fn set_string(
    path: &str,
    name: &str,
    value: &str,
) -> Result<()> {
    let (root, subkey) = open_root(path)?;
    let (key, _) = root.create_subkey(subkey)?;

    key.set_value(name, &value)?;

    Ok(())
}
```
