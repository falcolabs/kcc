use std::sync::Arc;

use parking_lot::RwLock;
use scratch_ast::errors::ScratchError;

use crate::vm::transform::VMStartup;

pub mod argaccess;
pub mod intepreter;
pub mod internals;
pub mod terminal;
pub mod transform;

pub type ScratchResult = Result<(), ScratchError>;

pub fn run(startup: VMStartup) {
    let global_state = Arc::new(RwLock::new(startup.gstate));
    std::thread::scope(|scope| {
        for (target, threads) in startup.targets {
            let local_state = Arc::new(RwLock::new(target));
            for thread in threads {
                let cgs = Arc::clone(&global_state);
                let cls = Arc::clone(&local_state);
                scope.spawn(move || intepreter::exec_thread(thread, cgs, cls).unwrap());
            }
        }
    });
}
