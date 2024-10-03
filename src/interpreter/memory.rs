pub struct Memory {
    contents: [u16; 65535]
}

impl std::ops::Index<u16> for Memory {
    type Output = u16;

    fn index(&self, i: u16) -> u16 {
        self.contents[i]
    }
}

impl std::ops::IndexMut<u16> for Memory {
    fn index_mut(&mut self, i: u16) -> &mut u16 { &mut self.contents[i] }
}

impl Memory {
     pub fn new(init: Option<&[u16]>) -> Memory {
         Memory {
             contents: match init {
                 Some(init) => init.clone().try_into().expect("Too much memory!"),
                 None => [0u16; 65535],
             }
         }
     }
 }

pub fn word_to_nibbles(word: u16) -> [u8;4] {
    [
        (word & 0b0000000000001111) as u8,
        ((word >> 4) & 0b0000000000001111) as u8,
        ((word >> 8) & 0b0000000000001111) as u8,
        ((word >> 12) & 0b0000000000001111) as u8,
    ]
}