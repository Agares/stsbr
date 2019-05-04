use crate::block::{Block, BlockError, BlockState, ClickEvent};
use nix::sys::socket::{AddressFamily, SockAddr};

pub struct NetworkInterface {}

impl Block for NetworkInterface {
    fn current_state(&self) -> Result<BlockState, BlockError> {
        let mut addrs = nix::ifaddrs::getifaddrs()
            .map_err(|_| BlockError::new("Failed to get interface addresses".to_string()))?;
        let iface = addrs.find(|a| {
            a.interface_name == "eno1"
                && a.address
                    .map_or(false, |a| a.family() == AddressFamily::Inet)
        });

        match iface {
            Some(i) => match i.address {
                Some(SockAddr::Inet(address)) => Ok(BlockState::new(address.ip().to_string())),
                Some(_) => Err(BlockError::new("Wrong address type".to_string())),
                None => Err(BlockError::new("No address".to_string())),
            },
            None => Err(BlockError::new("Failed to find interface".to_string())),
        }
    }

    fn handle_click(&self, _event: ClickEvent) {}
}

impl NetworkInterface {
    pub fn new() -> Self {
        NetworkInterface {}
    }
}
