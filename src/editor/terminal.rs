use std::io::{Result, Write, stdout};

use crossterm::{
    Command, cursor, queue,
    style::Print,
    terminal::{self, Clear, ClearType, disable_raw_mode, enable_raw_mode, size},
};

#[derive(Copy, Clone, Default)]
pub struct Size {
    pub width: usize,
    pub height: usize,
}

#[derive(Copy, Clone, Default)]
pub struct Position {
    pub col: usize,
    pub row: usize,
}

pub struct Terminal;

impl Terminal {
    pub fn initialize() -> Result<()> {
        enable_raw_mode()?;
        Self::enter_alt_screen()?;
        Ok(())
    }
    pub fn terminate() -> Result<()> {
        disable_raw_mode()?;
        Self::leave_alt_screen()?;
        Ok(())
    }
    pub fn print(s: &str) -> Result<()> {
        Self::queue_command(Print(s))
    }
    pub fn queue_command<T: Command>(cmd: T) -> Result<()> {
        queue!(stdout(), cmd)
    }
    pub fn execute() -> Result<()> {
        stdout().flush()
    }
    pub fn hide_caret() -> Result<()> {
        Self::queue_command(cursor::Hide)
    }
    pub fn show_caret() -> Result<()> {
        Self::queue_command(cursor::Show)
    }
    pub fn clear_line() -> Result<()> {
        Self::queue_command(Clear(ClearType::CurrentLine))
    }
    pub fn size() -> Result<Size> {
        let (width_u16, height_u16) = size()?;

        let (width, height) = (width_u16 as usize, height_u16 as usize);
        Ok(Size { width, height })
    }
    fn as_u16(a: usize) -> u16 {
        u16::try_from(a).unwrap()
    }
    pub fn move_caret_to(p: Position) -> Result<()> {
        Self::queue_command(cursor::MoveTo(Self::as_u16(p.col), Self::as_u16(p.row)))
    }
    pub fn enter_alt_screen() -> Result<()> {
        Self::queue_command(terminal::EnterAlternateScreen)
    }
    pub fn leave_alt_screen() -> Result<()> {
        Self::queue_command(terminal::LeaveAlternateScreen)
    }
}
