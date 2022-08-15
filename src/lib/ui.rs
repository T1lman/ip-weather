use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{error::Error, io};
use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::Span,
    widgets::{BarChart, Block, BorderType, Borders, Paragraph},
    Frame, Terminal,
};

use crate::data::Data;

pub async fn main() -> Result<(), Box<dyn Error>> {
    let data = Data::new().await;

    let mut resvec: Vec<(&str, u64)> = Vec::new();

    for i in 0..data.weatherdata.tempvec.len() {
        resvec.push((
            &data.weatherdata.timevec[i],
            data.weatherdata.tempvec[i] as u64,
        ))
    }

    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // create app and run it
    let res = run_app(&mut terminal, &data, resvec);

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err)
    }

    Ok(())
}

fn run_app<B: Backend>(
    terminal: &mut Terminal<B>,
    data: &Data,
    dataset: Vec<(&str, u64)>,
) -> io::Result<()> {
    loop {
        terminal.draw(|f| ui(f, data, &dataset))?;

        if let Event::Key(key) = event::read()? {
            if let KeyCode::Char('q') = key.code {
                return Ok(());
            }
        }
    }
}

fn ui<B: Backend>(f: &mut Frame<B>, data: &Data, dataset: &[(&str, u64)]) {
    // Wrapping block for a group
    // Just draw the block and the group on the same area and build the group
    // with at least a margin of 1
    let size = f.size();
    let theme_color = Color::LightMagenta;
    // Surrounding block
    let block = Block::default()
        .borders(Borders::empty())
        .title(Span::styled(
            format!(" {} ", data.ipdata.ip),
            Style::default()
                .add_modifier(Modifier::UNDERLINED)
                .fg(theme_color),
        ))
        .title_alignment(Alignment::Center)
        .border_type(BorderType::Rounded)
        .border_style(
            Style::default()
                .fg(theme_color)
                .add_modifier(Modifier::BOLD),
        );
    f.render_widget(block, size);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints([Constraint::Percentage(25), Constraint::Percentage(75)].as_ref())
        .split(f.size());
    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(
            Style::default()
                .fg(theme_color)
                .add_modifier(Modifier::BOLD),
        )
        .border_type(BorderType::Rounded)
        .title(Span::styled(
            format!("Facts about {}", data.ipdata.ip),
            Style::default()
                .add_modifier(Modifier::BOLD)
                .fg(theme_color),
        ));
    let para = Paragraph::new(data.ipdata.fmtstr.clone())
        .style(Style::default().fg(Color::Yellow))
        .block(block);
    f.render_widget(para, chunks[0]);

    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(theme_color))
        .border_type(BorderType::Rounded)
        .title(Span::styled(
            format!("Temperature chart of {} in °C", data.ipdata.city),
            Style::default()
                .add_modifier(Modifier::BOLD)
                .fg(theme_color),
        ));
    let chart = BarChart::default()
        .data(dataset)
        .block(block)
        .bar_width(5)
        .bar_style(Style::default().fg(Color::Yellow))
        .value_style(Style::default().fg(Color::White).bg(Color::Yellow))
        .label_style(
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::ITALIC),
        );

    f.render_widget(chart, chunks[1]);
}

//TODO! Implement Barchart!
