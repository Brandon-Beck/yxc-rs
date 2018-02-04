
use yamaha::responses::*;

use std;
use std::io;
use futures::{Future, Stream};
use hyper::Client;
use tokio_core::reactor::Core;
use serde_json::Value;
use serde::de::Deserialize;
use serde::de::DeserializeOwned;
use serde;
use serde_json;
use yamaha::MusicCast;
use yamaha::responses::*;
type GetError = Box<std::error::Error>;
type GetResult<T> = Result<T,Box<std::error::Error>>;
impl MusicCast {
    /// pg. 5 "API Overview" Base URL "<BaseURL>" shown in
    /// URI is http://{host}/YamahaExtendedControl
    fn baseurl(&self) -> String {
        format!("http://{host}/YamahaExtendedControl", host = &self.ip)
    }
    pub fn subpath_to_url(&self, subpath: &str) -> String {
        format!("{baseurl}/{subpath}",baseurl = &self.baseurl(),subpath = subpath)
    }
    // NOTE I am not realy sure what magic is going on here with lifetimes.
    //pub fn get(&self, subpath: &str) -> Result<String, Box<::std::error::Error>> {
    pub fn get<T: DeserializeOwned>(&self, subpath: &str) -> Result<T, GetError> {
        let mut core = Core::new()?;
        let client = Client::new(&core.handle());
    
        let uri = self.subpath_to_url(subpath).parse()?;
        let work = client.get(uri).and_then(|res| {
            println!("Response: {}", res.status());
        
            res.body().concat2().and_then(move |body| {
                let v: T = serde_json::from_slice(&body).expect("A LOGICAL error?");
                /*let v: T = serde_json::from_slice(&body).map_err(|err| {
                    io::Error::new(
                        io::ErrorKind::Other,
                        err
                    )
                }).expect("SerdeMapErr Failed Us");*/
                return Ok(v)
            })
        });
        let v = core.run(work).expect("Worker Failed us");
        Ok(v)
    }
    // /// pg. 7 "System - getDeviceInfo"
    //fn get_device_info(&self) -> Result<GetDeviceInfoResponse,GetError> { self.get("v1/system/getDeviceInfo") }
    fn get_device_info(&self) -> GetResult<GetDeviceInfoResponse> { self.get("v1/system/getDeviceInfo") }
    fn get_features(&self) -> GetResult<GetFeaturesResponse> { self.get("v1/system/getFeatures") }
    fn get_network_status(&self) -> GetResult<GetNetworkStatusResponse> { self.get("v1/system/getNetworkStatus") }
    fn set_wired_lan(&self, params: SetWiredLanParam) -> GetResult<SetWiredLanResponse> { self.get("v1/system/get") }
}

#[cfg(test)]
mod tests {
    use yamaha::MusicCast;
    use yamaha::responses::*;
    use serde::Serialize;
    use serde::Deserialize;
    use serde_json::Value;
    fn discover_first() -> Option<MusicCast> {
        match MusicCast::discover() {
            Ok(val) => if val.len() > 0 { return Some(val[0].clone()) } else { return None },
            _ => return None,
        };
        return None;
    }
    #[test]
    fn inten_test() {
        let m = discover_first().expect("Failed to find any yamaha devices");
        //println!("ACTUAL {:?}", &m.get("v1/system/getDeviceInfo").expect("Failure getting info #1"));
        //let v: GetDeviceInfoResponse = serde_json::from_str(&m.get("v1/system/getDeviceInfo")
        //                                                    .expect("Failure getting info #2") )
        //    .expect("Failure parsing info as json");
        //let v: GetDeviceInfoResponse = m.get("v1/system/getDeviceInfo").expect("Failure Getting Device Info");
        println!("{:?}",m.get_device_info().expect("Failure Getting Device Info"));
        //println!("{:?}",m.get::<Value>("v1/system/getFeatures").expect("NO THINGS"));
        println!("{:?}",m.get_features().expect("Failure Getting Features"));
        println!("{:?}",m.get_network_status().expect("Failure Getting Network Status"));
    }
}



