use ratatui::{
    Frame,
    layout::Rect,
    style::{
        Color,
        Stylize,
    },
    widgets::Paragraph,
};

pub fn draw(frame: &mut Frame, area: Rect, text: &str, color: Option<Color>) {
    match color {
        Some(color) => frame.render_widget(Paragraph::new(text).fg(color), area),
        None => frame.render_widget(Paragraph::new(text), area),
    }
}
