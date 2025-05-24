use super::state::Handler as StateHandler;
use crossterm::event::Event;

use super::{App, Focus, KeyCode, KeyEvent, KeyEventKind, Mode};

pub trait Handler {
    fn handle_key_event(&mut self, key_event: KeyEvent) -> std::io::Result<()>;
    fn handle_events(&mut self) -> std::io::Result<()>;
}

impl<'a> Handler for App<'a> {
    fn handle_events(&mut self) -> std::io::Result<()> {
        match crossterm::event::read()? {
            Event::Key(key_event) => {
                if key_event.kind == KeyEventKind::Press {
                    self.handle_key_event(key_event)?;
                }
            }
            _ => {}
        }
        Ok(())
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) -> std::io::Result<()> {
        match self.mode {
            Mode::Read => match key_event.code {
                KeyCode::Char('q') => {
                    self.quit();
                    let i = self.active_index;
                    std::fs::write(&self.active_index_path, i.to_string())?;
                }
                KeyCode::Char('d') => {
                    let index = self.active_index;
                    self.remove_idea(index);
                }
                KeyCode::Up => {
                    if self.active_index != 0 {
                        self.active_index -= 1;
                    }
                }
                KeyCode::Char('k') => {
                    if self.active_index != 0 {
                        self.active_index -= 1;
                    }
                }
                KeyCode::Down => {
                    if self.active_index != self.ideas.len() - 1 {
                        self.active_index += 1;
                    }
                }
                KeyCode::Char('j') => {
                    if self.active_index != self.ideas.len() - 1 {
                        self.active_index += 1;
                    }
                }
                KeyCode::Char('e') | KeyCode::Char('c') => {
                    self.focus = Some(Focus::Title);
                    self.load_buffer();
                    self.mode = Mode::Edit;
                }
                KeyCode::Char('a') | KeyCode::Char('i') => {
                    self.mode = Mode::Write;
                    self.focus = Some(Focus::Title);
                }
                _ => {}
            },
            Mode::Write => match key_event.code {
                KeyCode::Char(char) => {
                    if let Some(focus) = &self.focus {
                        match focus {
                            Focus::Title => {
                                self.buffer[0].push(char);
                                self.cursor_position.0 += 1;
                            }
                            Focus::Description => {
                                self.buffer[1].push(char);
                                self.cursor_position.0 += 1;
                            }
                            _ => {}
                        }
                    }
                }
                KeyCode::Enter => {
                    let title = &self.buffer[0].clone();
                    let description = &self.buffer[1].clone();
                    self.add_idea(title, description);
                    self.clear_buffer();
                    self.focus = Some(Focus::Title);
                    self.save()
                }
                KeyCode::Backspace => {
                    if let Some(focus) = &self.focus {
                        match focus {
                            Focus::Title => {
                                self.buffer[0].pop();
                            }
                            Focus::Description => {
                                self.buffer[1].pop();
                            }
                            _ => {}
                        }
                    }
                }
                KeyCode::Tab => {
                    if let Some(focus) = &self.focus {
                        match focus {
                            Focus::Title => {
                                self.focus = {
                                    let sanitized = self.buffer[0].trim();
                                    if sanitized != "" {
                                        self.cursor_position.0 = self.buffer[1].len() as u16;
                                        Some(Focus::Description)
                                    } else {
                                        Some(Focus::Title)
                                    }
                                }
                            }
                            Focus::Description => {
                                self.focus = Some(Focus::Add);
                                self.cursor_position.0 = self.buffer[1].len() as u16;
                            }
                            Focus::Add => self.focus = Some(Focus::Title),
                        }
                    }
                }
                KeyCode::Esc => {
                    self.mode = Mode::Read;
                    self.focus = None;
                }
                KeyCode::Left => {
                    self.cursor_position.0 = self.cursor_position.0.saturating_sub(1);
                }
                KeyCode::Right => {
                    self.cursor_position.0 = self.cursor_position.0.saturating_add(1);
                }
                _ => {}
            },
            Mode::Edit => match key_event.code {
                KeyCode::Char(char) => {
                    if let Some(focus) = &self.focus {
                        match focus {
                            Focus::Title => {
                                self.buffer[0].push(char);
                                self.cursor_position.0 = self.cursor_position.0.saturating_add(1);
                            }
                            Focus::Description => {
                                self.buffer[1].push(char);
                                self.cursor_position.0 = self.cursor_position.0.saturating_add(1);
                            }
                            _ => {}
                        }
                    }
                }
                KeyCode::Enter => {
                    self.save_edit();
                    self.clear_buffer();
                    self.focus = Some(Focus::Title);
                    self.save()
                }
                KeyCode::Backspace => {
                    if let Some(focus) = &self.focus {
                        match focus {
                            Focus::Title => {
                                self.buffer[0].pop();
                            }
                            Focus::Description => {
                                self.buffer[1].pop();
                            }
                            _ => {}
                        }
                    }
                }
                KeyCode::Tab => {
                    if let Some(focus) = &self.focus {
                        match focus {
                            Focus::Title => {
                                self.focus = {
                                    let sanitized = self.buffer[0].trim();
                                    if sanitized != "" {
                                        self.cursor_position.0 = self.buffer[1].len() as u16;
                                        Some(Focus::Description)
                                    } else {
                                        Some(Focus::Title)
                                    }
                                }
                            }
                            Focus::Description => {
                                self.focus = Some(Focus::Add);
                                self.cursor_position.0 = self.buffer[1].len() as u16;
                            }
                            Focus::Add => self.focus = Some(Focus::Title),
                        }
                    }
                }
                KeyCode::Esc => {
                    self.mode = Mode::Read;
                    self.focus = None;
                }
                KeyCode::Left => {
                    self.cursor_position.0 = self.cursor_position.0.saturating_sub(1);
                }
                KeyCode::Right => {
                    self.cursor_position.0 = self.cursor_position.0.saturating_sub(1);
                }
                _ => {}
            },
        }
        Ok(())
    }
}
