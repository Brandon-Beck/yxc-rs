use serde;
use serde_json;
#[macro_use]
use serde_derive;

use std::fmt;
use std::net::IpAddr;
//use serde_derive::*;
type NYI_JSON = serde_json::Value;
/*
 * All replies 
 */

/// pg. 5 "API Overview", response_code always returned
/// SuccessReponse contains response data within it.
/// Errors only return the error code.
/// This is a more Rustic abstraction which deviates
/// from Yamaha's API.
/// fn -> Result(YamahaSuccess,YamahaErr)
#[derive(Deserialize, Debug)]
#[serde(tag = "response_code")]
pub enum YamahaResponse {
    SuccessResponse,
    YamahaErr
}
// Just a response code, nothing else
#[derive(Deserialize, Debug)]
pub enum YamahaResponseCode {
    Success,
    YamahaErr
}
pub enum YamahaErr {
    SystemError,
    StreamingError,
    // New or Undocumented Yamaha provided error codes
    UnknownError(u32),
}

/*
 * Errors
 */
#[derive(Deserialize, Debug)]
enum SystemError {
    Initializing = 1,
    InternalError = 2,
    InvalidRequest = 3,
    InvalidParameter = 4,
    Gaurded = 5,
    TimeOut = 6,
    FirmwareUpdating = 99,
}
#[derive(Deserialize, Debug)]
enum StreamingError {
    AccessError = 100,
    OtherErrors = 101,
    WrongUserName = 102,
    WrongPassword = 103,
    AccountExpired = 104,
    AccountDisconnected = 105,
    // FIXME Should we include "to the"?
    AccountNumberReachedLimit = 106,
    ServerMaintenance = 107,
    InvalidAccount = 108,
    LicenseError = 109,
    ReadOnlyMode = 110,
    MaxStations = 111,
    AccessDenied = 112,
}

/// Successfull get/post reply.
/// Contains additional data related to the request
#[derive(Deserialize, Debug)]
pub enum SuccessResponse {
    GetDeviceInfoResponse,
    JustSuccess,
}

/*
 * Request Specific Data structures
 */
#[derive(Deserialize, Debug)]
pub struct GetDeviceInfoResponse {
    model_name: String,
    destination: String,
    // Availible on API >= 1.17
    device_id: Option<String>,
    system_version: f32,
    api_version: f32,
    netmodule_version: String,
    operation_mode: String,
    update_error_code: String,
    // FIXME Reserved. Return value
    // is supposidly a json object,
    // but was omited in the example.
    update_progress: Option<String>,
}

/*
 * Lots of structures related to
 * GetFeaturesResponse
 */

#[derive(Deserialize, Debug)]
pub struct GetFeaturesResponse {
    system: System,
    zone: Vec<Zone>,
    tuner: Tuner,
    netusb: NetUsb,
    distribution: Distribution,
    clock: Option<Clock>,
}

#[derive(Deserialize, Debug)]
pub struct System {
    func_list: Vec<String>,
    zone_num: u16,
    input_list: Vec<Input>,
    range_step: Option<Vec<RangeStep>>,
    speaker_settings: Option<NYI_JSON>,
    ymap_list: Option<Vec<String>>,
    web_control_url: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct Zone {
    id: String,
    zone_b: Option<bool>,
    func_list: Vec<String>,
    input_list: Vec<String>,
    sound_program_list: Option<Vec<String>>,
    tone_control_list: Option<Vec<String>>,
    equalizer_mode_list: Option<Vec<String>>,
    link_control_list: Vec<String>,
    link_audio_delay_list: Option<Vec<String>>,
    range_step: Option<Vec<RangeStep>>,
}
#[derive(Deserialize, Debug)]
pub struct Tuner {
    func_list: Vec<String>,
    range_step: Option<Vec<RangeStep>>,
    preset: Preset,
}
#[derive(Deserialize, Debug)]
pub struct NetUsb {
    func_list: Vec<String>,
    preset: Preset,
    recent_info: RecentInfo,
    play_queue: NYI_JSON,
    mc_playlist: NYI_JSON,
    vtuner_fver: Option<String>,
}
#[derive(Deserialize, Debug)]
pub struct Distribution {
    server_zone_list: Vec<String>
}

#[derive(Deserialize, Debug)]
pub struct Clock {
    func_list: Vec<String>,
    range_step: Vec<RangeStep>,
    alarm_fade_type_num: i32,
    alarm_mode_list: Vec<String>,
    alarm_input_list: Vec<String>,
    alarm_preset_list: Vec<String>,
}
#[derive(Deserialize, Debug)]
pub struct RangeStep {
    id: String,
    min: i32,
    max: i32,
    step: i32,
}
#[derive(Deserialize, Debug)]
pub struct Input {
    id: String,
    distribution_enable: bool,
    rename_enable: bool,
    account_enable: bool,
    play_info_type: String,
}

#[derive(Deserialize, Debug)]
pub struct Preset {
    /// type key renamed preset_type
    /// deviating from API due to name restrictions
    #[serde(rename = "type")]
    preset_type: Option<String>,
    num: i32,
}
#[derive(Deserialize, Debug)]
pub struct RecentInfo {
    num: i32,
}


#[derive(Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum ConnectionType {
    WiredLan,
    WirelessLan,
    WirelessDirect,
    Extend_1,
    Extend_2,
    Extend_3,
    Unknown,
}

