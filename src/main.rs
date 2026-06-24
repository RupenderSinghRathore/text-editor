#![warn(clippy::all, clippy::pedantic, clippy::print_stdout)]

mod editor;
use std::io;

use editor::Editor;

fn main() -> io::Result<()> {
    Editor::new()?.run()?;
    Ok(())
}
