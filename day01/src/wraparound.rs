pub struct WraparoundCounter {
    pub current_value: i32,
    pub target_number: i32,
    pub passed_through: i32,
    pub stopped_at: i32,
    pub max: i32,
    pub min: i32,
}

impl WraparoundCounter {
    pub fn new(current_value: i32, target_number: i32, min: i32, max: i32) -> Self {
        Self {
            current_value,
            target_number,
            min,
            max,
            passed_through: 0,
            stopped_at: 0,
        }
    }
    pub fn incr(&mut self) {
        self.current_value += 1;
    }
    pub fn decr(&mut self) {
        self.current_value -= 1;
    }
    pub fn add(&mut self, mut number: i32) {
        if number == 0 {
            return;
        }
        while number > 0 {
            self.incr();
            number -= 1;
            if self.current_value > self.max {
                self.current_value = self.min
            }
            if self.current_value == self.target_number {
                self.passed_through += 1;
            }
        }
        while number < 0 {
            self.decr();
            number += 1;
            if self.current_value < self.min {
                self.current_value = self.max
            }
            if self.current_value == self.target_number {
                self.passed_through += 1;
            }
        }
        if self.current_value == self.target_number {
            self.stopped_at += 1;
        }
    }
}
