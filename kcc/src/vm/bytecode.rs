use std::sync::Arc;

use colored::Colorize;
use hashbrown::HashMap;
use parking_lot::RwLock;
use scratch_ast::prelude::*;

#[derive(Clone, Debug)]
pub enum VMValuePointer {
    List { name: String, id: usize },
    Variable { name: String, id: usize },
    Broadcast { name: String, id: usize },
}

#[derive(Clone, Debug)]
pub struct VMField {
    pub display_value: String,
    pub pointer: Option<VMValuePointer>,
}

#[derive(Clone, Debug)]
pub enum VMEvaluable {
    Bare(RichValue),
    Pointer(VMValuePointer),
    Block(Expression),
    Field(VMField),
}

#[allow(clippy::too_many_arguments)]
pub fn fetch_dependencies(
    block: &Block,
    local_list_numid_map: &HashMap<String, usize>,
    local_var_numid_map: &HashMap<String, usize>,
    local_broadcast_numid_map: &HashMap<String, usize>,
    global_list_numid_map: &HashMap<String, usize>,
    global_var_numid_map: &HashMap<String, usize>,
    global_broadcast_numid_map: &HashMap<String, usize>,
    block_list: &std::collections::HashMap<String, Block>,
) -> HashMap<String, VMEvaluable> {
    let mut output = HashMap::new();
    for (ik, iv) in &block.inputs {
        output.insert(
            ik.to_owned(),
            VMEvaluable::new(
                iv.value.to_owned(),
                local_list_numid_map,
                local_var_numid_map,
                local_broadcast_numid_map,
                global_list_numid_map,
                global_var_numid_map,
                global_broadcast_numid_map,
                block_list,
            ),
        );
    }
    for (ik, Field { value, value_id }) in &block.fields {
        output.insert(
            ik.to_string(),
            VMEvaluable::Field(VMField {
                display_value: value.to_string(),
                pointer: match ik.as_str() {
                    "VARIABLE" => Some(VMValuePointer::Variable {
                        name: value.to_string(),
                        id: *local_var_numid_map.get(&value_id.clone().expect("Malformed field array, field does not have a varid reference")).unwrap_or(global_var_numid_map.get(&value_id.clone().unwrap()).expect("varid referenced by field not found")),
                    }),
                    "LIST" => Some(VMValuePointer::List {
                        name: value.to_string(),
                        id: *local_list_numid_map.get(&value_id.clone().expect("Malformed field array, field does not have a listid reference")).unwrap_or(global_list_numid_map.get(&value_id.clone().unwrap()).expect("listid referenced by field not found")),
                    }),
                    "BROADCAST_OPTION" => Some(VMValuePointer::Broadcast {
                        name: value.to_string(),
                        id: *local_broadcast_numid_map.get(&value_id.clone().expect("Malformed field array, field does not have a broadcastid reference")).unwrap_or(global_broadcast_numid_map.get(&value_id.clone().unwrap()).expect("broadcastid referenced by field not found")),
                    }),
                    _ => None,
                },
            }),
        );
    }
    output
}

impl VMEvaluable {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        value: ShadowValue,
        local_list_numid_map: &HashMap<String, usize>,
        local_var_numid_map: &HashMap<String, usize>,
        local_broadcast_numid_map: &HashMap<String, usize>,
        global_list_numid_map: &HashMap<String, usize>,
        global_var_numid_map: &HashMap<String, usize>,
        global_broadcast_numid_map: &HashMap<String, usize>,
        block_list: &std::collections::HashMap<String, Block>,
    ) -> Self {
        match value {
            ShadowValue::Bare(b) => Self::Bare(b),
            ShadowValue::Pointer(ValuePointer::List { name, id: str_id }) => {
                Self::Pointer(VMValuePointer::List {
                    name,
                    id: *local_list_numid_map.get(&str_id).unwrap_or(
                        global_list_numid_map
                            .get(&str_id)
                            .expect("listid referenced by pointer not found"),
                    ),
                })
            }
            ShadowValue::Pointer(ValuePointer::Variable { name, id: str_id }) => {
                Self::Pointer(VMValuePointer::Variable {
                    name,
                    id: *local_var_numid_map.get(&str_id).unwrap_or(
                        global_var_numid_map
                            .get(&str_id)
                            .expect("varid referenced by pointer not found"),
                    ),
                })
            }
            ShadowValue::Block(b) => {
                let block = block_list.get(&b.id).unwrap();
                Self::Block(Expression {
                    opcode: block.block_type,
                    dependencies: fetch_dependencies(
                        block,
                        local_list_numid_map,
                        local_var_numid_map,
                        local_broadcast_numid_map,
                        global_list_numid_map,
                        global_var_numid_map,
                        global_broadcast_numid_map,
                        block_list,
                    ),
                    original_block: Box::new({
                        let mut o = block.to_owned();
                        o.obj_id = b.id;
                        o
                    }),
                })
            }
        }
    }
}

