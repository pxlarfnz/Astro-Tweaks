use anyhow::Result;

use crate::core::command::powershell_quiet;

pub fn has_nvidia() -> Result<bool> {
    let result = powershell_quiet(
        r#"
(Get-CimInstance Win32_VideoController |
    Where-Object {$_.Name -match "NVIDIA"} |
    Measure-Object).Count
"#,
    )?;

    Ok(result
        .trim()
        .parse::<u32>()
        .unwrap_or(0) > 0)
}

pub fn adapter_names() -> Result<Vec<String>> {
    let output = powershell_quiet(
        r#"
Get-CimInstance Win32_VideoController |
    Select-Object -ExpandProperty Name
"#,
    )?;

    Ok(output
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .map(str::to_owned)
        .collect())
}