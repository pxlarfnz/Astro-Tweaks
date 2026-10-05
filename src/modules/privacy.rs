use anyhow::Result;

use crate::core::registry;

pub fn disable_advertising_id() -> Result<()> {
    registry::set_dword(
        r"HKCU\SOFTWARE\Microsoft\Windows\CurrentVersion\AdvertisingInfo",
        "Enabled",
        0,
    )?;

    registry::set_dword(
        r"HKLM\Software\Policies\Microsoft\Windows\AdvertisingInfo",
        "DisabledByGroupPolicy",
        1,
    )?;

    Ok(())
}

pub fn disable_sync_provider_notifications() -> Result<()> {
    registry::set_dword(
        r"HKCU\SOFTWARE\Microsoft\Windows\CurrentVersion\Explorer\Advanced",
        "ShowSyncProviderNotifications",
        0,
    )?;

    Ok(())
}

pub fn disable_nvidia_telemetry() -> Result<()> {
    registry::set_dword(
        r"HKCU\Software\NVIDIA Corporation\NVControlPanel2\Client",
        "OptInOrOutPreference",
        0,
    )?;

    Ok(())
}

pub fn disable_office_telemetry() -> Result<()> {
    registry::set_dword(
        r"HKCU\Software\Policies\Microsoft\office\16.0\common",
        "sendcustomerdata",
        0,
    )?;

    registry::set_dword(
        r"HKCU\Software\Policies\Microsoft\office\common\clienttelemetry",
        "sendtelemetry",
        3,
    )?;

    registry::set_dword(
        r"HKCU\Software\Policies\Microsoft\office\16.0\common",
        "qmenable",
        0,
    )?;

    Ok(())
}

pub fn disable_device_setup_suggestions() -> Result<()> {
    registry::set_dword(
        r"HKCU\SOFTWARE\Microsoft\Windows\CurrentVersion\UserProfileEngagement",
        "ScoobeSystemSettingEnabled",
        0,
    )?;

    Ok(())
}

pub fn disable_dotnet_cli_telemetry() -> Result<()> {
    std::env::set_var(
        "DOTNET_CLI_TELEMETRY_OPTOUT",
        "1",
    );

    Ok(())
}

pub fn disable_input_telemetry() -> Result<()> {
    registry::set_dword(
        r"HKCU\SOFTWARE\Microsoft\InputPersonalization",
        "RestrictImplicitInkCollection",
        1,
    )?;

    registry::set_dword(
        r"HKCU\SOFTWARE\Microsoft\InputPersonalization",
        "RestrictImplicitTextCollection",
        1,
    )?;

    Ok(())
}

pub fn configure_media_player_privacy() -> Result<()> {
    registry::set_dword(
        r"HKCU\SOFTWARE\Microsoft\MediaPlayer\Preferences",
        "AcceptedPrivacyStatement",
        1,
    )?;

    registry::set_dword(
        r"HKCU\SOFTWARE\Microsoft\MediaPlayer\Preferences",
        "UsageTracking",
        0,
    )?;

    Ok(())
}

pub fn disable_app_launch_tracking() -> Result<()> {
    registry::set_dword(
        r"HKCU\SOFTWARE\Microsoft\Windows\CurrentVersion\Explorer\Advanced",
        "Start_TrackProgs",
        0,
    )?;

    Ok(())
}

pub fn disable_online_speech_recognition() -> Result<()> {
    registry::set_dword(
        r"HKCU\SOFTWARE\Microsoft\Speech_OneCore\Settings\OnlineSpeechPrivacy",
        "HasAccepted",
        0,
    )?;

    Ok(())
}

pub fn disable_recall_snapshots() -> Result<()> {
    // Windows 11 Recall policy.
    registry::set_dword(
        r"HKLM\SOFTWARE\Policies\Microsoft\Windows\WindowsAI",
        "DisableAIDataAnalysis",
        1,
    )?;

    Ok(())
}

