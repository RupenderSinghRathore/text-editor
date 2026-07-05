use io::Result;
use std::{
    fs,
    io::{self, Write},
};

#[derive(Default)]
pub struct Buffer {
    lines: Vec<String>,
}

impl Buffer {
    pub fn len(&self) -> usize {
        self.lines.len()
    }
    pub fn lines(&self) -> &[String] {
        &self.lines
    }
    pub fn mut_lines(&mut self) -> &mut Vec<String> {
        &mut self.lines
    }
    pub fn line(&self, i: usize) -> Option<&String> {
        self.lines.get(i)
    }
    pub fn mut_line(&mut self, i: usize) -> Option<&mut String> {
        self.lines.get_mut(i)
    }
    pub fn cloned_line(&self, i: usize) -> Option<String> {
        Some(self.lines.get(i)?.to_owned())
    }
    pub fn load_file(file: &str) -> Result<Self> {
        let content = fs::read_to_string(file)?;

        let mut lines: Vec<String> = vec![];
        for line in content.lines() {
            lines.push(line.to_string());
        }
        Ok(Self { lines })
    }
    pub fn load_empty() -> Self {
        let lines: Vec<String> = vec![];
        Self { lines }
    }
    pub fn is_empty(&self) -> bool {
        self.lines.is_empty()
    }
    pub fn content(&self) -> Vec<u8> {
        let mut buf = Vec::new();
        for s in self.lines() {
            buf.write_all(s.as_bytes()).unwrap();
            buf.push(b'\n');
        }
        buf
    }
}
