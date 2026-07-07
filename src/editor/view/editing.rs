use crate::editor::view::string::Utf8;
use crossterm::event::KeyCode;
use unicode_segmentation::UnicodeSegmentation;
use unicode_width::UnicodeWidthChar;

use crate::editor::view::{Location, View};

impl View {
    pub fn edit_document(&mut self, key: KeyCode) {
        self.location = match key {
            KeyCode::Char(c) => self.write_char(c),
            KeyCode::Backspace => self.handle_backspace(),
            KeyCode::Enter => self.handle_enter(),
            _ => self.location,
        };
        self.scroll_location_into_view();
        self.unsaved_changes = true;
        self.needs_redraw = true;
    }
    fn write_char(&mut self, c: char) -> Location {
        let Location { mut x, y } = self.location;
        let lines = self.buffer.mut_lines();

        if let Some(line) = lines.get_mut(y) {
            Self::insert_grapheme_at_display_width(line, c, x);
            x = x.saturating_add(c.width().unwrap_or(0));
        }

        Location { x, y }
    }
    fn handle_backspace(&mut self) -> Location {
        let Location { mut x, mut y } = self.location;

        if x == 0 {
            if let Some(curr_line) = self.buffer.cloned_line(y)
                && y > 0
                && let Some(prev_line) = self.buffer.mut_line(y - 1)
            {
                x = prev_line.display_width();
                prev_line.push_str(curr_line.as_ref());
                self.buffer.mut_lines().remove(y);
                y = y.saturating_sub(1);
            }
        } else if let Some(line) = self.buffer.mut_line(y) {
            let prev_grapheme_index = line.grapheme_index_from_width(x).saturating_sub(1);
            let step = line.display_step_from_grapheme_index(prev_grapheme_index);
            x = x.saturating_sub(step);
            Self::remove_grapheme(line, prev_grapheme_index);
        }

        Location { x, y }
    }
    fn handle_enter(&mut self) -> Location {
        let Location { mut x, mut y } = self.location;

        if let Some(curr_line) = self.buffer.mut_line(y) {
            y = y.saturating_add(1);

            let offset_index = curr_line.offset_index_from_width(x);
            let new_line = if offset_index != curr_line.len() {
                curr_line.split_off(offset_index)
            } else {
                String::new()
            };
            self.buffer.mut_lines().insert(y, new_line);

            x = 0;
        }

        Location { x, y }
    }
    fn remove_grapheme(s: &mut String, i: usize) {
        let mut iter = s.grapheme_indices(true);
        let start = match iter.nth(i) {
            Some((start, _)) => start,
            None => return,
        };
        let end = iter.next().map_or(s.len(), |(end, _)| end);
        s.replace_range(start..end, "")
    }
    fn insert_grapheme_at_display_width(s: &mut String, ch: char, w: usize) {
        let grapheme_idx = s.grapheme_index_from_width(w);
        let mut iter = s.grapheme_indices(true);
        let offset_idx = match iter.nth(grapheme_idx) {
            Some((start, _)) => start,
            None => {
                if grapheme_idx == 0 {
                    0
                } else {
                    s.push(ch);
                    return;
                }
            }
        };
        s.insert(offset_idx, ch);
    }
}
