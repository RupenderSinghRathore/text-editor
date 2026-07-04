use crossterm::event::KeyCode;

use crate::editor::view::{self, Location, View};

impl View {
    pub fn edit_document(&mut self, key: KeyCode) {
        let lines = self.buffer.mut_lines();
        self.location = match key {
            KeyCode::Char(c) => self.write_char(c),
            KeyCode::Backspace => self.handle_backspace(),
            KeyCode::Enter => self.handle_enter(),
            _ => self.location,
        };
        self.unsaved_changes = true;
        self.needs_redraw = true;
    }
    fn write_char(&mut self, c: char) -> Location {
        let Location { mut x, y } = self.location;
        let lines = self.buffer.mut_lines();

        if y == lines.len() {
            lines.push(c.to_string());
        } else if let Some(line) = lines.get_mut(y) {
            line.insert(x, c);
        }
        x = x.saturating_add(1);

        Location { x, y }
    }
    fn handle_backspace(&mut self) -> Location {
        let Location { mut x, mut y } = self.location;

        if x == 0 {
            if let Some(curr_line) = self.buffer.cloned_line(y)
                && y > 0
                && let Some(prev_line) = self.buffer.mut_line(y - 1)
            {
                x = prev_line.len();
                prev_line.push_str(curr_line.as_ref());
                self.buffer.mut_lines().remove(y);
                y = y.saturating_sub(1);
            }
        } else if let Some(line) = self.buffer.mut_line(y) {
            x = x.saturating_sub(1);
            line.remove(x);
        }

        Location { x, y }
    }
    fn handle_enter(&mut self) -> Location {
        let Location { mut x, mut y } = self.location;

        if let Some(curr_line) = self.buffer.mut_line(y) {
            y = y.saturating_add(1);

            let new_line = curr_line.split_off(x);
            self.buffer.mut_lines().insert(y, new_line);

            x = 0;
        }

        Location { x, y }
    }
}
