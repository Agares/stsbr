use crate::data_source::{DataSource, DataSourceState};
use nix::sys::socket::{AddressFamily, SockAddr};

pub struct NetworkInterface {}

impl DataSource for NetworkInterface {
    fn current_state(&self) -> DataSourceState {
        let mut addrs = nix::ifaddrs::getifaddrs().unwrap(); // todo return Err()
        let iface = addrs.find(|a| a.interface_name == "eno1" && a.address.unwrap().family() == AddressFamily::Inet).unwrap();
        let ip = match iface.address.unwrap() {
            SockAddr::Inet(address) => address.ip().to_string(),
            _ => panic!("Wrong address type")
        };

        DataSourceState::new(ip)
    }
}

impl NetworkInterface {
    pub fn new() -> Self {
        NetworkInterface {}
    }
}