#[derive(Deserialize, Debug)]
pub struct MacAdresses {
    wired_lan: String,
    wireless_lan: String,
    wireless_direct: String,
}
#[derive(Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum MusicCastNetworkType {
    Root,
    Node,
    Leaf,
    Standard,
    Unknown,
}
#[derive(Deserialize, Debug)]
pub struct MusicCastNetwork {
    ready: bool,
    device_type: MusicCastNetworkType,
    key: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct WirelessLan {
    ssid: String,
    #[serde(rename = "type")]
    encryption_type: WirelessEncryptionType,
    key: Option<String>,
    ch: u8,
    // Valid values: -1 = link down, 0-100 if connected
    strength: i8,
}
#[derive(Deserialize, Debug)]
pub struct WirelessDirect {
    ssid: String,
    // NOTE only none and wpa2 are currently supported
    #[serde(rename = "type")]
    encryption_type: WirelessEncryptionType,
    key: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct GetNetworkStatusResponse {
    network_name: String,
    connection: ConnectionType,
    dhcp: Option<bool>,
    ip_address: Option<IpAddr>,
    subnet_mask: Option<IpAddr>,
    default_gateway: Option<IpAddr>,
    dns_server_1: Option<IpAddr>,
    dns_server_2: Option<IpAddr>,
    wireless_lan: WirelessLan,
    wireless_direct: WirelessDirect,
    musiccast_network: MusicCastNetwork,
    mac_address: MacAdresses,
    airplay_pin: String,
}

// TODO Solve the mysteries.
// 1. If unspecified old values are discarded or kept?
// 2. What happens when no parameters are passed?
#[derive(Serialize)]
pub struct SetWiredLanParam {
    // NOTE pg. 16, "System"
    // Type is suppose to be string, however,
    // example had JSON value of false (not the string false).
    // Probably realy boolean like in getNetworkStatus.
    // Alternitivly, it may be the IP of dhcp server,
    // and false if fully static.
    dhcp: Option<bool>,
    ip_address: Option<IpAddr>,
    subnet_mask: Option<IpAddr>,
    default_gateway: Option<IpAddr>,
    dns_server_1: Option<IpAddr>,
    dns_server_2: Option<IpAddr>
}
pub type SetWiredLanResponse = YamahaResponseCode;
pub type SetWirelessLanResponse = YamahaResponseCode;


// FIXME There has to be a better/more correct way to do this
#[derive(Deserialize,Serialize,Debug)]
#[serde(rename_all = "snake_case")]
pub enum WirelessEncryptionType {
    /// None renamed to Unencrypted
    #[serde(rename = "none")]
    Unencrypted,
    Wep,
    #[serde(rename = "wpa2_psk(aes)")]
    WPA2_PSK_AES,
    MixedMode
}
#[derive(Serialize)]
pub struct SetWirelessLanParam {
    ssid: Option<String>,
    /// type renamed encryption_type
    #[serde(rename = "type")]
    encryption_type: Option<WirelessEncryptionType>,
    // FIXME Ensure encryption_type is not none if key is set
    key: Option<String>,
    dhcp: Option<bool>,
    ip_address: Option<IpAddr>,
    subnet_mask: Option<IpAddr>,
    default_gateway: Option<IpAddr>,
    dns_server_1: Option<IpAddr>,
    dns_server_2: Option<IpAddr>
}







