use crate::editor::terminal::{Size, Terminal};
use buffer::Buffer;

use crossterm::event::KeyCode;
use std::{
    cmp::min,
    io::{Result, Write},
};

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
    offset: Location,
}

impl View {
    pub fn new() -> Self {
        View {
            buffer: Buffer::default(),
            needs_redraw: true,
            size: Terminal::size().unwrap_or_default(),
            location: Location::default(),
            offset: Location::default(),
        }
    }
    pub fn refresh_screen(&mut self) -> Result<()> {
        Terminal::hide_caret()?;

        self.render()?;

        Terminal::move_caret_to(
            self.location.x - self.offset.x,
            self.location.y - self.offset.y,
        )?;

        Terminal::show_caret()?;
        Terminal::execute()?;
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

        let top = self.offset.y;
        let lines = self.buffer.lines();
        for i in 0..height {
            if let Some(line) = lines.get(i.saturating_add(top)) {
                let left = self.offset.x;

                // when scrolling right end will have the entire width + offset or the len of string
                let right = min(self.offset.x.saturating_add(width), line.len());
                let line = &line.get(left..right).unwrap_or_default();

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
    fn render_line(at: usize, line_text: &str) -> Result<()> {
        Terminal::move_caret_to(0, at)?;
        Terminal::clear_line()?;
        Terminal::print(line_text)?;
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
                y = y.saturating_sub(1);
            }
            KeyCode::Down => {
                y = y.saturating_add(1);
            }
            KeyCode::Left => {
                x = x.saturating_sub(1);
            }
            KeyCode::Right => {
                x = x.saturating_add(1);
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

        // snap x and y to valid positions
        y = min(y, self.buffer.len());
        x = self
            .buffer
            .lines()
            .get(y)
            .map_or(0, |line| min(x, line.len()));

        self.location = Location { x, y };
        self.scroll_location_into_view();
    }
    fn scroll_location_into_view(&mut self) {
        let Location { x, y } = self.location;
        let Size { width, height } = self.size;
        let mut offset_changed = false;

        if y < self.offset.y {
            self.offset.y = y;
            offset_changed = true;
        } else if y >= self.offset.y.saturating_add(height) {
            self.offset.y = y.saturating_sub(height).saturating_add(1);
            offset_changed = true;
        }

        if x < self.offset.x {
            self.offset.x = x;
            offset_changed = true;
        } else if x >= self.offset.x.saturating_add(width) {
            self.offset.x = x.saturating_sub(width).saturating_add(1);
            offset_changed = true;
        }
        self.needs_redraw = offset_changed;
    }
    fn error_logging(log: String) {
        let mut file = std::fs::OpenOptions::new()
            .create(true)
            .write(true)
            .append(true)
            .open("panic_backtrace.txt")
            .unwrap();
        file.write_all(log.as_bytes()).unwrap();
    }
}
