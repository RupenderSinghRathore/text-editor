use std::io::{self, stdout};

use crossterm::{
    cursor, execute,
    terminal::{Clear, ClearType, disable_raw_mode, enable_raw_mode, size},
};
pub struct Terminal {}

impl Terminal {
    pub fn terminate() -> io::Result<()> {
        disable_raw_mode()?;
        Ok(())
    }
    pub fn initialize() -> io::Result<()> {
        enable_raw_mode()?;
        Self::clear_screen()?;
        Self::move_cursor_to(0, 0)?;
        Ok(())
    }
    pub fn clear_screen() -> io::Result<()> {
        execute!(io::stdout(), Clear(ClearType::All))?;
        Ok(())
    }
    pub fn size() -> io::Result<(u16, u16)> {
        size()
    }
    pub fn move_cursor_to(x: u16, y: u16) -> io::Result<()> {
        execute!(stdout(), cursor::MoveTo(x, y))
    }
}
