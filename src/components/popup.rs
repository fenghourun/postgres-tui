use std::io::Stdout;
use tui::{
    backend::CrosstermBackend,
    layout::{Alignment,Constraint, Direction, Layout},
    widgets::{Block, Borders, Clear, Paragraph},
    style::{Color, Style},
    Frame,
};

use crate::components::Component;

pub struct KeybindsPopup {
    percent_x: u16,
    percent_y: u16,
}

impl KeybindsPopup {
    pub fn new(percent_x: u16, percent_y: u16) -> KeybindsPopup {
        KeybindsPopup {
            percent_x,
            percent_y,
        }
    }
}

impl Component for KeybindsPopup {

    fn render(&self, f: &mut Frame<CrosstermBackend<Stdout>>) {

        let size = f.size();

        let block = Block::default().borders(Borders::ALL).title("Keybinds");

        let input = Paragraph::new(
            "
            1: Focus Side Bar
            2: Focus Main View
            3: Focus Search
            j: Move down
            k: Move up
            e: In Normal Mode => Edit Mode
            esc: In Edit Mode => Normal Mode
            b: Show Binds
            q: quit")
            .style(Style::default().fg(Color::Yellow))
            .alignment(Alignment::Left)
            .block(block);


        let vertical_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints(
                [
                    Constraint::Percentage((100 - self.percent_y) / 2),
                    Constraint::Percentage(self.percent_y),
                    Constraint::Percentage((100 - self.percent_y) / 2),
                ]
                .as_ref(),
            )
            .split(size);

        let area = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(
                [
                    Constraint::Percentage((100 - self.percent_x) / 2),
                    Constraint::Percentage(self.percent_x),
                    Constraint::Percentage((100 - self.percent_x) / 2),
                ]
                .as_ref(),
            )
            .split(vertical_layout[1])[1];

        f.render_widget(Clear, area);
        f.render_widget(input, area);
    }
}


