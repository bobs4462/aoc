use crate::text::challenge1::challenge1_pans_get;
use tui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph, Wrap},
};

use super::{Movement, TermionFrame};

pub(super) struct Puzzles {
    days: usize,
    parts: usize,
    days_state: ListState,
    parts_state: ListState,
    selected_column: SelectedColumn,
}

enum SelectedColumn {
    Days,
    Parts,
}

impl Default for Puzzles {
    fn default() -> Self {
        let mut days_state = ListState::default();
        days_state.select(Some(0));
        let parts_state = ListState::default();
        Puzzles {
            days: 25,
            parts: 2,
            days_state,
            parts_state,
            selected_column: SelectedColumn::Days,
        }
    }
}

impl Puzzles {
    pub(super) fn draw(&mut self, f: &mut TermionFrame) {
        let chunks = Layout::default()
            .margin(1)
            .direction(Direction::Horizontal)
            .constraints(
                [
                    Constraint::Percentage(10),
                    Constraint::Percentage(10),
                    Constraint::Percentage(80),
                ]
                .as_ref(),
            )
            .split(f.size());

        let days_chunk = Layout::default()
            .horizontal_margin(1)
            .constraints([Constraint::Length(3), Constraint::Max(30)].as_ref())
            .split(chunks[0]);
        let parts_chunk = Layout::default()
            .horizontal_margin(1)
            .constraints([Constraint::Length(3), Constraint::Max(30)].as_ref())
            .split(chunks[1]);
        let block = Block::default().borders(Borders::ALL);

        f.render_widget(block.clone(), chunks[0]);
        f.render_widget(block.clone().borders(Borders::BOTTOM), days_chunk[0]);
        f.render_widget(block.clone(), chunks[1]);
        f.render_widget(block.clone().borders(Borders::BOTTOM), parts_chunk[0]);
        f.render_widget(block.clone(), chunks[2]);
        let items: Vec<ListItem> = (1..=self.days)
            .map(|d| ListItem::new(format!("Day {}", d)))
            .collect();
        let list = List::new(items)
            .style(Style::default().fg(Color::White))
            .start_corner(tui::layout::Corner::TopLeft)
            .highlight_style(Style::default().add_modifier(Modifier::BOLD))
            .highlight_symbol("⇥ ");
        f.render_stateful_widget(list, days_chunk[1], &mut self.days_state);
        let items: Vec<ListItem> = (1..=self.parts)
            .map(|p| ListItem::new(format!("Part {}", p)))
            .collect();
        let list = List::new(items)
            .style(Style::default().fg(Color::White))
            .start_corner(tui::layout::Corner::TopLeft)
            .highlight_style(Style::default().add_modifier(Modifier::BOLD))
            .highlight_symbol("⇥ ");
        f.render_stateful_widget(list, parts_chunk[1], &mut self.parts_state);
        let text = challenge1_pans_get();
        let paragraph = Paragraph::new(text)
            .alignment(tui::layout::Alignment::Left)
            .block(block.clone())
            .wrap(Wrap { trim: false });
        f.render_widget(paragraph, chunks[2]);
    }
    pub(super) fn move_selected(&mut self, movement: Movement) {
        match movement {
            Movement::Up | Movement::Down => self.move_vert(movement),
            Movement::Left | Movement::Right => self.move_horz(movement),
        }
    }

    fn move_vert(&mut self, movement: Movement) {
        match self.selected_column {
            SelectedColumn::Days => {
                let i = self
                    .days_state
                    .selected()
                    .expect("Day should've been selected");
                let i = (i as isize + movement as isize + self.days as isize) % self.days as isize;
                self.days_state.select(Some(i as usize));
            }
            SelectedColumn::Parts => {
                let i = self
                    .parts_state
                    .selected()
                    .expect("Part should've been selected");
                let i =
                    (i as isize + movement as isize + self.parts as isize) % self.parts as isize;
                self.parts_state.select(Some(i as usize));
            }
        }
    }

    fn move_horz(&mut self, _: Movement) {
        match self.selected_column {
            SelectedColumn::Days => {
                self.selected_column = SelectedColumn::Parts;
                self.parts_state.select(Some(0));
                self.days_state.select(None)
            }
            SelectedColumn::Parts => {
                self.selected_column = SelectedColumn::Days;
                self.days_state.select(Some(0));
                self.parts_state.select(None)
            }
        }
    }

    pub(super) fn pick(&mut self) -> (usize, usize) {
        let day = self.days_state.selected().map_or(0, |i| i + 1);
        let part = self.parts_state.selected().map_or(0, |i| i + 1);
        if let SelectedColumn::Days = self.selected_column {
            self.selected_column = SelectedColumn::Parts;
            self.days_state.select(None);
            self.parts_state.select(Some(1));
        }
        (day, part)
    }
}
