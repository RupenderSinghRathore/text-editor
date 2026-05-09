use std::io::{self, Read};

fn main() {
    crossterm::terminal::enable_raw_mode().unwrap();
    let handle = io::stdin().lock();
    for b in handle.bytes() {
        let b = b.unwrap();
        let c = b as char;

        if c.is_control() {
            println!("Binary: {0:08b} ASCII: {0:#03} \r", b);
        } else {
            println!("Binary: {0:08b} ASCII: {0:#03} Character: {1:#?} \r", b, c);
        }

        match c {
            'q' | '\u{3}' => {
                crossterm::terminal::disable_raw_mode().unwrap();
                break;
            }
            _ => (),
        }
        if c == 'q' {
            crossterm::terminal::disable_raw_mode().unwrap();
            break;
        }
        // println!("{c}\r");
    }
}
