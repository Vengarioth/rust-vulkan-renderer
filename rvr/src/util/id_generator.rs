pub struct IdGenerator {
    next: u32,
}

impl IdGenerator {
    pub fn new() -> Self {
        Self {
            next: 0,
        }
    }

    pub fn next(&mut self) -> u32 {
        let next = self.next;
        self.next += 1;
        next
    }
}
