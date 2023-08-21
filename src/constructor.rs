#[derive(Default)]
pub struct Second {
    value: u64
}

impl Second {
    pub fn new(value: u64) -> Self {
        Self { value }
    }

    pub fn value(&self) -> u64 {
        self.value
    }

    pub fn set(&mut self, value: u64) {
        self.value = value;
    }
}

fn main() {
    let mut s = Second::default();
    
    s.set(100);

    println!("{}", s.value());
}
