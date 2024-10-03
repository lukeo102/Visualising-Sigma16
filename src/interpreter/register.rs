pub struct Register {
    value: u16
}

impl Register {
    pub fn new() -> Register {
        Register { value: 0 }
    }

    // Preincrement
    pub fn peinc(&mut self, amount: u16) -> u16 {
        self.value += amount;
        return self.value;
    }

    // Postincrement
    pub fn poinc(&mut self, amount: u16) -> u16 {
        let temp: u16 = self.value;
        self.value += amount;
        return temp;
    }

    pub fn set(&mut self, value: u16) {
        self.value = value;
    }

    pub fn get(&mut self) -> u16 {
        self.value
    }
}