mod event;
mod render;
mod state;
mod utils;

use event::Handler;
use render::Render;
use std::{io::BufReader, path::PathBuf};
use utils::hex_to_rgb;

use crossterm::event::{KeyCode, KeyEvent, KeyEventKind};

use ratatui::{
    DefaultTerminal, Frame,
    layout::{Constraint, Direction, Layout, Position, Rect},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
};

#[derive(serde::Serialize, serde::Deserialize, Debug)]
struct Idea {
    title: String,
    description: String,
}

impl Idea {
    fn new(title: &str, description: &str) -> Self {
        Self {
            title: String::from(title),
            description: String::from(description),
        }
    }
}

#[derive(PartialEq)]
enum Mode {
    Read,
    Write,
    Edit,
}

#[derive(PartialEq)]
enum Focus {
    Title,
    Description,
    Add,
}

pub struct App<'a> {
    exit: bool,
    active_index: usize,
    ideas: Vec<Idea>,
    focus: Option<Focus>,
    mode: Mode,
    buffer: &'a mut [String; 2],
    cursor_position: (u16, u16),
    path: PathBuf,
    active_index_path: PathBuf,
}

#[allow(dead_code)]
impl<'a> App<'a> {
    pub fn new(buf: &'a mut [String; 2]) -> Self {
        let mut path = std::env::current_exe().unwrap();
        path.pop();
        path.pop();
        path.pop();
        path.push("ideas.json");
        println!("{:?}", path);

        let mut path2 = std::env::current_exe().unwrap();
        path2.pop();
        path2.pop();
        path2.pop();
        path2.push("index.txt");
        println!("{:?}", path2);

        Self {
            path: path,
            active_index_path: path2,
            mode: Mode::Read,
            exit: false,
            active_index: 0,
            focus: None,
            ideas: Vec::new(),
            buffer: buf,
            cursor_position: (0, 0),
        }
    }

    fn color(hex: &str) -> Color {
        let (r, g, b) = hex_to_rgb(hex).unwrap();
        Color::Rgb(r, g, b)
    }

    fn get_ideas_widget(&self) -> Vec<Line> {
        let mut lines: Vec<Line>;
        lines = Vec::new();

        for (i, idea) in self.ideas.iter().enumerate() {
            if i == self.active_index {
                lines.push("".into());
                let mut title_string = String::new();
                title_string.push_str("> ");
                title_string.push_str(&idea.title);
                let title = Span::styled(title_string, Style::default().fg(Color::Green));
                lines.push(title.into());

                if idea.description != "" {
                    let mut desc_string = String::new();
                    desc_string.push_str("      ");
                    desc_string.push_str(&idea.description);
                    let desc = Span::styled(desc_string, Style::default().fg(Color::Gray));
                    lines.push(desc.into());
                }

                lines.push("".into());
            } else {
                let mut title_string = String::new();
                title_string.push_str(&idea.title);
                let title = Span::styled(title_string, Style::default().fg(Color::White));
                lines.push(title.into());
            }
        }
        lines
    }

    pub fn run(&mut self, mut terminal: DefaultTerminal) -> std::io::Result<()> {
        let file = std::fs::File::open(&self.path)?;
        let reader = BufReader::new(file);
        let ideas: Vec<Idea> = serde_json::from_reader(reader).unwrap();
        self.ideas = ideas;

        let istring = std::fs::read_to_string(&self.active_index_path)?;
        let active_index: usize = istring.trim().parse().unwrap();
        self.active_index = active_index;

        while !self.exit {
            terminal.draw(|frame| {
                let area = frame.area();

                let title = Span::styled("  Ideas  ", Style::default().fg(Color::Green));
                let block = Block::default()
                    .title(title)
                    .borders(Borders::ALL)
                    .style(Style::default().fg(Color::DarkGray));
                frame.render_widget(block, area);

                let [_, body_area] = Layout::default()
                    .direction(Direction::Vertical)
                    .constraints([Constraint::Max(2), Constraint::Min(1)])
                    .areas(area);

                let ideas = self.get_ideas_widget();
                let para = Paragraph::new(ideas).style(Style::default());

                frame.render_widget(para, body_area);

                match self.mode {
                    Mode::Write => {
                        self.render_write_mode(frame, area);
                    }
                    Mode::Edit => {
                        self.render_edit_mode(frame, area);
                    }
                    _ => {}
                }
            })?;
            self.handle_events()?;
        }
        Ok(())
    }
}
