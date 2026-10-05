use anyhow::Result;

const SERVICES: &[&str] = &[
    "SysMain",
    "WbioSrvc",
    "PcaSvc",
    "WerSvc",
    "WpnService",
    "WpnUserService",
    "BDESVC",
    "DiagTrack",
    "dmwappushservice",
    "lfsvc",
    "MapsBroker",
    "NetTcpPortSharing",
    "RemoteRegistry",
    "SensorService",
    "SNMPTRAP",
    "TabletInputService",
    "TrkWks",
    "XblAuthManager",
    "XblGameSave",
    "XboxNetApiSvc",
    "OneSyncSvc",
    "PrintWorkflowUserSvc",
    "WalletService",
    "AJRouter",
    "AssignedAccessManagerSvc",
    "ClipSVC",
    "ConsentUxUserSvc",
    "DevicePickerUserSvc",
    "DevicesFlowUserSvc",
    "PimIndexMaintenanceSvc",
    "UnistoreSvc",
    "UserDataSvc",
    "WiaRpc",
    "defragsvc",
    "fhsvc",
    "wbengine",
    "WSearch",
    "wisvc",
    "GpuEnergyDrv",
    "SmsRouter",
    "ALG",
    "QWAVE",
    "IpxlatCfgSvc",
    "icssvc",
    "DusmSvc",
    "edgeupdate",
    "shpamsvc",
    "svsvc",
    "MSiSCSI",
    "Netlogon",
    "CscService",
    "ssh-agent",
    "AppReadiness",
    "tzautoupdate",
    "NfsClnt",
    "SharedRealitySvc",
    "RetailDemo",
    "lltdsvc",
    "diagsvc",
    "DPS",
    "WdiServiceHost",
    "WdiSystemHost",
    "TroubleshootingSvc",
    "DsSvc",
    "FrameServer",
    "FontCache",
    "InstallService",
    "OSRSS",
    "sedsvc",
    "SENS",
    "Themes",
    "MessagingService",
    "CDPUserSvc",
    "BcastDVRUserService",
    "DeviceAssociationBrokerSvc",
    "cbdhsvc",
    "CaptureService",
    "diagnosticshub.standardcollector.service",
    "TapiSrv",
    "FontCache3.0.0.0",
    "WpcMonSvc",
    "SEMgrSvc",
    "PNRPsvc",
    "LanmanWorkstation",
    "WEPHOSTSVC",
    "p2psvc",
    "p2pimsvc",
    "PhoneSvc",
    "Wecsvc",
    "SensorDataService",
    "SensrSvc",
    "perceptionsimulation",
    "StiSvc",
    "WMPNetworkSvc",
    "autotimesvc",
    "edgeupdatem",
    "MicrosoftEdgeElevationService",
];

/*
 * These must never be touched by the optimizer.
 *
 * Defender and Windows Update are intentionally protected here.
 */
const PROTECTED: &[&str] = &[
    "wuauserv",
    "WaaSMedicSvc",
    "UsoSvc",
    "DoSvc",
    "BITS",
    "CryptSvc",
    "WinDefend",
    "WdNisSvc",
    "Sense",
    "SecurityHealthService",
    "wscsvc",
    "WlanSvc",
    "mpssvc",
    "BFE",
    "BluetoothUserService",
    "BTAGService",
    "bthserv",
    "BthAvctpSvc",
    "msiserver",
    "EventLog",
    "DcomLaunch",
    "RpcSs",
    "LSM",
    "vds",
    "nvlddmkm",
    "DisplayEnhancementService",
];

pub fn disable_selected() -> Result<()> {
    for service in SERVICES {
        if PROTECTED.contains(service) {
            continue;
        }

        let script = format!(
            r#"
$s = Get-Service -Name '{}' -ErrorAction SilentlyContinue

if ($s -and $s.StartType -ne "Disabled") {{
    Stop-Service '{}' -Force -ErrorAction SilentlyContinue
    Set-Service '{}' -StartupType Disabled -ErrorAction SilentlyContinue
}}
"#,
            service,
            service,
            service
        );

        if let Err(error) =
            crate::core::command::powershell(&script)
        {
            eprintln!(
                "[SERVICE] {} -> {}",
                service,
                error
            );
        }
    }

    Ok(())
}

/*
 * The source scripts also contained driver Start=4 changes.
 *
 * These are kept as inventory rather than automatically applied because
 * several of these names correspond to fundamental Windows networking,
 * storage, filesystem, and driver components.
 */
pub fn driver_inventory() -> &'static [&'static str] {
    &[
        "acpipagr",
        "AcpiPmi",
        "Beep",
        "CAD",
        "CLFS",
        "CSC",
        "luafv",
        "RasAcd",
        "Rasl2tp",
        "RasPppoe",
        "RasSstp",
        "Tcpip6",
        "tcpipreg",
        "dam",
        "wanarpv6",
        "PEAUTH",
        "QWAVEdrv",
        "cdrom",
        "fileinfo",
    ]
}