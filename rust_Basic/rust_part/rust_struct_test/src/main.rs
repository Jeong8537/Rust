struct Calculator {
    value: i32,
}

impl Calculator {
    fn new() -> Self {
        Self { value: 0 }
    }

    fn add(&mut self, num: i32) {
        self.value += num;
    }
    fn subtract(&mut self, num: i32) {
        self.value -= num;
    }
}

