use std::io::Stdout;
use termion::raw::RawTerminal;
use termion::screen::AlternateScreen;
use tui::{
    backend::TermionBackend,
    layout::{Constraint, Direction, Rect},
    style::{Modifier, Style},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph, Wrap},
};

use tui::{layout::Layout, Frame};

use crate::app::{App, AppState};

use crate::statics::YEARS;

type TermionFrame<'a> = Frame<'a, TermionBackend<AlternateScreen<RawTerminal<Stdout>>>>;

pub fn draw(app: &mut App, f: &mut TermionFrame) {
    let chunks = build_layout(f.size());
    let mut list_state = ListState::default();
    let mut selections = Vec::new();
    match app.state {
        AppState::SelectingYear => {
            list_state.select(Some(app.year as usize));
            render_list(f, chunks[0], YEARS.iter(), list_state);
        }
        AppState::SelectingDay => {
            list_state.select(Some(app.day as usize));
            render_list(
                f,
                chunks[0],
                YEARS[app.year as usize].days.iter(),
                list_state,
            );
        }
        AppState::SelectingFile => {
            list_state.select(Some(app.dir_entry as usize));
            let entries = app.dir_entries();
            render_list(f, chunks[0], entries.iter(), list_state);
        }
        AppState::SelectingPart => {
            list_state.select(Some(app.part as usize));
            render_list(f, chunks[0], [1, 2].iter(), list_state);
        }
        AppState::ReadToSolve => {
            let p = Paragraph::new(format!("Solving: {}", app))
                .block(Block::default().borders(Borders::ALL));
            f.render_widget(p, chunks[0]);
        }
        AppState::Solved(ref solution) => {
            let p = Paragraph::new(format!("Your solution is:\n{}", solution))
                .block(Block::default().borders(Borders::ALL))
                .wrap(Wrap { trim: false });
            f.render_widget(p, chunks[0]);
        }
        AppState::SolveError(ref e) => {
            let p = Paragraph::new(format!("Error occured while solving:\n{}", e))
                .block(Block::default().borders(Borders::ALL));
            f.render_widget(p, chunks[0]);
        }
    }
    selections.push(YEARS[app.year as usize].to_string());
    if app.state > AppState::SelectingYear {
        selections.push(YEARS[app.year as usize].day(app.day as usize).to_string());
    }
    if app.state > AppState::SelectingDay {
        let entries = app.dir_entries();
        selections.push(format!("File: {}", entries[app.dir_entry as usize]));
    }
    if app.state > AppState::SelectingFile {
        selections.push(format!("Part: {}", app.part + 1));
    }
    render_list(f, chunks[1], selections.iter(), ListState::default());
}
fn build_layout(area: Rect) -> Vec<Rect> {
    let mut chunks: Vec<Rect> = Vec::new();
    let layout = Layout::default()
        .constraints(
            [
                Constraint::Length(10),
                Constraint::Min(15),
                Constraint::Length(10),
            ]
            .as_ref(),
        )
        .split(area);
    let layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Length(50),
                Constraint::Min(15),
                Constraint::Length(50),
            ]
            .as_ref(),
        )
        .split(layout[1]);
    let mut layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Ratio(2, 3), Constraint::Ratio(1, 3)].as_ref())
        .split(layout[1]);
    chunks.append(&mut layout);
    chunks
}

fn render_list<I, D>(f: &mut TermionFrame, area: Rect, iterable: I, mut state: ListState)
where
    I: Iterator<Item = D>,
    D: std::fmt::Display,
{
    let items: Vec<ListItem> = iterable.map(|i| ListItem::new(i.to_string())).collect();
    let items = List::new(items)
        .block(Block::default().borders(Borders::ALL))
        .highlight_style(Style::default().add_modifier(Modifier::BOLD))
        .highlight_symbol("✓");
    f.render_stateful_widget(items, area, &mut state);
}
