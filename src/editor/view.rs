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
        let Size { height, .. } = Terminal::size()?;

        let rendered_rows = self.buffer.len().min(height);

        for v in self.buffer.lines().iter().take(height) {
            Terminal::clear_line()?;
            Terminal::print(v)?;
            Terminal::print("\r\n")?;
        }

        for i in rendered_rows..height {
            Terminal::clear_line()?;
            Terminal::print("~")?;

            if i == height / 3 {
                Self::welcome_screen()?;
            }

            if i + 1 < height {
                Terminal::print("\r\n")?;
            }
        }
        Ok(())
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
}
