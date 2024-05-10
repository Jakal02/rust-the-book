pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 10 {
            panic!("Guess value must be between 1 and 10, got {}", value);
        }
    Guess { value }
    }

    pub fn value(&self) -> i32 {
        self.value
    }
}