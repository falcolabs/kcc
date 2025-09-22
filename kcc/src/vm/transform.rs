use std::sync::{atomic::AtomicUsize, Arc};

use crate::vm::{argaccess::fetch_dependencies, internals::*};
use hashbrown::HashMap;
use parking_lot::RwLock;
use rayon::iter::{IntoParallelRefIterator, ParallelBridge, ParallelIterator};
use scratch_ast::model::{
    self, Block, BlockType, Mutation, PrimitiveValue, ProcedureCall, ProcedurePrototype, Target,
};

static ID_COUNTER: AtomicUsize = AtomicUsize::new(0);
static PROCCODE_COUNTER: AtomicUsize = AtomicUsize::new(0);
const HAT_BLOCKS: [BlockType; 9] = [
    BlockType::EventWhenFlagClicked,
    BlockType::EventWhenKeyPressed,
    BlockType::EventWhenThisSpriteClicked,
    BlockType::EventWhenStageClicked,
    BlockType::EventWhenBroadcastReceived,
    BlockType::EventWhenBackdropSwitchesTo,
    BlockType::EventWhenGreaterThan,
    BlockType::ControlStartAsClone,
    BlockType::ProceduresDefinition,
];

fn extract_threads(
    block_list: std::collections::HashMap<String, Block>,
    local_varid_to_numid: Arc<HashMap<String, usize>>,
    local_listid_to_numid: Arc<HashMap<String, usize>>,
    local_broadcastid_to_numid: Arc<HashMap<String, usize>>,
    global_varid_to_numid: Arc<HashMap<String, usize>>,
    global_listid_to_numid: Arc<HashMap<String, usize>>,
    global_broadcastid_to_numid: Arc<HashMap<String, usize>>,
    global_mutation_proccode_to_numid: Arc<RwLock<HashMap<String, usize>>>,
    global_mutation_argname_to_numid: Arc<RwLock<HashMap<String, usize>>>,
    global_mutation_argid_to_numid: Arc<RwLock<HashMap<String, usize>>>,
) -> VMSourceCode {
    let bl = Arc::new(block_list);
    let hats: Vec<&String> = bl
        .par_iter()
        .filter_map(|(i, b)| {
            if HAT_BLOCKS.contains(&b.block_type) {
                return Some(i);
            }
            None
        })
        .collect();

    hats.par_iter()
        .map(|h| {
            extract_thread(
                h.to_string(),
                Arc::clone(&bl),
                Arc::clone(&local_varid_to_numid),
                Arc::clone(&local_listid_to_numid),
                Arc::clone(&local_broadcastid_to_numid),
                Arc::clone(&global_varid_to_numid),
                Arc::clone(&global_listid_to_numid),
                Arc::clone(&global_broadcastid_to_numid),
                Arc::clone(&global_mutation_proccode_to_numid),
                Arc::clone(&global_mutation_argname_to_numid),
                Arc::clone(&global_mutation_argid_to_numid),
            )
        })
        .collect()
}

