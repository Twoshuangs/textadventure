use crate::{map, widget};
use ratatui::{
    DefaultTerminal, Frame,
    buffer::Buffer,
    layout::{Constraint, Direction, Flex, Layout, Rect},
    text::Text,
    widgets::{Block, Clear, Paragraph, Widget, Wrap},
};
use std::io;

pub struct App {
    pub exit: bool,
    pub player: [i32; 2],
    pub stringmap: String,
    pub vecmap: map::Map,
}

impl App {
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
        }
        Ok(())
    }

    fn draw(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let gamearea = widget::center(area, Constraint::Percentage(80), Constraint::Percentage(70));

        Paragraph::new(Text::from("hi")).render(gamearea, buf);
    }
}
