use crate::interpreter::memory::U16_MAX;
use std::ops::{Add, Div, Mul, Rem, Sub};

#[derive(
    Copy, Clone, serde::Serialize, serde::Deserialize, serde_diff::SerdeDiff, Eq, PartialEq,
)]
pub struct Register {
    value: u16,
    altered: bool,
    is_r0: bool,
}

impl Add<u16> for Register {
    type Output = u32;

    fn add(self, rhs: u16) -> Self::Output {
        u32::from(self.value) + u32::from(rhs)
    }
}

impl Add<Register> for Register {
    type Output = u32;

    fn add(self, mut rhs: Register) -> Self::Output {
        u32::from(self.value) + u32::from(rhs.get())
    }
}

impl Sub<Register> for Register {
    type Output = u32;

    fn sub(self, mut rhs: Register) -> u32 {
        u32::from(self.value) + u32::from(rhs.get() ^ 0xffff) + 1
    }
}

impl Mul<Register> for Register {
    type Output = u32;

    fn mul(self, mut rhs: Register) -> u32 {
        u32::from(self.value) * u32::from(rhs.get())
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
        u32::from(value.get())
    }
}

impl Default for Register {
    fn default() -> Self {
        Self::new()
    }
}

impl Register {
    pub fn new() -> Register {
        Register {
            value: 0,
            altered: false,
            is_r0: false,
        }
    }

    // Preincrement
    pub fn peinc(&mut self, amount: u16) -> u16 {
        if self.is_r0 {
            return 0;
        }

        self.altered = true;
        if self.value < U16_MAX {
            self.value += amount;
        } else {
            self.value = 0;
        }
        self.value
    }

    // Postincrement
    pub fn poinc(&mut self, amount: u16) -> u16 {
        if self.is_r0 {
            return 0;
        }
        self.altered = true;
        let temp: u16 = self.value;
        if self.value < U16_MAX {
            self.value += amount;
        } else {
            self.value = 0;
        }
        temp
    }

    pub fn set(&mut self, value: u16) {
        if self.is_r0 {
            return;
        }
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
            return true;
        }
        false
    }

    pub fn set_r0(&mut self) {
        self.is_r0 = true;
    }
}
