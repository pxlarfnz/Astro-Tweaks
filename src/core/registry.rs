use anyhow::{bail, Result};

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

pub fn ensure_admin() -> Result<()> {
    if !is_admin()? {
        bail!("Vivid Tweaks must be run as Administrator.");
    }

    Ok(())
}