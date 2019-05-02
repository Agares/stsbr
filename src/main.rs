use crate::data_source::{DataSource, DataSourceState};
use crate::i3bar::{get_header_json, sources_to_json};
use chrono::Local;
use std::time::Duration;
use std::os::raw::c_double;
use nix::sys::socket::{AddressFamily, SockAddr};
use libpulse_binding::context::introspect::SinkInfo;
use libpulse_binding::callbacks::ListResult::Item;
use libpulse_binding::mainloop::threaded::Mainloop;
use libpulse_binding::context::Context;
use libpulse_binding::callbacks::ListResult;
use std::sync::Mutex;

#[macro_use]
extern crate lazy_static;

mod data_source;
mod i3bar;

struct DateTime {}
impl DataSource for DateTime {
    fn current_state(&self) -> DataSourceState {
        let now: chrono::DateTime<Local> = Local::now();
        DataSourceState::new(now.format("%Y-%m-%d %T").to_string())
    }
}

struct SystemLoad {}
impl DataSource for SystemLoad {
    fn current_state(&self) -> DataSourceState {
        let mut load_averages: [c_double; 1] = [0f64];
        let received = unsafe { libc::getloadavg(load_averages.as_mut_ptr(), 1) };

        if received != 1 {
            // todo return Err()
            panic!("Cannot get load average!");
        }

        let load = load_averages[0];

        DataSourceState::new(format!("{}", load))
    }
}

struct NetworkInterface {}
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

struct FreeDiskSpace {}
impl DataSource for FreeDiskSpace {
    fn current_state(&self) -> DataSourceState {
        let stats = nix::sys::statvfs::statvfs("/").unwrap();

        let bytes_free = stats.blocks_available() * stats.block_size();
        let gigabytes_free = bytes_free as f32 / (1024.0f32.powi(3));
        DataSourceState::new(format!("{:.2} GB", gigabytes_free))
    }
}

struct MediaPlayer {}
impl DataSource for MediaPlayer {
    fn current_state(&self) -> DataSourceState {
        let bus = mpris::PlayerFinder::new().unwrap();
        let all_players = bus.find_all().unwrap();
        let player = all_players.first().unwrap();
        let metadata = player.get_metadata().unwrap();
        let formatted = format!(
            "{} - {}",
            metadata.artists().unwrap().first().unwrap(),
            metadata.title().unwrap()
        );

        DataSourceState::new(formatted)
    }
}

struct Volume {
    sink_info:Mutex<Option<String>>
}

impl DataSource for Volume {
    fn current_state(&self) -> DataSourceState {
        let info = self.sink_info.lock().unwrap().clone();

        if let Some(x) = info {
            DataSourceState::new(x)
        } else {
            DataSourceState::new("???".to_string())
        }
    }
}

impl Volume {
    fn callback_sink_info(&self, info:ListResult<&SinkInfo>) {
        match info {
            Item(x) => {
                let mut reference = self
                    .sink_info
                    .lock()
                    .unwrap();

                *reference = Some((*x).volume.print())
            }
            _ => {}
        }
    }
}

lazy_static! {
    static ref V:Volume = {
        Volume { sink_info: Mutex::new(None)}
    };
}

fn main() {
    let sources: Vec<&DataSource> = vec![
        &(*V),
        &MediaPlayer {},
        &FreeDiskSpace {},
        &NetworkInterface {},
        &SystemLoad {},
        &DateTime {},
    ];

    let mut mainloop = Mainloop::new().unwrap();
    mainloop.start();

    let mut ctx = Context::new(&mainloop, "stsbr").unwrap();
    ctx.connect(None, libpulse_binding::context::flags::NOFLAGS, None);

    loop {
        match ctx.get_state() {
            libpulse_binding::context::State::Ready => {break;},
            libpulse_binding::context::State::Failed
            | libpulse_binding::context::State::Terminated => {
                panic!("Failed to connect to pulse");
            },
            _ => {}
        }
    }

    println!("{}", get_header_json(false));
    println!("[");

    loop {
        ctx.introspect().get_sink_info_by_name("@DEFAULT_SINK@", |info| V.callback_sink_info(info));
        println!("{},", sources_to_json(&sources));

        std::thread::sleep(Duration::from_millis(200));
    }
}
