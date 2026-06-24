use crate::editor::terminal::{Position, Size, Terminal};
use buffer::Buffer;

use crossterm::event::KeyCode;
use std::io::Result;

mod buffer;

const NAME: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Debug, Clone, Copy, Default)]
struct Location {
    x: usize,
    y: usize,
}

#[derive(Default)]
pub struct View {
    pub buffer: Buffer,
    needs_redraw: bool,
    size: Size,
    location: Location,
}

impl View {
    pub fn new() -> Self {
        View {
            buffer: Buffer::default(),
            needs_redraw: true,
            size: Terminal::size().unwrap_or_default(),
            location: Location::default(),
        }
    }
    fn render_line(at: usize, line_text: &str) -> Result<()> {
        Terminal::move_caret_to(Position { col: 0, row: at })?;
        Terminal::clear_line()?;
        Terminal::print(line_text)?;
        Ok(())
    }
    fn render(&mut self) -> Result<()> {
        if !self.needs_redraw {
            return Ok(());
        }
        let Size { height, width } = self.size;
        if height == 0 || width == 0 {
            return Ok(());
        }

        let lines = self.buffer.lines();
        for i in 0..height {
            if let Some(line) = lines.get(i) {
                Self::render_line(i, line)?;
            } else if self.buffer.is_empty() && i == height / 3 {
                Self::render_line(i, &Self::welcome_screen_msg(width))?;
            } else {
                Self::render_line(i, "~")?;
            }
        }

        self.needs_redraw = false;
        Ok(())
    }
    pub fn refresh_screen(&mut self) -> Result<()> {
        Terminal::hide_caret()?;

        self.render()?;

        Terminal::move_caret_to(Position {
            col: self.location.x,
            row: self.location.y,
        })?;

        Terminal::show_caret()?;
        Terminal::execute()?;
        Ok(())
    }
    fn welcome_screen_msg(width: usize) -> String {
        let msg = format!("{NAME} - {VERSION}");
        let spaces_needed = " ".repeat((width - msg.len()) / 2);
        spaces_needed + &msg
    }
    pub fn resize(&mut self, size: Size) {
        self.size = size;
        self.needs_redraw = true;
    }
    pub fn load(&mut self, file: &str) {
        if let Ok(buf) = Buffer::load(file) {
            self.buffer = buf;
        }
    }
    pub fn move_caret(&mut self, code: KeyCode) {
        let Location { mut x, mut y } = self.location;
        let Size { width, height } = self.size;
        match code {
            KeyCode::Up => {
                if y > 0 {
                    y -= 1
                }
            }
            KeyCode::Down => {
                if y < height {
                    y += 1;
                }
            }
            KeyCode::Left => {
                if x > 0 {
                    x -= 1;
                }
            }
            KeyCode::Right => {
                if x < width {
                    x += 1;
                }
            }
            KeyCode::PageUp => {
                y = 0;
            }
            KeyCode::PageDown => {
                y = height.saturating_sub(1);
            }
            KeyCode::Home => {
                x = 0;
            }
            KeyCode::End => {
                x = width.saturating_sub(1);
            }
            _ => (),
        }
        self.location = Location { x, y };
    }
}
