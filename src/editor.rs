use std::{backtrace, env, io::Result};

use crossterm::event::{Event, KeyCode, KeyModifiers, read};

use terminal::{Size, Terminal};
use view::View;

mod terminal;
mod view;

#[derive(Default)]
pub struct Editor {
    should_quit: bool,
    view: View,
}

impl Editor {
    pub fn new() -> Result<Self> {
        std::panic::set_hook(Box::new(move |_| {
            Terminal::terminate().unwrap();
            let backtrace = backtrace::Backtrace::capture();
            // std::fs::write("panic_backtrace.txt", format!("{backtrace}\n")).unwrap();
            println!("{backtrace}");
        }));
        Terminal::initialize()?;
        Ok(Self {
            should_quit: false,
            view: View::new(),
        })
    }
    pub fn run(&mut self) -> Result<()> {
        let args: Vec<String> = env::args().collect();
        if let Some(arg) = args.get(1) {
            self.view.load(arg);
        }
        while !self.should_quit {
            self.view.refresh_screen()?;

            let event = read()?;
            self.eval_event(&event);
        }

        Ok(())
    }
    fn eval_event(&mut self, event: &Event) {
        match event {
            // Event::Key enum field wraps a KeyEvent struct
            Event::Key(event) => match event.code {
                KeyCode::Char('c') if event.modifiers == KeyModifiers::CONTROL => {
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
                    self.view.move_caret(event.code);
                }
                KeyCode::Char(x) => {
                    self.view.write_char(x);
                }
                KeyCode::Backspace => {
                    self.view.handle_backspace()
                }
                _ => (),
            },

            Event::Resize(col, row) => {
                let width = *col as usize;
                let height = *row as usize;
                self.view.resize(Size { width, height });
            }
            _ => (),
        }
    }
}

impl Drop for Editor {
    fn drop(&mut self) {
        _ = Terminal::terminate();
        if self.should_quit {
            let _ = Terminal::print("goodbye..");
        }
    }
}
