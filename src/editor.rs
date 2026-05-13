use std::io::{self, Read};
pub struct Editor {}

impl Editor {
    pub fn default() -> Self {
        Editor {}
    }
    pub fn run(&self) {
        if let Err(e) = crossterm::terminal::enable_raw_mode() {
            println!("Error: {e}");
        }
        let handle = io::stdin().lock();
        for b in handle.bytes() {
            let b = b.unwrap();
            let c = b as char;

            if c.is_control() {
                println!("Binary: {b:08b} ASCII: {b:#03} \r");
            } else {
                println!("Binary: {b:08b} ASCII: {b:#03} Character: {c:#?} \r");
            }

            match c {
                'q' | '\u{3}' => {
                    crossterm::terminal::disable_raw_mode().unwrap();
                    break;
                }
                _ => (),
            }
        }
    }
}
