pub mod datatable;
pub mod debug;
pub mod help_window;
pub mod searchbar;
pub mod sidebar;
pub mod statusline;

use ratatui::{
    layout::{Constraint, Direction, Layout},
    Frame,
};

use crate::app::App;

pub trait Component {
    fn render(&self, f: &mut Frame);
}

pub fn draw(f: &mut Frame, app: &mut App) {
    let area = f.size();

    let horizontal_split = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(30), Constraint::Percentage(70)].as_ref())
        .split(area);

    let statusline_area = Layout::default()
        .constraints([Constraint::Ratio(23, 24), Constraint::Ratio(1, 24)].as_ref())
        .split(area)[1];

    let left_vertical_split = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Ratio(23, 24), Constraint::Ratio(1, 24)].as_ref())
        .split(horizontal_split[0]);

    let right_vertical_split = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Ratio(2, 24),
                Constraint::Ratio(21, 24),
                Constraint::Ratio(1, 24),
            ]
            .as_ref(),
        )
        .split(horizontal_split[1]);

    sidebar::render(f, app, left_vertical_split[0]);
    datatable::render(f, app, right_vertical_split[1]);
    searchbar::render(f, app, right_vertical_split[0]);
    statusline::render(f, app, statusline_area);

    if app.show_keybinds {
        let p = help_window::KeybindsPopup::new(60, 40);
        p.render(f);
    }

    if app.show_debug {
        let p = debug::DebugPopup::new(60, 40, app.debug_message.clone());
        p.render(f);
    }
}
