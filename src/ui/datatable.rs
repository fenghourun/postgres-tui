use crate::app::{App, FocusElement};
use cli_log::info;
use ratatui::{
    layout::Rect,
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

pub fn render(f: &mut Frame, app: &App, area: Rect) {
    let (render_color, title) = match app.focused_element {
        FocusElement::Main => (Color::Green, " Main View (focused) "),
        _ => (Color::Red, " Main View "),
    };

    let default_style = Style::default().fg(render_color);

    let block = Block::default()
        .title(title)
        .borders(Borders::ALL)
        .style(default_style);

    match app.cluster.get_current_selected_table() {
        Some(current_table) => {
            let column_names = current_table.columns.clone().join(",");
            info!("Column names: {}", column_names);
            let names = Paragraph::new(column_names).block(Block::default().borders(Borders::ALL));
            f.render_widget(names, area);
        }
        None => {}
    };

    match app.cluster.get_current_data() {
        Some(current_data) => {
            let names = Paragraph::new(current_data.join("\n"))
                .block(Block::default().borders(Borders::ALL));
            info!("Data table Data: {:?}", current_data);
            f.render_widget(names, area);
        }
        None => {}
    };

    f.render_widget(block, area);
}
