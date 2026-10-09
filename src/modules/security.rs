use anyhow::Result;

pub fn audit() -> Result<()> {
    println!();
    println!("======================================");
    println!("       ASTRO SECURITY AUDIT");
    println!("======================================");

    crate::core::command::powershell(
        r#"
Write-Host "Device Guard / VBS status:"
try {
    Get-CimInstance Win32_DeviceGuard |
        Select-Object `
            VirtualizationBasedSecurityStatus,
            SecurityServicesConfigured,
            SecurityServicesRunning |
        Format-List
} catch {
    Write-Host "Device Guard information unavailable."
}

Write-Host ""
Write-Host "Process mitigation policy:"
try {
    Get-ProcessMitigation -System |
        Format-List
} catch {
    Write-Host "Process mitigation information unavailable."
}

Write-Host ""
Write-Host "Windows Defender services:"
Get-Service |
Where-Object {
    $_.Name -in @(
        "WinDefend",
        "WdNisSvc",
        "SecurityHealthService"
    )
} |
Select-Object Name, Status, StartType |
Format-Table -AutoSize

Write-Host ""
Write-Host "Windows Update services:"
Get-Service |
Where-Object {
    $_.Name -in @(
        "wuauserv",
        "UsoSvc",
        "WaaSMedicSvc",
        "BITS"
    )
} |
Select-Object Name, Status, StartType |
Format-Table -AutoSize
"#,
    )
}