#[derive(Clone, Debug)]
pub struct Expression {
    pub opcode: BlockType,
    pub dependencies: HashMap<String, VMEvaluable>,
    pub original_block: Box<Block>,
}

impl std::fmt::Display for Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:>2}{}{}{}{}{}",
            self.original_block.obj_id.cyan(),
            ".".black(),
            format!("{:?}", self.opcode).bright_green(),
            "(".black(),
            {
                let mut output = Vec::new();

                for (name, val) in &self.dependencies {
                    output.push(format!(
                        "{}{}{}",
                        name.to_lowercase().black(),
                        ": ".black(),
                        match val {
                            VMEvaluable::Bare(rv) =>
                                serde_json::to_string(rv).unwrap().bright_cyan().to_string(),
                            VMEvaluable::Block(b) => format!(
                                "{}{}{}",
                                b.original_block.obj_id.cyan(),
                                ".".black(),
                                format!("{:?}", b.opcode).bright_green(),
                            ),
                            VMEvaluable::Pointer(VMValuePointer::List { name, id }) => format!(
                                "{}{}{} {}{}{}",
                                "(".black(),
                                "list".yellow(),
                                ")".black(),
                                id.to_string().cyan(),
                                ".".black(),
                                name.bright_yellow()
                            ),
                            VMEvaluable::Pointer(VMValuePointer::Variable { name, id }) => format!(
                                "{}{}{} {}{}{}",
                                "(".black(),
                                "var".yellow(),
                                ")".black(),
                                id.to_string().cyan(),
                                ".".black(),
                                name.bright_yellow()
                            ),
                            VMEvaluable::Pointer(VMValuePointer::Broadcast { name, id }) =>
                                format!(
                                    "{}{}{} {}{}{}",
                                    "(".black(),
                                    "var".yellow(),
                                    ")".black(),
                                    id.to_string().cyan(),
                                    ".".black(),
                                    name.bright_yellow()
                                ),
                            VMEvaluable::Field(f) => format!(
                                "{}{}{}{}",
                                "[".black(),
                                f.display_value.bright_cyan(),
                                match &f.pointer {
                                    Some(VMValuePointer::List { name, id }) => format!(
                                        "{} {}{}{} {}{}{}",
                                        ",".black(),
                                        "(".black(),
                                        "list".yellow(),
                                        ")".black(),
                                        id.to_string().cyan(),
                                        ".".black(),
                                        name.bright_yellow()
                                    ),
                                    Some(VMValuePointer::Variable { name, id }) => format!(
                                        "{} {}{}{} {}{}{}",
                                        ",".black(),
                                        "(".black(),
                                        "var".yellow(),
                                        ")".black(),
                                        id.to_string().cyan(),
                                        ".".black(),
                                        name.bright_yellow()
                                    ),
                                    Some(VMValuePointer::Broadcast { name, id }) => format!(
                                        "{} {}{}{} {}{}{}",
                                        ",".black(),
                                        "(".black(),
                                        "var".yellow(),
                                        ")".black(),
                                        id.to_string().cyan(),
                                        ".".black(),
                                        name.bright_yellow()
                                    ),
                                    None => "".to_string(),
                                },
                                "]".black()
                            ),
                        }
                    ));
                }

                output.join(&", ".black().to_string())
            },
            ")".black()
        )
    }
}

#[derive(Clone, Debug)]
pub struct VMThread {
    pub stack: Vec<RichValue>,
    pub code: Vec<Expression>,
}

#[derive(Debug)]
pub struct VMGlobalState {
    pub variables: HashMap<usize, RwLock<PrimitiveValue>>,
    pub lists: HashMap<usize, Arc<RwLock<Vec<RwLock<PrimitiveValue>>>>>,
    pub broadcasts: HashMap<usize, String>,
    pub list_numid_map: Arc<HashMap<String, usize>>,
    pub var_numid_map: Arc<HashMap<String, usize>>,
    pub broadcast_numid_map: Arc<HashMap<String, usize>>,
}

#[derive(Debug)]
pub struct VMLocalState {
    pub name: String,
    pub variables: HashMap<usize, RwLock<PrimitiveValue>>,
    pub lists: HashMap<usize, Arc<RwLock<Vec<RwLock<PrimitiveValue>>>>>,
    pub broadcasts: HashMap<usize, String>,
    pub list_numid_map: Arc<HashMap<String, usize>>,
    pub var_numid_map: Arc<HashMap<String, usize>>,
    pub broadcast_numid_map: Arc<HashMap<String, usize>>,
}
