use std::io::{self, Write, stdout};

use crossterm::event::{Event, KeyCode, KeyEvent, KeyModifiers, read};

mod terminal;
use terminal::Terminal;

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
            self.eval_event(event)?;
        }

        Ok(())
    }
    fn refresh_screen(&mut self) -> io::Result<()> {
        if self.should_quit {
            Terminal::clear_screen()?;
            println!("Goodbye. \r");
        } else {
            Self::draw_rows()?;
            Terminal::move_cursor_to(0, 0)?;
        }
        Ok(())
    }
    fn eval_event(&mut self, event: Event) -> io::Result<()> {
        if let Event::Key(KeyEvent {
            code, modifiers, ..
        }) = event
        {
            match code {
                KeyCode::Char('q') if modifiers == KeyModifiers::CONTROL => self.should_quit = true,
                _ => (),
            }
        }
        Ok(())
    }
    fn draw_rows() -> io::Result<()> {
        let height = Terminal::size()?.1;
        for i in 0..height {
            print!("~");
            if i + 1 < height {
                print!("\n\r");
            }
        }
        stdout().flush()?;
        Ok(())
    }
}
