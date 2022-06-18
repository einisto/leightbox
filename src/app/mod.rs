pub mod ui;

use crossterm::event::KeyCode;
use std::{fmt, net::Ipv4Addr};
use tui::widgets::ListState;

#[derive(Debug, Clone, Copy)]
pub enum ClientStatus {
    Idle,
    Downloading,
    Disconnected,
}

#[derive(PartialEq, Eq, Debug)]
pub enum ConnectionType {
    Connect,
    Host,
    Unset,
}

impl fmt::Display for ConnectionType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

pub struct Client {
    ip: Ipv4Addr,
    status: ClientStatus,
}

impl Client {
    pub fn new(ip: Ipv4Addr, status: ClientStatus) -> Self {
        Client { ip, status }
    }

    //pub fn update_status(&mut self) {}
}

pub struct HostInfo {
    ip: Ipv4Addr,
    password: String,
}

impl HostInfo {
    pub fn new(ip: Ipv4Addr, password: String) -> Self {
        HostInfo { ip, password }
    }
}

#[derive(Clone)]
pub struct File {
    name: String,
    size: u64,
    is_selected: bool,
}

impl File {
    pub fn new(name: String, size: u64) -> Self {
        File {
            name,
            size,
            is_selected: false,
        }
    }
}

impl fmt::Display for File {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

pub struct StatefulFileList {
    state: ListState,
    items: Vec<File>,
}

impl StatefulFileList {
    pub fn new(items: Vec<File>) -> Self {
        let mut state = ListState::default();
        if items.len() > 0 {
            state.select(Some(0));
        }

        StatefulFileList { state, items }
    }

    pub fn next(&mut self) {
        if self.items.len() > 0 {
            let i = match self.state.selected() {
                Some(i) => {
                    if i >= self.items.len() - 1 {
                        0
                    } else {
                        i + 1
                    }
                }
                None => 0,
            };

            self.state.select(Some(i))
        }
    }

    pub fn prev(&mut self) {
        if self.items.len() > 0 {
            let i = match self.state.selected() {
                Some(i) => {
                    if i == 0 {
                        self.items.len() - 1
                    } else {
                        i - 1
                    }
                }
                None => 0,
            };

            self.state.select(Some(i))
        }
    }

    pub fn unselect(&mut self) {
        self.state.select(None);
    }

    pub fn push(&mut self, value: File) {
        self.items.push(value);
    }

    pub fn get_selected(&mut self, i: usize) -> Option<File> {
        let file = self.items.get_mut(i).unwrap();
        if file.is_selected {
            None
        } else {
            file.is_selected = true;
            Some(file.clone())
        }
    }
}

pub struct App<'a> {
    pub title: &'a str,
    pub should_quit: bool,
    pub connection_type: ConnectionType,
    pub connected_clients: Vec<Client>,
    pub host_info: HostInfo,
    pub available_files: StatefulFileList,
    pub downloading_files: Vec<File>,
    pub finished_files: Vec<File>,
}

impl<'a> App<'a> {
    pub fn new(
        title: &'a str,
        connection_type: ConnectionType,
        host_info: HostInfo,
        shared_files: Vec<File>,
    ) -> Self {
        App {
            title,
            should_quit: false,
            connection_type,
            connected_clients: Vec::new(),
            host_info,
            available_files: StatefulFileList::new(shared_files),
            downloading_files: Vec::new(),
            finished_files: Vec::new(),
        }
    }

    pub fn match_keycode(&mut self, keycode: KeyCode) {
        match keycode {
            KeyCode::Char(c) => match c {
                'q' => self.should_quit = true,
                'j' => self.available_files.next(),
                'k' => self.available_files.prev(),
                _ => {}
            },
            KeyCode::Enter => match self.available_files.state.selected() {
                Some(i) => match self.available_files.get_selected(i) {
                    Some(file) => self.downloading_files.push(file),
                    None => {}
                },
                None => {}
            },
            _ => {}
        }
    }

    //pub fn on_tick(&mut self) {}
}
