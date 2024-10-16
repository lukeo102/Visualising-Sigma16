use std::ops::{Add, Sub, Mul, Div, Rem};
use crate::interpreter::memory::U16_MAX;


#[derive(Copy, Clone)]
pub struct Register {
    value: u16,
    altered: bool,
    is_r0: bool,
}

impl Add<u16> for Register {
    type Output = u32;

    fn add(self, rhs: u16) -> Self::Output {
        self.value as u32 + rhs as u32
    }
}

impl Add<Register> for Register {
    type Output = u32;

    fn add(self, mut rhs: Register) -> Self::Output {
        self.value as u32 + rhs.get() as u32
    }
}

impl Sub<Register> for Register {
    type Output = u32;

    fn sub(self, mut rhs: Register) -> u32 {
        self.value as u32 + (rhs.get() ^ 0xffff) as u32 + 1 
    }
}

impl Mul<Register> for Register {
    type Output = u32;

    fn mul(self, mut rhs: Register) -> u32 {
        self.value as u32 * rhs.get() as u32
    }
}

impl Div<Register> for Register {
    type Output = u16;

    fn div(self, mut rhs: Register) -> u16 {
        self.value / rhs.get()
    }
}

impl Rem<Register> for Register {
    type Output = u16;

    fn rem(self, mut rhs: Register) -> u16 {
        self.value % rhs.get()
    }
}

impl From<Register> for u8 {
    fn from(mut value: Register) -> Self {
        value.get() as u8
    }
}

impl From<Register> for u16 {
    fn from(mut value: Register) -> Self {
        value.get()
    }
}

impl From<Register> for u32 {
    fn from(mut value: Register) -> Self {
        value.get() as u32
    }
}


impl Register {
    pub fn new() -> Register {
        Register { value: 0, altered: false, is_r0: false }
    }

    // Preincrement
    pub fn peinc(&mut self, amount: u16) -> u16 {
        if self.is_r0 { return 0 }
        
        self.altered = true;
        if self.value < U16_MAX {
            self.value += amount;
        } else {
            self.value = 0;
        }
        return self.value;
    }

    // Postincrement
    pub fn poinc(&mut self, amount: u16) -> u16 {
        if self.is_r0 { return 0 }
        self.altered = true;
        let temp: u16 = self.value;
        if self.value < U16_MAX {
            self.value += amount;
        } else {
            self.value = 0;
        }
        return temp;
    }

    pub fn set(&mut self, value: u16) {
        if self.is_r0 { return }
        self.altered = true;
        self.value = value;
    }

    pub fn get(&mut self) -> u16 {
        self.altered = true;        
        self.value
    }
    
    pub fn get_altered(&mut self) -> bool {
        if self.altered {
            // self.altered = false;
            return true
        }
        false
    }
    
    pub fn set_r0(&mut self) {
        self.is_r0 = true;
    }
}