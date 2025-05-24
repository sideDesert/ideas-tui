use ratatui::layout::Rect;

use super::{Block, Frame, Line, Paragraph};

pub struct View<'v> {
    width: u16,
    buffer: &'v str,
    cursor_position: (u16, u16),
}

impl<'v> View<'v> {
    pub fn new(width: u16, buffer: &'v str) -> Self {
        Self {
            width,
            buffer,
            cursor_position: (0, 0),
        }
    }

    fn build(&mut self) -> Vec<String> {
        let mut vec: Vec<String> = Vec::new();
        let mut buf_string = String::from("");
        let words = self.buffer.split_whitespace();

        let mut x: u16 = 0;
        let mut y: u16 = 0;

        for word in words {
            x = word.len() as u16 + buf_string.len() as u16;
            if x > self.width {
                vec.push(buf_string[..].to_string());
                buf_string.clear();
                x = word.len() as u16;
            } else {
                y += 1;
            }
            let push_str = format!("{} ", word);
            buf_string.push_str(&push_str);
        }
        vec.push(buf_string[..].to_string());
        let final_x = if x >= self.width { 0 } else { x + 1 };
        let final_y = y + 1;
        self.cursor_position = (final_x, final_y);

        vec
    }

    pub fn render(&mut self, frame: &mut Frame, block: Block, area: Rect) {
        let lines = self.build();
        let text: Vec<Line> = lines.iter().map(|l| Line::from(l.as_str())).collect();
        let paragraph = Paragraph::new(text).block(block);

        frame.render_widget(paragraph, area);
    }

    pub fn get_last_word_cursor_position(&self) -> (u16, u16) {
        self.cursor_position
    }
}

mod tests {
    #[test]
    fn test_get_view_buffer() {
        use crate::app::buffer::View;

        let buffer = "Hello my name is Siddarth Saha";
        let mut view = View::new(10, buffer);
        let res = view.build();
        let mut truth = Vec::new();

        truth.push(String::from("Hello my "));
        truth.push(String::from("name is "));
        truth.push(String::from("Siddarth "));
        truth.push(String::from("Saha "));

        assert_eq!(res, truth);
    }

    #[test]
    fn test_cursor_position() {
        use crate::app::buffer::View;
        let buffer = "Hello my name is Siddarth Saha";
        let view = View::new(10, buffer);
        let res = view.get_last_word_cursor_position();
        let truth = (5, 4);
        assert_eq!(res, truth);

        let buffer = "Hello my name is Siddarth Saha. I love programming";
        let view = View::new(10, buffer);
        let res = view.get_last_word_cursor_position();
        let truth = (0, 5);
        assert_eq!(res, truth);
    }
}
