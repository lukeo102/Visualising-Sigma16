use crate::interpreter::interpreter::run;
use crate::interpreter::memory::Memory;
use crate::interpreter::register::Register;
use log::{log, Level};
use std::collections::HashMap;
use std::fmt::Display;

#[derive(serde::Serialize, serde::Deserialize)]
pub enum RunningState {
    Init,
    Ready,
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
            RunningState::Init => "INIT".to_string(),
            RunningState::Running => "RUNNING".to_string(),
            RunningState::Paused => "PAUSED".to_string(),
            RunningState::Breakpoint => "BREAKPOINT".to_string(),
            RunningState::Haulted => "HALTED".to_string(),
            RunningState::Interrupted => "INTERRUPTED".to_string(),
            RunningState::Step => "STEP".to_string(),
            RunningState::Ready => "READY".to_string(),
        };
        write!(f, "{str}")
    }
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct State {
    pub pc: Register,
    pub ir: Register,
    pub r: [Register; 16],
    pub memory: Memory,
    pub state: RunningState,
    pub verbose: bool,
    symbol_table: HashMap<String, u16>,
    monitored_symbols: Vec<String>,
    monitored_addresses: Vec<u16>,
    monitored_registers: Vec<usize>,
}

impl State {
    pub fn new(memory: &[u16]) -> State {
        let mut state = State {
            pc: (Register::new()),
            ir: (Register::new()),
            r: [Register::new(); 16],
            memory: (Memory::new(Option::from(memory))),
            state: RunningState::Init,
            verbose: false,
            symbol_table: HashMap::new(),
            monitored_symbols: Vec::new(),
            monitored_addresses: Vec::new(),
            monitored_registers: Vec::new(),
        };
        state.r[0].set_r0();
        state
    }

    pub fn run(&mut self) {
        run(self);
    }

    pub fn monitor_check(&mut self) {}

    pub fn monitor_enable(&mut self, element: MonitorType) -> Option<MonitorType> {
        match element {
            MonitorType::Address(item) => {
                if self.monitored_addresses.contains(&item) {
                    None
                } else {
                    self.monitored_addresses.push(item.clone());
                    Some(element)
                }
            }
            MonitorType::Symbol(item) => {
                if self.monitored_symbols.contains(&item) {
                    None
                } else {
                    self.monitored_symbols.push(item.clone());
                    Some(MonitorType::Symbol(item))
                }
            }
            MonitorType::Register(item) => {
                if self.monitored_registers.contains(&item) {
                    None
                } else {
                    self.monitored_registers.push(item.clone());
                    Some(element)
                }
            }
        }
    }

    pub fn monitor_disable(&mut self, element: MonitorType) -> Option<MonitorType> {
        match element {
            MonitorType::Address(item) => {
                let idx = self.monitored_addresses.iter().position(|x| *x == item);
                match idx {
                    Some(i) => {
                        self.monitored_addresses.remove(i);
                        Some(element)
                    }
                    None => None,
                }
            }
            MonitorType::Symbol(item) => {
                let idx = self.monitored_symbols.iter().position(|x| *x == item);
                match idx {
                    Some(i) => {
                        self.monitored_symbols.remove(i);
                        Some(MonitorType::Symbol(item))
                    }
                    None => None,
                }
            }
            MonitorType::Register(item) => {
                let idx = self.monitored_registers.iter().position(|x| *x == item);
                match idx {
                    Some(i) => {
                        self.monitored_registers.remove(i);
                        Some(element)
                    }
                    None => None,
                }
            }
        }
    }

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
        if self.ir.get_altered() {
            log.push_str(&format!(
                "  IR: {} | {:#06x}\n",
                self.ir.get(),
                self.ir.get()
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
    Address(u16),
    Symbol(String),
    Register(usize),
}
