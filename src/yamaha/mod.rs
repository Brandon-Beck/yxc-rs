/**
 * Yamaha MusicCast Device Discovery and Control API.
 **/

/// A MusicCast device. 

pub mod discovery;
pub mod responses;
pub mod control;
pub use self::discovery::*;
use std::net::IpAddr;
/// A MusicCast device. 
#[derive(Debug, Clone)]
pub struct MusicCast {
    friendly_name: Option<String>,
    uuid: String,
    ip: IpAddr,
}
/*
#[derive(Debug, Clone)]
pub struct MusicCast {
    friendly_name: Option<String>,
    uuid: String,
    ip: IpAddr,
}


*/
