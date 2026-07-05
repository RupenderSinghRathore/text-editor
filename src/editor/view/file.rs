use crate::logger::log;
use std::{
    fs::OpenOptions,
    io::{BufWriter, Write},
};

use crate::editor::view::View;

impl View {
    pub fn save_file(&mut self) {
        if !self.unsaved_changes {
            return;
        }
        // save file
        let file = match OpenOptions::new()
            .write(true)
            .truncate(true)
            .create(true)
            .open(&self.filename)
        {
            Result::Ok(f) => f,
            Err(e) => {
                log(e);
                return;
            }
        };
        let mut buf_file = BufWriter::new(file);

        if let Err(e) = buf_file.write(self.buffer.content().as_slice()) {
            log(e);
            return;
        };
        self.unsaved_changes = false;
        self.needs_redraw = true;
    }
}
