use std::io;

use crossterm::event::{Event, KeyCode, KeyEvent, KeyModifiers, read};

mod terminal;
use terminal::{Position, Size, Terminal};

pub struct Editor {
    should_quit: bool,
}

impl Editor {
    pub const fn default() -> Self {
        Self { should_quit: false }
    }
    pub fn run(&mut self) -> io::Result<()> {
        Terminal::initialize()?;
        let result = self.repl();
        Terminal::terminate()?;
        result
    }
    pub fn repl(&mut self) -> io::Result<()> {
        while !self.should_quit {
            self.refresh_screen()?;

            let event = read()?;
            self.eval_event(&event);
        }

        Ok(())
    }
    fn refresh_screen(&mut self) -> io::Result<()> {
        Terminal::hide_cursor()?;
        if self.should_quit {
            Terminal::clear_screen()?;
            Terminal::print("Goodbye. \r")?;
        } else {
            Self::draw_rows()?;
            Terminal::move_cursor_to(Position { x: 0, y: 0 })?;
        }
        Terminal::show_cursor()?;
        Terminal::execute()?;
        Ok(())
    }
    fn eval_event(&mut self, event: &Event) {
        if let Event::Key(KeyEvent {
            code, modifiers, ..
        }) = event
        {
            match code {
                KeyCode::Char('q') if *modifiers == KeyModifiers::CONTROL => {
                    self.should_quit = true;
                }
                _ => (),
            }
        }
    }
    fn draw_rows() -> io::Result<()> {
        let Size { height, .. } = Terminal::size()?;
        for i in 0..height {
            Terminal::clear_line()?;
            Terminal::print("~")?;
            if i + 1 < height {
                Terminal::print("\n\r")?;
            }
        }
        Ok(())
    }
}
