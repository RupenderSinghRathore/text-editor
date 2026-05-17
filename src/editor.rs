use std::io::Result;

use core::cmp::min;
use crossterm::event::{Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers, read};

mod terminal;
use terminal::{Position, Size, Terminal};

const NAME: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");

const MIN_HEIGHT: usize = 0;
const MIN_WIDTH: usize = 2;

#[derive(Debug, Clone, Copy, Default)]
struct Location {
    x: usize,
    y: usize,
}

#[derive(Default)]
pub struct Editor {
    should_quit: bool,
    location: Location,
}

impl Editor {
    pub fn run(&mut self) -> Result<()> {
        Terminal::initialize()?;

        Terminal::move_caret_to(Position::default())?;
        Self::draw_rows()?;

        let result = self.repl();
        Terminal::terminate()?;
        result
    }
    pub fn repl(&mut self) -> Result<()> {
        while !self.should_quit {
            self.refresh_screen()?;

            let event = read()?;
            self.eval_event(&event)?;
        }

        Ok(())
    }
    fn refresh_screen(&mut self) -> Result<()> {
        Terminal::hide_caret()?;
        if self.should_quit {
            Terminal::clear_screen()?;
            Terminal::print("Goodbye. \r")?;
        } else {
            Terminal::move_caret_to(Position {
                col: self.location.x,
                row: self.location.y,
            })?;
        }
        Terminal::show_caret()?;
        Terminal::execute()?;
        Ok(())
    }
    fn eval_event(&mut self, event: &Event) -> Result<()> {
        if let Event::Key(KeyEvent {
            code,
            modifiers,
            kind: KeyEventKind::Press,
            ..
        }) = event
        {
            match code {
                KeyCode::Char('q') if *modifiers == KeyModifiers::CONTROL => {
                    self.should_quit = true;
                }
                KeyCode::Up
                | KeyCode::Down
                | KeyCode::Left
                | KeyCode::Right
                | KeyCode::PageUp
                | KeyCode::PageDown
                | KeyCode::Home
                | KeyCode::End => {
                    self.move_caret(*code)?;
                }
                _ => (),
            }
        }
        Ok(())
    }
    fn move_caret(&mut self, code: KeyCode) -> Result<()> {
        let Location { mut x, mut y } = self.location;
        let Size { width, height } = Terminal::size()?;
        match code {
            KeyCode::Up => {
                y = y.saturating_sub(1);
            }
            KeyCode::Down => {
                y = min(y.saturating_add(1), height.saturating_sub(1));
            }
            KeyCode::Left => {
                x = x.saturating_sub(1);
            }
            KeyCode::Right => {
                x = min(x.saturating_add(1), width.saturating_sub(1));
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
        Ok(())
    }
    fn draw_rows() -> Result<()> {
        let Size { height, .. } = Terminal::size()?;
        for i in 0..height {
            Terminal::clear_line()?;
            Terminal::print("~")?;
            if i == height / 3 {
                Self::welcome_screen()?;
            }
            if i + 1 < height {
                Terminal::print("\n\r")?;
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
