use unicode_segmentation::UnicodeSegmentation;
use unicode_width::UnicodeWidthStr;

pub trait Utf8 {
    fn grapheme_index_from_width(&self, width: usize) -> usize;
    fn offset_index_from_width(&self, width: usize) -> usize;
    fn get_str_for_width(&self, left: usize, right: usize) -> String;
    fn display_step_from_grapheme_index(&self, i: usize) -> usize;
    fn display_width(&self) -> usize;
}

impl Utf8 for str {
    fn grapheme_index_from_width(&self, width: usize) -> usize {
        let mut w: usize = 0;
        let mut i = 0;
        for ch in self.graphemes(true) {
            if w == width {
                return i;
            }
            w = w.saturating_add(ch.width());
            i += 1;
        }
        i
    }
    fn offset_index_from_width(&self, width: usize) -> usize {
        let mut w: usize = 0;
        for (i, ch) in self.grapheme_indices(true) {
            if w == width {
                return i;
            }
            w = w.saturating_add(ch.width());
        }
        self.len()
    }
    fn get_str_for_width(&self, left: usize, right: usize) -> String {
        let mut new_str = String::new();
        let mut w: usize = 0;
        for ch in self.graphemes(true) {
            let cw = ch.width();
            if w + cw <= left {
                w += cw;
                continue;
            }
            if w >= right {
                break;
            }
            new_str.push_str(ch);
            w += cw;
        }
        new_str
    }
    fn display_step_from_grapheme_index(&self, i: usize) -> usize {
        let mut iter = self.graphemes(true);
        let mut width = iter
            .nth(i)
            .unwrap_or(self.graphemes(true).last().unwrap())
            .width();
        while width == 0 {
            width = iter.next().unwrap().width();
        }
        width
    }
    fn display_width(&self) -> usize {
        self.width()
    }
}
