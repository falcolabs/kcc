use std::sync::{atomic::AtomicUsize, Arc};

use crate::vm::bytecode::*;
use hashbrown::HashMap;
use parking_lot::RwLock;
use rayon::iter::{IntoParallelRefIterator, ParallelBridge, ParallelIterator};
use scratch_ast::model::{self, Block, BlockType, PrimitiveValue, Target};

static ID_COUNTER: AtomicUsize = AtomicUsize::new(0);
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
) -> Vec<VMThread> {
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
) -> VMThread {
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
        code.push(Expression {
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
        });
        next_id = &current_block.next_id;
    }

    VMThread {
        stack: Vec::new(),
        code,
    }
}

pub struct VMStartup {
    pub gstate: VMGlobalState,
    pub targets: Vec<(VMLocalState, Vec<VMThread>)>,
}

impl From<model::Project> for VMStartup {
    fn from(value: model::Project) -> VMStartup {
        let mut global_listid_to_value = hashbrown::HashMap::new();
        let mut global_varid_to_value = hashbrown::HashMap::new();
        let mut global_broadcastid_to_value = hashbrown::HashMap::new();
        let mut global_varid_to_numid: Arc<HashMap<String, usize>> = Arc::new(HashMap::new());
        let mut global_listid_to_numid: Arc<HashMap<String, usize>> = Arc::new(HashMap::new());
        let mut global_broadcastid_to_numid: Arc<HashMap<String, usize>> = Arc::new(HashMap::new());
        let mut target_tuple: Vec<(VMLocalState, Vec<VMThread>)> = Vec::new();

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
                            list_numid_map: Arc::clone(&listid_to_numid),
                            var_numid_map: Arc::clone(&varid_to_numid),
                            broadcast_numid_map: Arc::clone(&broadcastid_to_numid),
                        },
                        extract_threads(
                            s.blocks.clone(),
                            Arc::clone(&varid_to_numid),
                            Arc::clone(&listid_to_numid),
                            Arc::clone(&broadcastid_to_numid),
                            Arc::clone(&global_varid_to_numid),
                            Arc::clone(&global_listid_to_numid),
                            Arc::clone(&global_broadcastid_to_numid),
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
                            list_numid_map: Arc::clone(&global_varid_to_numid),
                            var_numid_map: Arc::clone(&global_listid_to_numid),
                            broadcast_numid_map: Arc::clone(&global_broadcastid_to_numid),
                        },
                        extract_threads(
                            s.blocks.clone(),
                            Arc::clone(&global_varid_to_numid),
                            Arc::clone(&global_listid_to_numid),
                            Arc::clone(&global_broadcastid_to_numid),
                            Arc::clone(&global_varid_to_numid),
                            Arc::clone(&global_listid_to_numid),
                            Arc::clone(&global_broadcastid_to_numid),
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
                list_numid_map: Arc::clone(&global_listid_to_numid),
                var_numid_map: Arc::clone(&global_varid_to_numid),
                broadcast_numid_map: Arc::clone(&global_broadcastid_to_numid),
            },
            targets: target_tuple,
        }
    }
}
