use std::collections::HashMap;

use crate::state::State;

pub struct History {
    memory_changes: HashMap<usize, u16>,
    register_changes: HashMap<usize, u16>,
    previous: Option<Box<History>>,
}

impl History {
    pub fn new(previous: Option<History>, state: &State) -> History {
        match previous.take() {
            None => {
                let memory: HashMap<usize, u16> = HashMap::new();
                let register: HashMap<usize, u16> = HashMap::new();

                History {
                    memory_changes: memory,
                    register_changes: register,
                    previous: None,
                }
            }
            Some(prev) => {}
        }
    }
}
