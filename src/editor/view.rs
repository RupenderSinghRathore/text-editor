use crate::editor::terminal::{Position, Size, Terminal};
use buffer::Buffer;

use std::io::Result;

mod buffer;

const NAME: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");

pub struct View {
    pub buffer: Buffer,
    needs_redraw: bool,
    size: Size,
}

impl View {
    fn render_line(at: usize, line_text: &str) -> Result<()> {
        Terminal::move_caret_to(Position { col: 0, row: at })?;
        Terminal::clear_line()?;
        Terminal::print(line_text)?;
        Ok(())
    }
    pub fn render(&mut self) -> Result<()> {
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
    pub fn needs_redraw(&mut self) {
        self.needs_redraw = true;
    }
    fn welcome_screen_msg(width: usize) -> String {
        let msg = format!("{NAME} - {VERSION}");
        let spaces_needed = " ".repeat((width - msg.len()) / 2);
        spaces_needed + &msg
    }
    pub fn load(&mut self, file: &str) {
        if let Ok(buf) = Buffer::load(file) {
            self.buffer = buf;
        }
    }
}

impl Default for View {
    fn default() -> Self {
        View {
            buffer: Buffer::default(),
            needs_redraw: true,
            size: Terminal::size().unwrap_or_default(),
        }
    }
}
