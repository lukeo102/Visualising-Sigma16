use std::collections::HashMap;

use crate::state::{RunningState, State};

#[derive(serde::Serialize, serde::Deserialize, Clone)]
pub struct History {
    memory_changes: HashMap<usize, u16>,
    register_changes: HashMap<usize, u16>,
    previous: Option<Box<History>>,
    state: RunningState,
    pc: u16,
}

impl History {
    pub fn new(previous: &mut Option<History>, state: &State) -> History {
        match previous.take() {
            None => {
                let memory: HashMap<usize, u16> = HashMap::new();
                let register: HashMap<usize, u16> = HashMap::new();

                History {
                    memory_changes: memory,
                    register_changes: register,
                    previous: None,
                    state: state.state.clone(),
                    pc: state.pc.clone().get(),
                }
            }
            Some(prev) => History {
                state: state.state.clone(),
                pc: state.pc.clone().get(),
                previous: Some(Box::new(prev)),
                memory_changes: History::make_memory_map(state),
                register_changes: History::make_register_map(state),
            },
        }
    }

    fn make_memory_map(current: &State) -> HashMap<usize, u16> {
        let altered_memory = current.memory.get_altered_i();
        let mut memory_map: HashMap<usize, u16> = HashMap::new();

        for i in altered_memory {
            memory_map.insert(*i, current.memory[*i].clone());
        }

        memory_map
    }

    fn make_register_map(current: &State) -> HashMap<usize, u16> {
        let mut register_map: HashMap<usize, u16> = HashMap::new();
        let mut current_registers = current.r.clone();

        for i in 1..16 {
            if current_registers[i].get_altered() {
                register_map.insert(i, current_registers[i].get());
            }
        }
        register_map
    }

    pub fn apply(history: History, state: &mut State) -> History {
        state.state = history.state.clone();
        state.pc.set(history.pc);

        for (i, updated) in history.register_changes.clone() {
            state.r[i].set(updated);
        }

        for (i, updated) in history.memory_changes.clone() {
            state.memory[i] = updated;
        }

        match history.previous {
            None => history,
            Some(previous) => *previous,
        }
    }
}
