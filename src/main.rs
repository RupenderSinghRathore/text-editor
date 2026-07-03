#![warn(clippy::all, clippy::pedantic)]

mod editor;
mod logger;
use std::io;

use editor::Editor;

fn main() -> io::Result<()> {
    Editor::new()?.run()?;
    Ok(())
}
