use heapless::{
    CapacityError,
    String,
};
use ratatui::{
    Frame,
    layout::{
        Offset,
        Position,
        Rect,
    },
    style::{
        Color,
        Stylize,
    },
    text::Text,
    widgets::{
        Block,
        Borders,
        Paragraph,
    },
};

#[derive(Debug)]
pub struct CommandLine<const T: usize> {
    content: String<T>,
    prefix_length: usize,
}

impl<const T: usize> CommandLine<T> {
    pub const fn new() -> Self {
        Self {
            content: String::new(),
            prefix_length: 0,
        }
    }

    pub fn push(&mut self, c: char) -> Result<(), CapacityError> {
        self.content.push(c)
    }

    pub fn pop(&mut self) -> Option<char> {
        if self.content.len() > self.prefix_length {
            self.content.pop()
        } else {
            None
        }
    }

    pub fn draw(&self, frame: &mut Frame, area: Rect) {
        let block = Block::default().borders(Borders::ALL).fg(Color::Red).title(" Command Line ");

        frame.render_widget(block, area);

        let par = Paragraph::new(Text::from(self.content.as_str()).fg(Color::White));

        frame.render_widget(par, area.offset(Offset::new(2, 1)));

        frame.set_cursor_position(Position {
            x: area.x + u16::try_from(self.content.len()).unwrap_or(u16::MAX) + 2,
            y: area.y + 1,
        });
    }

    pub const fn set_prefix_length(&mut self, length: usize) {
        self.prefix_length = length;
    }

    pub fn set_text(&mut self, text: &str) -> Result<(), CapacityError> {
        self.content.clear();

        self.content.push_str(text)
    }

    pub fn is_empty(&self) -> bool {
        self.content.is_empty()
    }

    pub fn as_str(&self) -> &str {
        self.content.as_str()
    }
}
