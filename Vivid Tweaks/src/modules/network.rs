use anyhow::Result;
use winreg::{
    enums::*,
    RegKey,
};

pub fn apply() -> Result<()> {
    println!("[NETWORK] Applying TCP/AFD tuning...");

    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);

    let (afd, _) =
        hklm.create_subkey(
            r"SYSTEM\CurrentControlSet\Services\AFD\Parameters"
        )?;

    afd.set_value(
        "FastSendDatagramThreshold",
        &1500u32,
    )?;

    afd.set_value(
        "FastCopyReceiveThreshold",
        &1500u32,
    )?;

    let (tcp, _) =
        hklm.create_subkey(
            r"SYSTEM\CurrentControlSet\Services\Tcpip\Parameters"
        )?;

    tcp.set_value(
        "TcpMaxDataRetransmissions",
        &3u32,
    )?;

    tcp.set_value(
        "MaxUserPort",
        &65534u32,
    )?;

    tcp.set_value(
        "TcpTimedWaitDelay",
        &30u32,
    )?;

    tcp.set_value(
        "DefaultTTL",
        &64u32,
    )?;

    tcp.set_value(
        "Tcp1323Opts",
        &1u32,
    )?;

    tcp.set_value(
        "EnableICMPRedirect",
        &0u32,
    )?;

    tcp.set_value(
        "DisableIPSourceRouting",
        &2u32,
    )?;

    /*
     * Netsh stack.
     */
    let commands: &[&[&str]] = &[
        &["int", "tcp", "set", "global", "rss=enabled"],
        &["int", "tcp", "set", "global", "rsc=disabled"],
        &["int", "tcp", "set", "global", "autotuninglevel=restricted"],
        &["int", "tcp", "set", "global", "ecncapability=enabled"],
        &["int", "tcp", "set", "global", "timestamps=disabled"],
        &["int", "tcp", "set", "global", "initialRto=2000"],
        &[
            "int",
            "tcp",
            "set",
            "global",
            "nonsackrttresiliency=disabled",
        ],
        &[
            "int",
            "tcp",
            "set",
            "global",
            "maxsynretransmissions=2",
        ],
        &["int", "tcp", "set", "global", "dca=enabled"],
        &["int", "tcp", "set", "global", "netdma=enabled"],
        &["int", "tcp", "set", "heuristics", "disabled"],
        &[
            "int",
            "ip",
            "set",
            "global",
            "taskoffload=disabled",
        ],
        &[
            "int",
            "ip",
            "set",
            "global",
            "neighborcachelimit=4096",
        ],
        &[
            "int",
            "tcp",
            "set",
            "supplemental",
            "Internet",
            "congestionprovider=ctcp",
        ],
        &[
            "int",
            "isatap",
            "set",
            "state",
            "disabled",
        ],
        &[
            "int",
            "teredo",
            "set",
            "state",
            "disabled",
        ],
    ];

    for command in commands {
        if let Err(error) =
            crate::core::command::netsh(command)
        {
            eprintln!(
                "[NETSH] {:?} -> {}",
                command,
                error
            );
        }
    }

    /*
     * Adapter-level operations.
     */
    crate::core::command::powershell(
        r#"
$adapters =
    Get-NetAdapter |
    Where-Object {$_.HardwareInterface}

foreach ($adapter in $adapters) {

    try {
        Disable-NetAdapterVmq `
            -Name $adapter.Name `
            -Confirm:$false `
            -ErrorAction SilentlyContinue
    } catch {}

    try {
        Disable-NetAdapterUro `
            -Name $adapter.Name `
            -Confirm:$false `
            -ErrorAction SilentlyContinue
    } catch {}

    try {
        Disable-NetAdapterUso `
            -Name $adapter.Name `
            -Confirm:$false `
            -ErrorAction SilentlyContinue
    } catch {}
}
"#,
    )?;

    /*
     * Interface TCP latency settings.
     */
    crate::core::command::powershell(
        r#"
Get-NetIPConfiguration |
Where-Object {$_.IPv4DefaultGateway} |
ForEach-Object {

    $guid = $_.NetAdapter.InterfaceGuid

    if ($guid) {

        $path =
            "HKLM:\SYSTEM\CurrentControlSet\Services\Tcpip\Parameters\Interfaces\$guid"

        New-Item $path -Force |
            Out-Null

        Set-ItemProperty $path TCPNoDelay 1
        Set-ItemProperty $path TcpAckFrequency 1
        Set-ItemProperty $path TcpDelAckTicks 0
    }
}
"#,
    )?;

    Ok(())
}