use super::{App, Client, ConnectionType, File};
use tui::{
    backend::Backend,
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Span, Spans, Text},
    widgets::{Block, BorderType, Borders, Cell, List, Row, Table},
    widgets::{ListItem, Paragraph},
    Frame,
};

pub fn draw<B: Backend>(f: &mut Frame<B>, app: &mut App) {
    let size = f.size();

    let chunks = Layout::default()
        .margin(1)
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Length(3),
                Constraint::Min(10),
                Constraint::Length(3),
            ]
            .as_ref(),
        )
        .split(size);

    match app.connection_type {
        ConnectionType::Connect => {
            let client_chunks = Layout::default()
                .margin(2)
                .direction(Direction::Horizontal)
                .constraints(
                    [
                        Constraint::Ratio(1, 3),
                        Constraint::Ratio(1, 3),
                        Constraint::Ratio(1, 3),
                    ]
                    .as_ref(),
                )
                .split(chunks[1]);

            f.render_stateful_widget(
                filemenu(&app.available_files.items, "Available"),
                client_chunks[0],
                &mut app.available_files.state,
            );
            f.render_widget(
                filemenu(&app.downloading_files, "Downloading"),
                client_chunks[1],
            );
            f.render_widget(filemenu(&app.finished_files, "Finished"), client_chunks[2]);
        }
        ConnectionType::Host => {
            f.render_widget(clientmenu(&app.connected_clients), chunks[1]);
        }
        _ => {}
    }

    f.render_widget(titlebar(), chunks[0]);
    f.render_widget(infobar(app), chunks[2]);
}

fn titlebar<'a>() -> Paragraph<'a> {
    Paragraph::new("leightbox")
        .style(Style::default().fg(Color::LightRed))
        .alignment(Alignment::Center)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .style(Style::default().fg(Color::LightBlue))
                .border_type(BorderType::Thick),
        )
}

fn infobar<'a>(app: &App) -> Paragraph<'a> {
    let text = match app.connection_type {
        ConnectionType::Host => Text::styled(
            format!(
                "IP: {}   Password: {}",
                app.host_info.ip, app.host_info.password
            ),
            Style::default(),
        ),
        ConnectionType::Connect => Text::styled(
            format!(
                "Available: {}   Downloading: {}   Finished: {}",
                app.available_files.items.len(),
                app.downloading_files.len(),
                app.finished_files.len()
            ),
            Style::default(),
        ),
        _ => Text::raw(""),
    };

    Paragraph::new(text).alignment(Alignment::Center).block(
        Block::default()
            .borders(Borders::ALL)
            .style(Style::default().fg(Color::LightBlue))
            .border_type(BorderType::Thick),
    )
}

fn filemenu<'a>(items: &'a Vec<File>, title: &'a str) -> List<'a> {
    let items: Vec<ListItem> = items
        .iter()
        .map(|file| {
            ListItem::new(Spans::from(vec![
                Span::styled(file.name.as_str(), Style::default().fg(Color::LightMagenta)),
                Span::styled(
                    format!("   {} bytes", file.size),
                    Style::default().fg(Color::Gray),
                ),
            ]))
        })
        .collect();

    List::new(items)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Thick)
                .style(Style::default().fg(Color::LightYellow))
                .title(title),
        )
        .highlight_style(
            Style::default()
                .bg(Color::DarkGray)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol(" >> ")
}

fn clientmenu<'a>(items: &'a Vec<Client>) -> Table<'a> {
    let items: Vec<Row> = items
        .iter()
        .map(|client| {
            Row::new(vec![
                Cell::from(format!("{}", client.ip))
                    .style(Style::default().fg(Color::LightMagenta)),
                Cell::from(format!("{:?}", client.status)).style(Style::default().fg(Color::Gray)),
            ])
        })
        .collect();

    Table::new(items)
        .header(
            Row::new(vec!["IP", "Status"])
                .style(Style::default().fg(Color::LightYellow))
                .bottom_margin(2),
        )
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Thick)
                .style(Style::default().fg(Color::LightYellow))
                .title("Connected"),
        )
        .column_spacing(4)
        .widths([Constraint::Min(20), Constraint::Min(20)].as_ref())
}
