use super::{Movement, TermionFrame};
use crate::config::Year;
use crate::text::welcome::welcome_spans_get;
use tui::widgets::{Block, Borders, List, ListItem, ListState, Paragraph};
use tui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    widgets::Wrap,
};

pub(super) struct Welcome {
    years: Vec<Year>,
    state: ListState,
}

impl Default for Welcome {
    fn default() -> Self {
        let years = Year::vec();
        let mut state = ListState::default();
        state.select(Some(0));
        Welcome { years, state }
    }
}

impl Welcome {
    pub(super) fn draw(&mut self, f: &mut TermionFrame) {
        let chunks = Layout::default()
            // .margin(2)
            .constraints(
                [
                    Constraint::Percentage(10),
                    Constraint::Percentage(80),
                    Constraint::Percentage(10),
                ]
                .as_ref(),
            )
            .split(f.size());

        let chunks = Layout::default()
            .margin(2)
            .direction(Direction::Horizontal)
            .constraints(
                [
                    Constraint::Percentage(15),
                    Constraint::Percentage(70),
                    Constraint::Percentage(15),
                ]
                .as_ref(),
            )
            .split(chunks[1]);
        let block = Block::default().borders(Borders::ALL);

        f.render_widget(block, chunks[1]);
        let chunks = Layout::default()
            .vertical_margin(1)
            .horizontal_margin(3)
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(80), Constraint::Percentage(20)].as_ref())
            .split(chunks[1]);
        let block = Block::default();
        let text = welcome_spans_get();
        let paragraph = Paragraph::new(text)
            .block(block)
            .alignment(tui::layout::Alignment::Left)
            .wrap(Wrap { trim: false });

        f.render_widget(paragraph, chunks[0]);
        let block = Block::default().borders(Borders::LEFT);
        f.render_widget(block, chunks[1]);
        let chunks = Layout::default()
            .margin(2)
            .constraints([Constraint::Min(8)].as_ref())
            .split(chunks[1]);
        let items: Vec<ListItem> = self
            .years
            .iter()
            .map(|y| ListItem::new(y.to_string()))
            .collect();
        let list = List::new(items)
            .style(Style::default().fg(Color::White))
            .start_corner(tui::layout::Corner::TopLeft)
            .highlight_style(Style::default().add_modifier(Modifier::BOLD))
            .highlight_symbol("⇥ ");
        f.render_stateful_widget(list, chunks[0], &mut self.state);
    }

    pub(super) fn move_selected(&mut self, movement: Movement) {
        let index = self.state.selected().expect("List item should be selected");
        let index = (index as isize + movement as isize + self.years.len() as isize)
            % self.years.len() as isize;
        self.state.select(Some(index as usize));
    }

    pub(super) fn pick(&self) -> Year {
        let index = self.state.selected().expect("List item should be selected");
        return self.years[index];
    }
}
