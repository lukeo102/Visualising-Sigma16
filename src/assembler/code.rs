use crate::assembler::assembler::Assembler;
use log::{log, Level};
use std::collections::HashMap;

use super::error::AssemblingError;

#[derive(serde::Serialize, serde::Deserialize)]
pub(crate) struct Code {
    pub memory: Vec<u16>,
    pub code: String,
    pub memory_to_code: HashMap<usize, usize>,
    pub symbol_table: HashMap<String, usize>,
    pub errors: Vec<AssemblingError>,
}

impl Code {
    pub fn new(code: String) -> Code {
        let mut assembler = Assembler::new(code);
        assembler.assemble();

        if !assembler.errors.is_empty() {
            for error in assembler.errors.clone() {
                log!(
                    Level::Error,
                    "message: {}\nline: {}\n resolution {}",
                    error.message,
                    error.line,
                    error.resolution
                );
            }
        }

        Self {
            memory: assembler.assembled,
            code: assembler.code,
            memory_to_code: assembler.mem_to_code,
            symbol_table: assembler.symbol_table,
            errors: assembler.errors,
        }
    }

    pub fn code_line_from_mem_loc(&self, mem_loc: usize) -> (String, usize) {
        let lines = self.code.lines().collect::<Vec<&str>>();
        let line = self.memory_to_code[&mem_loc] - 1;
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
