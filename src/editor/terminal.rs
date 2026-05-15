use std::io::{self, Write, stdout};

use crossterm::{
    cursor, queue,
    style::Print,
    terminal::{Clear, ClearType, disable_raw_mode, enable_raw_mode, size},
};
pub struct Terminal {}

#[derive(Copy, Clone)]
pub struct Size {
    pub width: u16,
    pub height: u16,
}

#[derive(Copy, Clone)]
pub struct Position {
    pub x: u16,
    pub y: u16,
}

impl Terminal {
    pub fn terminate() -> io::Result<()> {
        disable_raw_mode()?;
        Ok(())
    }
    pub fn initialize() -> io::Result<()> {
        enable_raw_mode()?;
        Self::clear_screen()?;
        Self::move_cursor_to(Position { x: 0, y: 0 })?;
        Ok(())
    }
    pub fn print(s: &str) -> io::Result<()> {
        queue!(stdout(), Print(s))
    }
    pub fn execute() -> io::Result<()> {
        stdout().flush()
    }
    pub fn clear_screen() -> io::Result<()> {
        queue!(stdout(), Clear(ClearType::All))?;
        Ok(())
    }
    pub fn hide_cursor() -> io::Result<()> {
        queue!(stdout(), cursor::Hide)
    }
    pub fn show_cursor() -> io::Result<()> {
        queue!(stdout(), cursor::Show)
    }
    pub fn clear_line() -> io::Result<()> {
        queue!(stdout(), Clear(ClearType::CurrentLine))
    }
    pub fn size() -> io::Result<Size> {
        let (width, height) = size()?;
        Ok(Size { width, height })
    }
    pub fn move_cursor_to(p: Position) -> io::Result<()> {
        queue!(stdout(), cursor::MoveTo(p.x, p.y))
    }
}
