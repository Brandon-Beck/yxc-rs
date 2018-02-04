//mod discovery;

/**
 * Yamaha MusicCast Device Discovery and Control API.
 **/

use discovery;
use std::str::Split;
use std::fmt;
use std::fmt::Display;
use ssdp::message::SearchResponse;
use ssdp::header::HeaderRef;
use yamaha::responses::*;
use yamaha::MusicCast;

pub type MusicCastDiscoverResult = Result<Vec<MusicCast>,&'static str>;
const YAMAHA_UUID_PREFIX: &str = "9ab0c000-f668-11de-9976";


/// Musiccast Discovery via SSDP.
impl MusicCast {
    /// Returns the Friendly Name of the Yamaha device if present in the SSDP respone message.
    /// Else returns None.
    fn parse_friendly_name(msg: &SearchResponse) -> Option<String> {
        match msg.get_raw("X-ModelName")  {
            // I do not know why I get an array of Vecs, but I do.
            Some(raw) => for ref u in raw {
                let uname = String::from_utf8(u.to_vec()).expect("Could not Convert vec to string");
                let split: Vec<&str> = uname.split(":").collect();
                if split.len() >= 3 && split[2].trim() != "" {
                    //friendly_name = Some(split[2].trim().to_string());
                    return Some(split[2].trim().to_string());
                }
            },
            None => return None,
        };
        return None;
    }
    /// Returns the USN from the given message if found, else returns Err.
    fn parse_usn(msg: &SearchResponse) -> Result<String,&'static str> {
        match msg.get_raw("USN")  {
            // I do not know why I get an array of Vecs, but I do.
            Some(raw) => for u in raw {
                return Ok(String::from_utf8(u.to_vec()).expect("Could not Convert vec to string"));
            },
            None => return Err("Failed to get USN"),
        };
        return Err("Failed to get USN");
    }
    /// Returns true if uuid starts with Yamaha's uuid prefix.
    fn filter_uuid(uuid: &str) -> bool {
        if !uuid.starts_with(YAMAHA_UUID_PREFIX) { 
            return false;
        }
        return true;
    }
    // Should probably be Result, since no uuid is definitly an error
    // of ours or the responding server
    fn parse_uuid(usn: &str) -> Option<String> {
        let split: Vec<&str> = usn.split(":").collect();
        //println!("{:?}", split);
        if split.len() >= 2 && split[0] == "uuid" {
            //return split[1].to_string();
            return Some(split[1].to_string());
        }
        return None;
    }
    /// Discovers all MusicCast devices.
    pub fn discover() -> MusicCastDiscoverResult {
        let mut results: Vec<MusicCast> = Vec::new();
        for (msg,src) in discovery::discover().unwrap() {
            //println!("{:?} {}", msg, src);
            let usn: String = MusicCast::parse_usn(&msg).unwrap();
            let uuid: String;
            match MusicCast::parse_uuid(&usn) {
                Some(a_uuid) => uuid = a_uuid,
                None => continue,
            }
            if !MusicCast::filter_uuid(&uuid) {
                continue;
            }
            let mut friendly_name: Option<String> = MusicCast::parse_friendly_name(&msg);
            results.push(
                MusicCast {
                    friendly_name,
                    uuid,
                    ip: src.ip()
                }
            );
        }
        return Ok(results)
    }
    //fn discover_by_urn() -> MusicCastDiscoverResult {}
    /// Discovers all MusicCast Devices, but filters out all but those with a matching
    /// Friendly Name. Friendly Name is generaly set by the end user, and is often much more
    /// human friendly than IP or UUID, but also has a greater chance of collisions as it is not
    /// explicitly required to be a Unique Identifier.
    pub fn discover_by_friendly_name(wanted_name: String) -> MusicCastDiscoverResult {
        let mut results: Vec<MusicCast> = Vec::new();
        for m in MusicCast::discover().unwrap() {
            match m {
                MusicCast { friendly_name: Some(ref friendly_name), .. } if *friendly_name == wanted_name => {
                    results.push(m.clone());
                },
                MusicCast { .. } => {
                    continue
                },
            }
        }
        return Ok(results);
    }
}

impl Display for MusicCast {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        return match self {
            &MusicCast {friendly_name: Some(ref friendly_name), .. } =>
                write!(fmt, "{}", friendly_name.as_str()),
            _ => write!(fmt, "{}", self.uuid.as_str()),
        }
    }
}



/// Status and control API. Implemented closly to
/// Yamaha Extended Control API Specification (Basic).
/// Camel Case functions converted to snake case.
impl MusicCast {
}


#[cfg(test)]
mod tests {
    use yamaha::*;
    // FIXME These tests require human
    // interaction and verification
    #[test]
    #[ignore]
    fn discover_all() {
        // Error if cannot listen on interfaces
        for m in MusicCast::discover().unwrap() {
            println!("Found Yamaha Device {:?}",m);
        }
    }
/*    #[test]
    fn discover_by_uuid() {
        // Friendly Names may collide.
        for m in MusicCast::discover_by_uuid(
            "").unwrap() {
            println!("Found Yamaha Device {:?}",m);
        }
    }*/
    #[test]
    #[ignore]
    fn discover_by_friendly_name() {
        // Friendly Names may collide.
        for m in MusicCast::discover_by_friendly_name(
            "Living Room Amplifier".to_string()).unwrap() {
            println!("Found Yamaha Device {:?}",m);
        }
    }
   /* #[test]
    fn inputs() {
        let m = MusicCast::new();
    }*/
}



