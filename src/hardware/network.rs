use anyhow::Result;

use crate::core::command::powershell_quiet;

pub fn physical_adapters() -> Result<Vec<String>> {
    let output = powershell_quiet(
        r#"
Get-NetAdapter |
    Where-Object {$_.HardwareInterface} |
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