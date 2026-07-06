use unicode_width::{UnicodeWidthChar, UnicodeWidthStr};

pub trait Utf8 {
    fn grapheme_index_from_width(&self, width: usize) -> usize;
    fn get_str_for_width(&self, left: usize, right: usize) -> String;
    fn display_step_from_grapheme_index(&self, i: usize) -> usize;
    fn display_width(&self) -> usize;
}

impl Utf8 for str {
    fn grapheme_index_from_width(&self, width: usize) -> usize {
        let mut w: usize = 0;
        let mut i = 0;
        for ch in self.chars() {
            if w == width {
                return i;
            }
            w = w.saturating_add(ch.width().unwrap_or(0));
            i += 1;
        }
        i
    }
    fn get_str_for_width(&self, left: usize, right: usize) -> String {
        let mut new_str = String::new();
        let mut w: usize = 0;
        for ch in self.chars() {
            let cw = ch.width().unwrap_or(0);
            if w + cw <= left {
                w += cw;
                continue;
            }
            if w >= right {
                break;
            }
            new_str.push(ch);
            w += cw;
        }
        new_str
    }
    fn display_step_from_grapheme_index(&self, i: usize) -> usize {
        let mut iter = self.chars();
        let mut width = iter
            .nth(i)
            .unwrap_or(self.chars().last().unwrap())
            .width()
            .unwrap_or(0);
        while width == 0 {
            width = iter.next().unwrap().width().unwrap_or(0);
        }
        if width == 1 {
            1
        } else if width == 2 {
            2
        } else {
            panic!("fuck my life");
        }
    }
    fn display_width(&self) -> usize {
        self.width()
    }
}
