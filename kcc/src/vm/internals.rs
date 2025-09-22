use std::sync::Arc;

use hashbrown::HashMap;
use parking_lot::RwLock;
use scratch_ast::prelude::*;

#[derive(Hash, PartialEq, Eq, Clone, Debug)]
pub enum ThreadTrigger {
    GreenFlag,
    Broadcast(String),
    Mutation(usize),
}

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
    Block(StackExpression),
    Field(VMField),
    Default,
}

#[derive(Clone, Debug)]
pub struct StackExpression {
    pub opcode: BlockType,
    pub dependencies: HashMap<String, VMEvaluable>,
    pub original_block: Box<Block>,
}

#[derive(Clone, Debug)]
pub enum Expression {
    Stack(StackExpression),
    Conditional(),
    LoopTimes(),
    LoopCondition(),
    LoopForever(),
    InvokeBroadcast(),
    InvokeCustomBlock {
        target: usize,
        arguments: HashMap<usize, VMEvaluable>,
    },
}

#[derive(Clone, Debug)]
pub struct VMThread {
    pub custom_block_arguments: HashMap<usize, PrimitiveValue>,
    pub code: Vec<Expression>,
}

pub type VMSourceCode = HashMap<ThreadTrigger, VMThread>;

#[derive(Debug)]
pub struct VMGlobalState {
    pub variables: HashMap<usize, RwLock<PrimitiveValue>>,
    pub lists: HashMap<usize, Arc<RwLock<Vec<RwLock<PrimitiveValue>>>>>,
    pub broadcasts: HashMap<usize, String>,
    pub listname_to_numid: Arc<HashMap<String, usize>>,
    pub varname_to_numid: Arc<HashMap<String, usize>>,
    pub broadcastname_to_numid: Arc<HashMap<String, usize>>,
    pub mutationname_to_numid: Arc<HashMap<String, usize>>,
}

#[derive(Debug)]
pub struct VMLocalState {
    pub name: String,
    pub variables: HashMap<usize, RwLock<PrimitiveValue>>,
    pub lists: HashMap<usize, Arc<RwLock<Vec<RwLock<PrimitiveValue>>>>>,
    pub broadcasts: HashMap<usize, String>,
    pub listname_to_numid: Arc<HashMap<String, usize>>,
    pub varname_to_numi: Arc<HashMap<String, usize>>,
    pub broadcastname_to_numid: Arc<HashMap<String, usize>>,
}
