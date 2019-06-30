use crate::block::{Block, BlockError, BlockState, ClickEvent, Icon};
use nix::sys::socket::{SockAddr, AddressFamily};

pub struct NetworkInterface {
    interface: String,
}

impl From<nix::Error> for BlockError {
    fn from(e: nix::Error) -> Self {
        BlockError::new(format!("Failed to find network interface: {:?}", e))
    }
}

impl Block for NetworkInterface {
    fn current_state(&mut self) -> Result<BlockState, BlockError> {
        let mut addrs = nix::ifaddrs::getifaddrs()?;

        let iface = addrs.find(
            |a| a.interface_name == self.interface && a.address.map_or(false, |b| b.family() == AddressFamily::Inet)
        );

        match iface {
            Some(i) => match i.address {
                Some(SockAddr::Inet(address)) => {
                    Ok(BlockState::new(format!("{} {}", Icon::Globe, address.ip())))
                }
                Some(address) => Err(BlockError::new(format!("Wrong address type: {:?}", address))),
                None => Err(BlockError::new("No address".to_string())),
            },
            None => Err(BlockError::new(
                "Failed to find network interface".to_string(),
            )),
        }
    }

    fn handle_click(&self, _event: ClickEvent) {}
}

impl NetworkInterface {
    pub fn new(interface: String) -> Self {
        NetworkInterface { interface }
    }
}
