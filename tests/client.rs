use leightbox::app::{ConnectionType, File, HostInfo};
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use std::{error::Error, net::Ipv4Addr, time::Duration};

type Result<T> = std::result::Result<T, Box<dyn Error>>;

#[test]
fn setup_client() -> Result<()> {
    let connection_type = ConnectionType::Connect;
    let host_info = HostInfo::new(Ipv4Addr::new(127, 0, 0, 1), String::from("abcd123"));
    let files: Vec<File> = generate_data();
    let tick_rate = Duration::from_millis(200);

    let mut terminal = leightbox::setup_crossterm()?;
    let app = leightbox::create_app(connection_type, host_info, files);

    leightbox::app_loop(&mut terminal, app, tick_rate)?;

    leightbox::restore_terminal(terminal)?;

    Ok(())
}

fn generate_data() -> Vec<File> {
    let mut rng = thread_rng();
    let mut files: Vec<File> = Vec::new();

    for _ in 1..20 {
        let filename: String = thread_rng()
            .sample_iter(&Alphanumeric)
            .take(8)
            .map(char::from)
            .collect();
        let filesize = rng.gen_range(0..123456);

        files.push(File::new(format!("{}.jpeg", filename), filesize));
    }

    files
}
