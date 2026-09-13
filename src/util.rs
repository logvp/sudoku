use std::cell::Cell;

pub struct Counter {
    name: &'static str,
    data: Cell<usize>,
}
impl Counter {
    pub fn new(name: &'static str) -> Self {
        Self {
            name,
            data: Cell::new(0),
        }
    }
    pub fn inc(&self) {
        self.data.update(|n| n + 1);
    }
}
impl Drop for Counter {
    fn drop(&mut self) {
        println!("Counter {}: {}", self.name, self.data.get())
    }
}
