use crate::assembler::assembler::Assembler;
use std::collections::HashMap;

pub(crate) struct Code {
    pub memory: Vec<u16>,
    pub code: String,
    pub memory_to_code: HashMap<usize, usize>,
    pub symbol_table: HashMap<String, usize>,
}

impl Code {
    pub fn new(code: String) -> Code {
        let mut assembler = Assembler::new(code.clone());
        assembler.assemble();

        Self {
            memory: assembler.assembled,
            code: assembler.code,
            memory_to_code: assembler.mem_to_code,
            symbol_table: assembler.symbol_table,
        }
    }

    pub fn code_line_from_mem_loc(&self, mem_loc: usize) -> (String, usize) {
        let lines = self.code.lines().collect::<Vec<&str>>();
        let line = self.memory_to_code[&mem_loc];
        (lines[line].to_string(), line)
    }

    pub fn get_code(&self) -> String {
        self.code.clone()
    }
    pub fn get_memory(&self) -> Vec<u16> {
        self.memory.clone()
    }
    pub fn get_lines_of_code(&self) -> usize {
        self.code.lines().count()
    }
    pub fn get_memory_location_count(&self) -> usize {
        self.memory.len()
    }
}
