use crate::assembler::code::Code;
use crate::interpreter::memory::Memory;
use crate::interpreter::register::Register;
use log::{log, Level};
use std::collections::HashMap;
use std::fmt::Display;

#[derive(serde::Serialize, serde::Deserialize, PartialEq, Debug, Clone, serde_diff::SerdeDiff)]
pub enum RunningState {
    Error,
    Running,
    Step,
    Paused,
    Breakpoint,
    Haulted,
    Interrupted,
}
impl Display for RunningState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            RunningState::Error => "ERROR".to_string(),
            RunningState::Running => "RUNNING".to_string(),
            RunningState::Paused => "PAUSED".to_string(),
            RunningState::Breakpoint => "BREAKPOINT".to_string(),
            RunningState::Haulted => "HALTED".to_string(),
            RunningState::Interrupted => "INTERRUPTED".to_string(),
            RunningState::Step => "STEP".to_string(),
        };
        write!(f, "{str}")
    }
}

#[derive(serde::Serialize, serde::Deserialize, serde_diff::SerdeDiff, Clone)]
pub struct State {
    pub pc: Register,
    pub r: [Register; 16],
    pub memory: Memory,
    pub state: RunningState,
    pub verbose: bool,
    pub symbol_table: HashMap<String, usize>,
    pub monitored_symbols: Vec<(String, bool)>,
    pub monitored_addresses: Vec<u16>,
    pub monitored_registers: [bool; 16],
}

impl State {
    pub fn new(code: &Code) -> State {
        let run_state = if code.errors.is_empty() {
            log!(Level::Info, "Empty, {:?}", code.errors.len());
            RunningState::Step
        } else {
            log!(Level::Info, "Not empty, {:?}", code.errors.len());
            RunningState::Error
        };

        let mut state = State {
            pc: (Register::new()),
            r: [Register::new(); 16],
            memory: (Memory::new(Option::from(code.memory.as_slice()))),
            state: run_state,
            verbose: false,
            symbol_table: code.symbol_table.clone(),
            monitored_symbols: {
                let mut symbols = Vec::new();
                for key in code.symbol_table.keys() {
                    symbols.push((key.clone(), false));
                }
                symbols
            },
            monitored_addresses: Vec::new(),
            monitored_registers: [false; 16],
        };
        state.r[0].set_r0();
        state
    }

    pub fn monitored_accessed(mut self) -> Vec<MonitorType> {
        let mut monitored: Vec<MonitorType> = Vec::new();
        for (reg, monitor) in self.monitored_registers.iter().enumerate() {
            if *monitor {
                if self.r[reg].get_altered() {
                    monitored.push(MonitorType::Register(reg));
                }
            }
        }

        for (addr, monitor) in self.monitored_registers.iter().enumerate() {
            if *monitor {
                if self.memory.get_altered_i().contains(&addr) {
                    monitored.push(MonitorType::Address(addr));
                }
            }
        }

        for (symbol, monitor) in self.monitored_symbols {
            if monitor {
                if self
                    .memory
                    .get_altered_i()
                    .contains(self.symbol_table.get(&symbol).unwrap())
                {
                    monitored.push(MonitorType::Symbol(symbol));
                }
            }
        }
        monitored
    }

    pub fn reset_altered(&mut self) {
        for reg in &mut self.r {
            reg.reset_altered();
        }

        self.memory.reset_accessed();
    }

    //pub fn run(&mut self) {
    //    run(self);
    //}

    pub fn print_verbose(&mut self) {
        // Print altered registers
        let mut log = String::new();
        log.push_str("\nRegisters\n");
        if self.pc.get_altered() {
            log.push_str(&format!(
                "  PC: {} | {:#06x}\n",
                self.pc.get(),
                self.pc.get()
            ));
        }
        for i in 0..16 {
            if self.r[i].get_altered() {
                log.push_str(&format!(
                    "  R{}: {} | {:#06x}\n",
                    i,
                    self.r[i].get(),
                    self.r[i].get()
                ));
            }
        }

        // Print altered memory
        log.push_str("\nMemory\n");
        for i in self.memory.get_used() {
            log.push_str(&format!("  {:#06x} => {:#06x}\n", i, self.memory[*i]));
        }

        log.push_str(&format!("Running state: {}\n", self.state));
        log!(Level::Info, "{log}");
    }
}

pub enum MonitorType {
    Address(usize),
    Symbol(String),
    Register(usize),
}