#[allow(clippy::too_many_arguments)]
fn extract_thread(
    hat_block_id: String,
    block_list: Arc<std::collections::HashMap<String, Block>>,
    local_varid_to_numid: Arc<HashMap<String, usize>>,
    local_listid_to_numid: Arc<HashMap<String, usize>>,
    local_broadcastid_to_numid: Arc<HashMap<String, usize>>,
    global_varid_to_numid: Arc<HashMap<String, usize>>,
    global_listid_to_numid: Arc<HashMap<String, usize>>,
    global_broadcastid_to_numid: Arc<HashMap<String, usize>>,
    global_mutation_proccode_to_numid: Arc<RwLock<HashMap<String, usize>>>,
    global_mutation_argname_to_numid: Arc<RwLock<HashMap<String, usize>>>,
    global_mutation_argid_to_numid: Arc<RwLock<HashMap<String, usize>>>,
) -> (ThreadTrigger, VMThread) {
    let mut code = Vec::new();
    let mut next_id: &Option<String> = &Some(hat_block_id);
    let mut current_block: &Block;
    while let Some(id) = next_id {
        current_block = block_list.get(id).unwrap_or_else(|| {
            panic!(
                "malformed project, references a block that does not exist: {}",
                id
            )
        });
        code.push(Expression::Stack(StackExpression {
            opcode: current_block.block_type,
            dependencies: fetch_dependencies(
                current_block,
                &local_listid_to_numid,
                &local_varid_to_numid,
                &local_broadcastid_to_numid,
                &global_listid_to_numid,
                &global_varid_to_numid,
                &global_broadcastid_to_numid,
                &block_list,
            ),
            original_block: Box::new({
                let mut o = current_block.clone();
                o.obj_id = id.to_string();
                o
            }),
        }));
        next_id = &current_block.next_id;
    }

    let mut custom_block_arguments = HashMap::new();
    let mut trigger: ThreadTrigger = ThreadTrigger::GreenFlag;
    if let Some(Expression::Stack(StackExpression {
        opcode,
        dependencies,
        ..
    })) = code.get(0)
    {
        if opcode == &BlockType::ProceduresDefinition {
            if let VMEvaluable::Block(prototype) = dependencies.get("custom_block").expect("Malformed custom block definition, definition hat block did not point to its prototype") {
                if let Mutation::ProcedurePrototype(ProcedurePrototype { proccode, arguments_ids, argument_names, argument_defaults, ..}) =  prototype.original_block.mutation.as_ref().unwrap() {
                    trigger = ThreadTrigger::Mutation(
                            *global_mutation_proccode_to_numid
                                .write()
                                .entry(proccode.to_string())
                                .or_insert_with(|| {
                                    PROCCODE_COUNTER
                                        .fetch_add(1, std::sync::atomic::Ordering::Relaxed)
                                }),
                        );
                    for ((name, strid), default_value) in std::iter::zip(std::iter::zip(argument_names, arguments_ids), argument_defaults) {
                        let aid = if let Some(a) = global_mutation_argid_to_numid.read().get(strid) {
                            *a
                        } else {
                            ID_COUNTER.fetch_add(1, std::sync::atomic::Ordering::Relaxed)
                        };
                        global_mutation_argid_to_numid.write().insert(strid.to_owned(), aid);
                        global_mutation_argname_to_numid.write().insert(name.to_string(), aid);
                        custom_block_arguments.insert(aid, default_value.to_owned());
                    }
                }
            }
        }
        for exp in code.iter_mut() {
            if let Expression::Stack(StackExpression {
                opcode,
                dependencies,
                original_block,
            }) = exp
            {
                if opcode == &BlockType::ProceduresCall {
                    if let Mutation::ProcedureCall(ProcedureCall { proccode, .. }) =
                        original_block.mutation.as_ref().unwrap()
                    {
                        global_mutation_proccode_to_numid
                            .write()
                            .entry(proccode.to_string())
                            .or_insert_with(|| {
                                PROCCODE_COUNTER.fetch_add(1, std::sync::atomic::Ordering::Relaxed)
                            });

                        *exp = Expression::InvokeCustomBlock {
                            target: match global_mutation_proccode_to_numid.read().get(proccode) {
                                Some(s) => *s,
                                None => {
                                    let numproc = PROCCODE_COUNTER
                                        .fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                                    global_mutation_proccode_to_numid
                                        .write()
                                        .insert(proccode.to_string(), numproc)
                                        .expect("making new custom block id failed");
                                    numproc
                                }
                            },
                            arguments: dependencies
                                .iter()
                                .map(|(strid, val)| {
                                    let aid = *global_mutation_argid_to_numid
                                        .write()
                                        .entry(strid.to_owned())
                                        .or_insert_with(|| {
                                            ID_COUNTER
                                                .fetch_add(1, std::sync::atomic::Ordering::Relaxed)
                                        });
                                    (aid, val.to_owned())
                                })
                                .collect(),
                        };
                    }
                }
            }
        }
    }
    (
        trigger,
        VMThread {
            code,
            custom_block_arguments,
        },
    )
}

pub struct VMStartup {
    pub gstate: VMGlobalState,
    pub targets: Vec<(VMLocalState, VMSourceCode)>,
}