pub fn disable_tailored_experiences() -> Result<()> {
    registry::set_dword(
        r"HKCU\SOFTWARE\Microsoft\Windows\CurrentVersion\Privacy",
        "TailoredExperiencesWithDiagnosticDataEnabled",
        0,
    )?;

    registry::set_dword(
        r"HKCU\SOFTWARE\Policies\Microsoft\Windows\CloudContent",
        "DisableTailoredExperiencesWithDiagnosticData",
        1,
    )?;

    Ok(())
}

pub fn disable_frequent_apps() -> Result<()> {
    registry::set_dword(
        r"HKCU\SOFTWARE\Microsoft\Windows\CurrentVersion\Policies\Explorer",
        "NoInstrumentation",
        1,
    )?;

    Ok(())
}

pub fn disable_language_list_access() -> Result<()> {
    registry::set_dword(
        r"HKCU\Control Panel\International\User Profile",
        "HttpAcceptLanguageOptOut",
        1,
    )?;

    Ok(())
}

pub fn disable_error_reporting() -> Result<()> {
    registry::set_dword(
        r"HKCU\SOFTWARE\Policies\Microsoft\Windows\Windows Error Reporting",
        "Disabled",
        1,
    )?;

    registry::set_dword(
        r"HKLM\SOFTWARE\Policies\Microsoft\PCHealth\ErrorReporting",
        "DoReport",
        0,
    )?;

    registry::set_dword(
        r"HKLM\SOFTWARE\Policies\Microsoft\Windows\Windows Error Reporting",
        "Disabled",
        1,
    )?;

    registry::set_dword(
        r"HKLM\SOFTWARE\Policies\Microsoft\Windows\Windows Error Reporting",
        "DontShowUI",
        1,
    )?;

    registry::set_dword(
        r"HKLM\SOFTWARE\Policies\Microsoft\Windows\Windows Error Reporting",
        "LoggingDisabled",
        1,
    )?;

    registry::set_dword(
        r"HKLM\SOFTWARE\Policies\Microsoft\Windows\Windows Error Reporting",
        "DontSendAdditionalData",
        1,
    )?;

    registry::set_dword(
        r"HKLM\SOFTWARE\Policies\Microsoft\Windows\DeviceInstall\Settings",
        "DisableSendGenericDriverNotFoundToWER",
        1,
    )?;

    registry::set_dword(
        r"HKLM\SOFTWARE\Policies\Microsoft\Windows\DeviceInstall\Settings",
        "DisableSendRequestAdditionalSoftwareToWER",
        1,
    )?;

    registry::set_dword(
        r"HKLM\Software\Microsoft\Windows\CurrentVersion\Component Based Servicing",
        "DisableWerReporting",
        1,
    )?;

    Ok(())
}

pub fn configure_search_privacy() -> Result<()> {
    let key =
        r"HKCU\SOFTWARE\Microsoft\Windows\CurrentVersion\Search";

    registry::set_dword(
        key,
        "BingSearchEnabled",
        0,
    )?;

    registry::set_dword(
        key,
        "CortanaEnabled",
        0,
    )?;

    Ok(())
}

pub fn disable_user_activity_upload() -> Result<()> {
    let key =
        r"HKLM\SOFTWARE\Policies\Microsoft\Windows\System";

    registry::set_dword(
        key,
        "EnableActivityFeed",
        0,
    )?;

    registry::set_dword(
        key,
        "PublishUserActivities",
        0,
    )?;

    registry::set_dword(
        key,
        "UploadUserActivities",
        0,
    )?;

    Ok(())
}

pub fn apply_privacy_optimizations() -> Result<()> {
    println!("Applying privacy optimizations...");

    disable_advertising_id()?;
    disable_sync_provider_notifications()?;
    disable_nvidia_telemetry()?;
    disable_office_telemetry()?;
    disable_device_setup_suggestions()?;
    disable_dotnet_cli_telemetry()?;
    disable_input_telemetry()?;
    configure_media_player_privacy()?;
    disable_app_launch_tracking()?;
    disable_online_speech_recognition()?;
    disable_recall_snapshots()?;
    disable_tailored_experiences()?;
    disable_frequent_apps()?;
    disable_language_list_access()?;
    disable_error_reporting()?;
    configure_search_privacy()?;
    disable_user_activity_upload()?;

    println!("Privacy optimizations complete.");

    Ok(())
}