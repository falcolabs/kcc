use std::sync::Arc;

use parking_lot::RwLock;
use scratch_ast::model::{Evaluable, RichValue};

use crate::vm::{errors::VMError, transform::VMStartup};

pub mod argaccess;
pub mod bytecode;
pub mod errors;
pub mod intepreter;
pub mod transform;

pub type ScratchResult = Result<(), VMError>;

impl crate::vm::bytecode::VMGlobalState {
    pub fn run(startup: VMStartup) {
        let global_state = Arc::new(RwLock::new(startup.gstate));
        std::thread::scope(|scope| {
            for (target, threads) in startup.targets {
                let local_state = Arc::new(RwLock::new(target));
                for thread in threads {
                    let cgs = Arc::clone(&global_state);
                    let cls = Arc::clone(&local_state);
                    scope.spawn(move || if let Err(e) = intepreter::exec_thread(thread, cgs, cls) {
                        println!("Runtime error occured. Traceback:");
                        for t in e.trace {
                            println!("at {}", t.location);
                            println!("    {:?}: {}", t.error_type, t.description);
                        }
                        panic!("See the above traceback for details.")
                    });
                }
            }
        });
    }

    pub fn evaluate_input(&self, input: &Evaluable) -> RichValue {
        match input {
            Evaluable::Bare(lt) => lt.clone(),
            _ => todo!(),
            //     elif isinstance(self.value, ShadowBlock):
            //         return await self.value.evaluate(interpreter)
            //     else:
            //         return await interpreter.execute(self.value)
        }
    }
}
