use anyhow::Result;

pub fn trim_working_sets() -> Result<()> {
    println!("[MEMORY] Trimming process working sets...");

    crate::core::command::powershell(
        r#"
Get-Process | ForEach-Object {
    try {
        $type = @'
using System;
using System.Runtime.InteropServices;

public static class AstroWorkingSet
{
    [DllImport("psapi.dll")]
    public static extern bool EmptyWorkingSet(IntPtr hProcess);
}
'@

        Add-Type `
            -TypeDefinition $type `
            -ErrorAction SilentlyContinue

        [AstroWorkingSet]::EmptyWorkingSet($_.Handle) |
            Out-Null
    }
    catch {}
}
"#,
    )
}
