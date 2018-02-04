use ssdp;
use ssdp::header::{HeaderMut, Man, MX, ST};
use ssdp::message::{SearchRequest, SearchResponse, Multicast};
use ssdp::FieldMap;

/*
 * We try to generate as little noise as possible durring discovery.
 * Unfortunatly, not all ssdp servers respond correctly,
 * eg. Philips Hue Bridge v2.0 currently seems to
 * respond to all request, regardless of specifiers.
 */

pub type DiscoverResult = Result<ssdp::SSDPReceiver<ssdp::message::SearchResponse>, ssdp::SSDPError>;
pub fn discover() -> DiscoverResult {
    // Create Our Search Request
    let mut request = SearchRequest::new();

    // Set Our Desired Headers (Not Verified By The Library)
    request.set(Man);
    request.set(MX(5));
    request.set(ST::All);

    // Iterate Over Streaming Responses
    return request.multicast();
}
pub fn discover_by(filter: String) -> DiscoverResult {
    // Create Our Search Request
    let mut request = SearchRequest::new();

    // Set Our Desired Headers (Not Verified By The Library)
    request.set(Man);
    request.set(MX(5));
    request.set(ST::Target(FieldMap::new(filter.as_str()).unwrap()));

    // Iterate Over Streaming Responses
    return request.multicast();
}

#[cfg(test)]
mod tests {
    use discovery::*;
    // FIXME These tests require human
    // interaction and verification
    #[test]
    #[ignore]
    fn can_discover() {
        discover();
    }
}