impl From<model::Project> for VMStartup {
    fn from(value: model::Project) -> VMStartup {
        let mut global_listid_to_value = hashbrown::HashMap::new();
        let mut global_varid_to_value = hashbrown::HashMap::new();
        let mut global_broadcastid_to_value = hashbrown::HashMap::new();
        let mut global_varid_to_numid: Arc<HashMap<String, usize>> = Arc::new(HashMap::new());
        let mut global_listid_to_numid: Arc<HashMap<String, usize>> = Arc::new(HashMap::new());
        let mut global_broadcastid_to_numid: Arc<HashMap<String, usize>> = Arc::new(HashMap::new());

        let global_mutation_proccode_to_numid: Arc<RwLock<HashMap<String, usize>>> =
            Arc::new(RwLock::new(HashMap::new()));
        let global_mutation_argname_to_numid: Arc<RwLock<HashMap<String, usize>>> =
            Arc::new(RwLock::new(HashMap::new()));
        let global_mutation_argid_to_numid: Arc<RwLock<HashMap<String, usize>>> =
            Arc::new(RwLock::new(HashMap::new()));
        let mut target_tuple: Vec<(VMLocalState, VMSourceCode)> = Vec::new();

        for t in value.targets {
            match t {
                Target::Sprite(s) => {
                    let varid_to_numid: Arc<HashMap<String, usize>> = Arc::new(
                        s.variables
                            .keys()
                            .par_bridge()
                            .map(|k| {
                                (
                                    k.to_owned(),
                                    ID_COUNTER.fetch_add(1, std::sync::atomic::Ordering::Relaxed),
                                )
                            })
                            .collect(),
                    );
                    let numid_to_varvalue: HashMap<usize, RwLock<PrimitiveValue>> = varid_to_numid
                        .par_iter()
                        .map(|(k, v)| {
                            (
                                v.to_owned(),
                                RwLock::new(
                                    s.variables
                                        .get(k)
                                        .unwrap_or_else(|| panic!("variable {} does not exist", k))
                                        .value
                                        .clone(),
                                ),
                            )
                        })
                        .collect();

                    let listid_to_numid: Arc<HashMap<String, usize>> = Arc::new(
                        s.lists
                            .keys()
                            .par_bridge()
                            .map(|k| {
                                (
                                    k.to_owned(),
                                    ID_COUNTER.fetch_add(1, std::sync::atomic::Ordering::Relaxed),
                                )
                            })
                            .collect(),
                    );
                    let numid_to_listvalue: HashMap<
                        usize,
                        Arc<RwLock<Vec<RwLock<PrimitiveValue>>>>,
                    > = {
                        let mut output = HashMap::new();

                        listid_to_numid.iter().for_each(|(k, v)| {
                            output.insert(
                                *v,
                                Arc::new(RwLock::new(
                                    s.lists
                                        .get(k)
                                        .unwrap_or_else(|| panic!("variable {} does not exist", k))
                                        .value
                                        .par_iter()
                                        .map(|v| RwLock::new(v.to_owned()))
                                        .collect(),
                                )),
                            );
                        });

                        output
                    };

                    let broadcastid_to_numid: Arc<HashMap<String, usize>> = Arc::new(
                        s.broadcasts
                            .keys()
                            .par_bridge()
                            .map(|id| {
                                (
                                    id.to_string(),
                                    ID_COUNTER.fetch_add(1, std::sync::atomic::Ordering::Relaxed),
                                )
                            })
                            .collect(),
                    );
                    let broadcastid_to_value: HashMap<usize, String> = broadcastid_to_numid
                        .par_iter()
                        .map(|(strid, numid)| {
                            (*numid, s.broadcasts.get(strid).unwrap().to_string())
                        })
                        .collect();
                    target_tuple.push((
                        VMLocalState {
                            name: s.name.clone(),
                            variables: numid_to_varvalue,
                            lists: numid_to_listvalue,
                            broadcasts: broadcastid_to_value,
                            listname_to_numid: Arc::clone(&listid_to_numid),
                            varname_to_numi: Arc::clone(&varid_to_numid),
                            broadcastname_to_numid: Arc::clone(&broadcastid_to_numid),
                        },
                        extract_threads(
                            s.blocks.clone(),
                            Arc::clone(&varid_to_numid),
                            Arc::clone(&listid_to_numid),
                            Arc::clone(&broadcastid_to_numid),
                            Arc::clone(&global_varid_to_numid),
                            Arc::clone(&global_listid_to_numid),
                            Arc::clone(&global_broadcastid_to_numid),
                            Arc::clone(&global_mutation_proccode_to_numid),
                            Arc::clone(&global_mutation_argname_to_numid),
                            Arc::clone(&global_mutation_argid_to_numid),
                        ),
                    ))
                }
                Target::Stage(s) => {
                    global_varid_to_numid = Arc::new(
                        s.variables
                            .keys()
                            .par_bridge()
                            .map(|k| {
                                (
                                    k.to_owned(),
                                    ID_COUNTER.fetch_add(1, std::sync::atomic::Ordering::Relaxed),
                                )
                            })
                            .collect(),
                    );
                    global_varid_to_value = global_varid_to_numid
                        .par_iter()
                        .map(|(k, v)| {
                            (
                                v.to_owned(),
                                RwLock::new(
                                    s.variables
                                        .get(k)
                                        .unwrap_or_else(|| panic!("variable {} does not exist", k))
                                        .value
                                        .clone(),
                                ),
                            )
                        })
                        .collect();

                    global_listid_to_numid = Arc::new(
                        s.lists
                            .keys()
                            .par_bridge()
                            .map(|k| {
                                (
                                    k.to_owned(),
                                    ID_COUNTER.fetch_add(1, std::sync::atomic::Ordering::Relaxed),
                                )
                            })
                            .collect(),
                    );
                    global_listid_to_value = {
                        let mut output = HashMap::new();

                        global_listid_to_numid.iter().for_each(|(k, v)| {
                            output.insert(
                                *v,
                                Arc::new(RwLock::new(
                                    s.lists
                                        .get(k)
                                        .unwrap_or_else(|| panic!("variable {} does not exist", k))
                                        .value
                                        .par_iter()
                                        .map(|v| RwLock::new(v.to_owned()))
                                        .collect(),
                                )),
                            );
                        });

                        output
                    };

                    global_broadcastid_to_numid = Arc::new(
                        s.broadcasts
                            .keys()
                            .par_bridge()
                            .map(|id| {
                                (
                                    id.to_string(),
                                    ID_COUNTER.fetch_add(1, std::sync::atomic::Ordering::Relaxed),
                                )
                            })
                            .collect(),
                    );
                    global_broadcastid_to_value = global_broadcastid_to_numid
                        .par_iter()
                        .map(|(strid, numid)| {
                            (*numid, s.broadcasts.get(strid).unwrap().to_string())
                        })
                        .collect();
                    target_tuple.push((
                        VMLocalState {
                            name: s.name.clone(),
                            variables: HashMap::new(),
                            lists: HashMap::new(),
                            broadcasts: HashMap::new(),
                            listname_to_numid: Arc::clone(&global_varid_to_numid),
                            varname_to_numi: Arc::clone(&global_listid_to_numid),
                            broadcastname_to_numid: Arc::clone(&global_broadcastid_to_numid),
                        },
                        extract_threads(
                            s.blocks.clone(),
                            Arc::clone(&global_varid_to_numid),
                            Arc::clone(&global_listid_to_numid),
                            Arc::clone(&global_broadcastid_to_numid),
                            Arc::clone(&global_varid_to_numid),
                            Arc::clone(&global_listid_to_numid),
                            Arc::clone(&global_broadcastid_to_numid),
                            Arc::clone(&global_mutation_proccode_to_numid),
                            Arc::clone(&global_mutation_argname_to_numid),
                            Arc::clone(&global_mutation_argid_to_numid),
                        ),
                    ))
                }
            };
        }

        VMStartup {
            gstate: VMGlobalState {
                lists: global_listid_to_value,
                variables: global_varid_to_value,
                broadcasts: global_broadcastid_to_value,
                listname_to_numid: Arc::clone(&global_listid_to_numid),
                varname_to_numid: Arc::clone(&global_varid_to_numid),
                broadcastname_to_numid: Arc::clone(&global_broadcastid_to_numid),
                mutationname_to_numid: Arc::new({
                    let x = global_mutation_argname_to_numid.read().clone();
                    x
                }),
            },
            targets: target_tuple,
        }
    }
}
