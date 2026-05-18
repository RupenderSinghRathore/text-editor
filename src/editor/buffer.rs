#[derive(Default)]
pub struct Buffer {
    lis: Vec<String>,
}

impl Buffer {
    // pub fn default() -> Self {
    //     Self {
    //         lis: vec![(String::from("Hello World!"))],
    //     }
    // }
    pub fn len(&self) -> usize {
        self.lis.len()
    }
    pub fn lines(&self) -> &[String] {
        &self.lis
    }
    pub fn add_line(&mut self, s: String) {
        self.lis.push(s);
    }
}
