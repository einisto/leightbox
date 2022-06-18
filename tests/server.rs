use leightbox::app::{Client, ClientStatus, ConnectionType, HostInfo};
use rand::{thread_rng, Rng};
use std::{error::Error, net::Ipv4Addr, time::Duration};

type Result<T> = std::result::Result<T, Box<dyn Error>>;

#[test]
#[ignore]
fn setup_server() -> Result<()> {
    let connection_type = ConnectionType::Host;
    let host_info = HostInfo::new(Ipv4Addr::new(127, 0, 0, 1), String::from("abcd123"));
    let files = Vec::new();
    let tick_rate = Duration::from_millis(200);
    let clients = generate_data();

    let mut terminal = leightbox::setup_crossterm()?;
    let mut app = leightbox::create_app(connection_type, host_info, files);

    app.connected_clients.extend(clients);

    leightbox::app_loop(&mut terminal, app, tick_rate)?;

    leightbox::restore_terminal(terminal)?;

    Ok(())
}

fn generate_data() -> Vec<Client> {
    let mut rng = thread_rng();
    let mut clients: Vec<Client> = Vec::new();
    let status_list: [ClientStatus; 3] = [
        ClientStatus::Idle,
        ClientStatus::Downloading,
        ClientStatus::Disconnected,
    ];

    for _ in 1..5 {
        let ip = Ipv4Addr::new(
            rng.gen_range(1..255),
            rng.gen_range(1..255),
            rng.gen_range(1..255),
            rng.gen_range(1..255),
        );
        let status = &status_list[rng.gen_range(0..3)];

        clients.push(Client::new(ip, status.clone()));
    }

    clients
}
