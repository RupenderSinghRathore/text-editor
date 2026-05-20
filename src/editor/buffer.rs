use io::Result;
use std::{fs, io};

#[derive(Default)]
pub struct Buffer {
    lines: Vec<String>,
}

impl Buffer {
    // pub fn default() -> Self {
    //     Self {
    //         lis: vec![(String::from("Hello World!"))],
    //     }
    // }
    pub fn len(&self) -> usize {
        self.lines.len()
    }
    pub fn lines(&self) -> &[String] {
        &self.lines
    }
    pub fn add(&mut self, s: String) {
        self.lines.push(s);
    }
    pub fn load(file: &str) -> Result<Self> {
        let content = fs::read_to_string(file)?;

        let mut lines: Vec<String> = vec![];
        for line in content.lines() {
            lines.push(line.to_string());
        }
        Ok(Self { lines })
    }
    pub fn is_empty(&self) -> bool {
        self.lines.is_empty()
    }
}
