mod editor;
mod logger;
use std::io;

use editor::Editor;

const APP_NAME: &str = "text-editor";

fn main() -> io::Result<()> {
    Editor::new()?.run()?;
    Ok(())
}
