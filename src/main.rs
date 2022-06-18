use leightbox::app::{ConnectionType, File, HostInfo};
use std::{error::Error, net::Ipv4Addr, time::Duration};

type Result<T> = std::result::Result<T, Box<dyn Error>>;

/*
    -c --client     Start in client-mode
    -s --server     Start in server-mode

    Client specific:
        -t --target     Define target server's IP
        -p --password   Define session's 'password'

    Server specific:
        -f --folder     Define the folder of which contents to share
*/

fn main() -> Result<()> {
    // Tmp before setting up clap
    let tick_rate = Duration::from_millis(200);
    let connection_type: ConnectionType = ConnectionType::Connect;
    let host_info = HostInfo::new(Ipv4Addr::new(127, 0, 0, 1), String::from(""));
    let files: Vec<File> = Vec::new();

    leightbox::init(tick_rate, connection_type, host_info, files)?;

    Ok(())
}
