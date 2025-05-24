use ratatui::layout::Rect;

use super::{Block, Frame, Line, Paragraph};

pub struct View {
    width: u16,
    buffer: String,
    cursor_position: (u16, u16),
}

impl View {
    pub fn new() -> Self {
        Self {
            width: 0,
            buffer: String::new(),
            cursor_position: (0, 0),
        }
    }

    pub fn set_width(&mut self, width: u16) {
        self.width = width;
    }

    pub fn set_buffer(&mut self, buffer: &str) {
        self.buffer = buffer.to_string();
    }

    pub fn build(&mut self) -> Vec<String> {
        let mut vec: Vec<String> = Vec::new();
        let mut buf_string: Vec<String> = Vec::new();
        let words = self.buffer.split_whitespace();

        for word in words {
            let x = word.len() as u16 + buf_string.join(" ").len() as u16;
            if x > self.width {
                vec.push(buf_string.join(" ").to_string());
                buf_string.clear();
            }
            buf_string.push(word.to_string());
        }
        vec.push(buf_string.join(" ").to_string());

        vec
    }

    pub fn render(&mut self, frame: &mut Frame, block: Block, area: Rect) {
        let lines = self.build();
        let text: Vec<Line> = lines.iter().map(|l| Line::from(l.as_str())).collect();
        // let text = Line::from(lines.as_str());
        let paragraph = Paragraph::new(text).block(block);

        frame.render_widget(paragraph, area);
    }

    pub fn get_last_word_cursor_position(&mut self) -> (u16, u16) {
        assert_ne!(self.width, 0);
        let lines = self.build();
        let x;
        match lines.last() {
            Some(line) => x = line.len() as u16,
            None => x = 0,
        }
        let y = (lines.len() as u16 - 1).max(0);
        (x, y)
    }
}

mod tests {
    #[test]
    fn test_get_view_buffer() {
        use crate::app::buffer::View;

        let buffer = "Hello my name is Siddarth Saha";
        let mut view = View::new();
        view.set_width(10);
        view.set_buffer(buffer);
        let res = view.build();
        let mut truth = Vec::new();

        truth.push(String::from("Hello my "));
        truth.push(String::from("name is "));
        truth.push(String::from("Siddarth "));
        truth.push(String::from("Saha "));

        assert_eq!(res, truth);

        let buffer = "Hello my name is Siddarth Saha and I love programming";
        let mut view = View::new();
        view.set_width(48);
        view.set_buffer(buffer);
        let res = view.build();
        let mut truth = Vec::new();

        truth.push(String::from("Hello my name is Siddarth Saha and I love "));
        truth.push(String::from("programming "));
        assert_eq!(res, truth);
    }

    #[test]
    fn test_cursor_position() {
        use crate::app::buffer::View;
        let buffer = "Hello my name is Siddarth Saha";
        let mut view = View::new();
        view.set_width(10);
        view.set_buffer(buffer);
        view.build();
        let res = view.get_last_word_cursor_position();
        let truth = (5, 4);
        assert_eq!(res, truth);

        let buffer = "Hello my name is Siddarth Saha. I love programming";
        let mut view = View::new();
        view.set_width(10);
        view.set_buffer(buffer);
        view.build();

        let res = view.get_last_word_cursor_position();
        let truth = (0, 5);
        assert_eq!(res, truth);
    }
}
