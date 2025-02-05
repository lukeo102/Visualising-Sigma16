pub const U16_MAX: u16 = 65535;

#[derive(serde::Serialize, serde::Deserialize, serde_diff::SerdeDiff, Clone)]
pub struct Memory {
    contents: Vec<u16>,
    accessed_i: Vec<usize>,
    mem_used: Vec<usize>,
    monitored: Vec<usize>,
}

impl std::ops::Index<usize> for Memory {
    type Output = u16;

    fn index(&self, i: usize) -> &u16 {
        &self.contents[i]
    }
}

impl std::ops::IndexMut<usize> for Memory {
    fn index_mut(&mut self, i: usize) -> &mut u16 {
        self.accessed_i.push(i);

        if !self.mem_used.contains(&i) {
            self.mem_used.push(i);
            self.mem_used.sort_unstable();
        }

        &mut self.contents[i]
    }
}

impl Memory {
    pub fn new(init: Option<&[u16]>) -> Memory {
        let mut mem = Memory {
            mem_used: Vec::new(),
            contents: Vec::with_capacity(U16_MAX as usize),
            accessed_i: Vec::new(),
            monitored: Vec::new(),
        };
        if let Some(init) = init {
            for i in 0..init.len() {
                mem.contents.push(init[i]);
                mem.mem_used.push(i);
            }
        }
        mem
    }

    pub fn monitor(&mut self, address: usize) {
        match self.monitored.iter().position(|a| *a == address) {
            Some(index) => {
                self.monitored.remove(index);
            }
            None => {
                self.monitored.push(address);
            }
        }
    }

    pub fn get_monitored(&self) -> Option<Vec<usize>> {
        let mut result: Vec<usize> = Vec::new();
        self.monitored.iter().for_each(|n| {
            if self.accessed_i.contains(n) {
                result.push(*n)
            }
        });
        if result.len() > 0 {
            Some(result)
        } else {
            None
        }
    }

    pub fn reset_accessed(&mut self) {
        self.accessed_i = Vec::new();
    }

    pub fn get_altered_i(&self) -> &[usize] {
        &self.accessed_i
    }

    pub fn get_used(&self) -> &[usize] {
        &self.mem_used
    }
}

pub fn word_to_nibbles(word: u16) -> [u8; 4] {
    [
        (word & 0b0000_0000_0000_1111) as u8,
        ((word >> 4) & 0b0000_0000_0000_1111) as u8,
        ((word >> 8) & 0b0000_0000_0000_1111) as u8,
        ((word >> 12) & 0b0000_0000_0000_1111) as u8,
    ]
}
