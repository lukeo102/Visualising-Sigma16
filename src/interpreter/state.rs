use crate::interpreter::memory::Memory;
use crate::interpreter::register::Register;
use std::fmt::Display;

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

pub struct State {
    pub pc: Register,
    pub ir: Register,
    pub r: [Register; 16],
    pub memory: Memory,
    pub state: RunningState,
    pub verbose: bool,
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
        };
        state.r[0].set_r0();
        state
    }

    pub fn print_verbose(&mut self) {
        // Print altered registers

        println!("Registers");
        if self.pc.get_altered() {
            println!("  PC: {} | {:#06x}", self.pc.get(), self.pc.get());
        }
        if self.ir.get_altered() {
            println!("  IR: {} | {:#06x}", self.ir.get(), self.ir.get());
        }
        for i in 0..16 {
            if self.r[i].get_altered() {
                println!("  R{}: {} | {:#06x}", i, self.r[i].get(), self.r[i].get());
            }
        }

        // Print altered memory
        println!("\nMemory");
        for i in self.memory.get_used() {
            println!("  {:#06x} => {:#06x}", i, self.memory[*i]);
        }

        println!("Running state: {}", self.state);
        println!("\n\n");
    }
}
