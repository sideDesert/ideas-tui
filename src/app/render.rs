use super::{
    App, Block, Borders, Color, Constraint, Direction, Focus, Frame, Layout, Line, Paragraph,
    Position, Rect, Style,
};
use crate::app::utils::centered_rect;

pub trait Render<'a> {
    fn render_write_mode(&mut self, frame: &mut Frame, area: Rect);
    fn render_edit_mode(&mut self, frame: &mut Frame, area: Rect);
}

impl<'a> Render<'a> for App<'a> {
    fn render_write_mode(&mut self, frame: &mut Frame, area: Rect) {
        let rect = centered_rect(40, 40, area);

        let rect_child = centered_rect(90, 90, rect);

        let active_style = Style::new().fg(Color::Green);
        let passive_style = Style::new().fg(Color::DarkGray);

        let active_button_style = Style::new().fg(Color::Green);
        let passive_button_style = Style::new().fg(Color::DarkGray);
        let button_style;

        let block = Block::new()
            .borders(Borders::ALL)
            .style(Style::default().fg(Color::DarkGray));

        let [title_area, description_area, footer_area] = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3),
                Constraint::Min(1),
                Constraint::Length(2),
            ])
            .areas(rect_child);

        // let title_text = &self.buffer[0];
        // let description_text = &self.buffer[1];

        let title_style;
        let desc_style;
        let border_style;
        let active_border_style = Borders::NONE;
        let passive_border_style = Borders::NONE;

        match self.focus.as_ref().unwrap() {
            Focus::Title => {
                title_style = active_style;
                desc_style = passive_style;
                button_style = passive_button_style;
                border_style = passive_border_style;

                self.cursor_offset.0 = title_area.x + 1;
                self.cursor_offset.1 = title_area.y + 1;

                self.title_view.set_width(48);
                frame.set_cursor_position(Position::new(
                    self.cursor_offset.0 + self.title_view.get_last_word_cursor_position().0,
                    self.cursor_offset.1,
                ));
            }
            Focus::Description => {
                title_style = passive_style;
                desc_style = active_style;
                button_style = passive_button_style;
                border_style = passive_border_style;

                self.cursor_offset.0 = description_area.x + 1;
                self.cursor_offset.1 = description_area.y + 1;

                frame.set_cursor_position(Position::new(
                    self.cursor_offset.0 + self.description_view.get_last_word_cursor_position().0,
                    self.description_view.get_last_word_cursor_position().1 + self.cursor_offset.1,
                ));
            }

            Focus::Add => {
                title_style = passive_style;
                desc_style = passive_style;
                button_style = active_button_style;
                border_style = active_border_style;
            }
        }

        let title_block = Block::default()
            .borders(Borders::ALL)
            .title("Title")
            .style(title_style);

        self.title_view.set_width(title_area.width);
        self.title_view.set_buffer(&self.buffer[0]);

        let desc_block = Block::default()
            .borders(Borders::ALL)
            .title("Description")
            .style(desc_style);

        self.description_view.set_width(title_area.width);
        self.description_view.set_buffer(&self.buffer[1]);

        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(40),
                Constraint::Length(14), // button width
                Constraint::Percentage(40),
            ])
            .split(footer_area);

        let button_area = chunks[1];
        let button_block = Block::default().borders(border_style).style(button_style);

        let button_text = Paragraph::new("  + Add Idea").block(button_block);

        frame.render_widget(block, rect);

        self.title_view.render(frame, title_block, title_area);
        self.description_view
            .render(frame, desc_block, description_area);
        frame.render_widget(button_text, button_area);
    }

    fn render_edit_mode(&mut self, frame: &mut Frame, area: Rect) {
        let rect = centered_rect(40, 40, area);

        let rect_child = centered_rect(90, 90, rect);

        let active_style = Style::new().fg(Color::Green);
        let passive_style = Style::new().fg(Color::DarkGray);

        let active_button_style = Style::new().fg(Color::Green);
        let passive_button_style = Style::new().fg(Color::DarkGray);
        let button_style;

        let block = Block::new()
            .borders(Borders::ALL)
            .style(Style::default().fg(Color::DarkGray));

        let [title_area, description_area, footer_area] = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3),
                Constraint::Min(1),
                Constraint::Length(2),
            ])
            .areas(rect_child);

        let title_text = &self.buffer[0];
        let description_text = &self.buffer[1];
        let title_style;
        let desc_style;
        let border_style;
        let active_border_style = Borders::NONE;
        let passive_border_style = Borders::NONE;

        match self.focus.as_ref().unwrap() {
            Focus::Title => {
                title_style = active_style;
                desc_style = passive_style;
                button_style = passive_button_style;
                border_style = passive_border_style;

                // self.cursor_position.0 = title_area.x + self.buffer[0].len() as u16 + 1;
                // self.cursor_position.1 = title_area.y + 1;

                frame.set_cursor_position(Position::new(
                    self.cursor_position.0,
                    self.cursor_position.1,
                ));
            }
            Focus::Description => {
                title_style = passive_style;
                desc_style = active_style;
                button_style = passive_button_style;
                border_style = passive_border_style;

                // self.cursor_position.0 = description_area.x + self.buffer[1].len() as u16 + 1;
                // self.cursor_position.1 = description_area.y + 1;

                frame.set_cursor_position(Position::new(
                    self.cursor_position.0,
                    self.cursor_position.1,
                ));
            }
            Focus::Add => {
                title_style = passive_style;
                desc_style = passive_style;
                button_style = active_button_style;
                border_style = active_border_style;
            }
        }

        let title_input = Paragraph::new(Line::from(title_text.clone()))
            .block(Block::default().borders(Borders::ALL).title("Title"))
            .style(title_style);

        let description_input = Paragraph::new(Line::from(description_text.clone())).block(
            Block::default()
                .borders(Borders::ALL)
                .title("Description")
                .style(desc_style),
        );

        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(40),
                Constraint::Length(14), // button width
                Constraint::Percentage(40),
            ])
            .split(footer_area);

        let button_area = chunks[1];
        let button_block = Block::default().borders(border_style).style(button_style);

        let button_text = Paragraph::new("  📝 Edit Idea").block(button_block);

        frame.render_widget(block, rect);
        frame.render_widget(title_input, title_area);

        frame.render_widget(description_input, description_area);
        frame.render_widget(button_text, button_area);
    }
}
