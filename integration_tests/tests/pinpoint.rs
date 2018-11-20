#![cfg(feature = "pinpoint")]

extern crate rusoto_core;
extern crate rusoto_pinpoint;
extern crate env_logger;

use rusoto_pinpoint::{Pinpoint, PinpointClient, GetAppsRequest};
use rusoto_core::Region;

#[test]
fn should_get_apps() {
    let _ = env_logger::try_init();
    let client = PinpointClient::new(Region::UsEast1);
    let request = GetAppsRequest::default();

    println!("{:?}", client.get_apps(request).sync().unwrap());
}
