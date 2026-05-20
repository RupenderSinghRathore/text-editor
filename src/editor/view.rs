use crate::editor::{
    buffer::Buffer,
    terminal::{Size, Terminal},
};

use std::io::Result;

const NAME: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Default)]
pub struct View {
    pub buffer: Buffer,
}

impl View {
    pub fn render(&self) -> Result<()> {
        if self.buffer.is_empty() {
            Self::render_welcome_screen()
        } else {
            self.render_buffer()
        }
    }
    fn render_buffer(&self) -> Result<()> {
        let Size { height, .. } = Terminal::size()?;
        let lines = self.buffer.lines();
        for i in 0..height {
            Terminal::clear_line()?;

            if let Some(line) = lines.get(i) {
                Terminal::print(line)?;
            } else {
                Self::blank_line()?;
            }

            if i + 1 < height {
                Terminal::print("\r\n")?;
            }
        }
        Ok(())
    }
    fn render_welcome_screen() -> Result<()> {
        let Size { height, .. } = Terminal::size()?;
        for i in 0..height {
            Terminal::clear_line()?;

            if i == height / 3 {
                Self::welcome_screen()?;
            } else {
                Self::blank_line()?;
            }

            if i + 1 < height {
                Terminal::print("\r\n")?;
            }
        }
        Ok(())
    }
    fn blank_line() -> Result<()> {
        Terminal::print("~")
    }
    fn welcome_screen() -> Result<()> {
        let Size { width, .. } = Terminal::size()?;
        let msg = format!("{NAME} - {VERSION}");
        let msg_len = msg.len();
        let x = width.saturating_sub(msg_len) / 2;
        Terminal::move_caret_to_column(x)?;
        Terminal::print(&msg)?;
        Ok(())
    }
    pub fn load(&mut self, file: &str) {
        if let Ok(buf) = Buffer::load(file) {
            self.buffer = buf;
        }
    }
}
