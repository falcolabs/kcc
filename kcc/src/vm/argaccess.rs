use std::sync::Arc;

use parking_lot::RwLock;
use scratch_ast::model::{PrimitiveValue, RichValue};

use crate::vm::{
    bytecode::{Expression, VMEvaluable, VMValuePointer},
    intepreter::{eval_exp, VMState},
    ScratchError,
};

#[inline]
pub fn rich_value_to_string(rval: &RichValue) -> Result<String, ScratchError> {
    Ok(Into::<PrimitiveValue>::into(rval.clone()).into())
}

#[inline]
pub fn rich_value_to_f64(rval: &RichValue) -> Result<f64, ScratchError> {
    Into::<PrimitiveValue>::into(rval.clone())
        .try_into()
        .map_err(|e| ScratchError::type_error(e, format!("casting rich value {rval:?} to float")))
}

#[inline]
pub fn rich_value_to_bool(rval: &RichValue) -> Result<bool, ScratchError> {
    Into::<PrimitiveValue>::into(rval.clone())
        .try_into()
        .map_err(|e| ScratchError::type_error(e, format!("casting rich value {rval:?} to bool")))
}

impl Expression {
    #[inline]
    pub fn argraw(&self, argname: &str) -> Option<&VMEvaluable> {
        self.dependencies.get(argname)
    }

    pub fn argstr(&self, argname: &str, state: &VMState) -> Result<String, ScratchError> {
        match self.argraw(argname).ok_or(ScratchError::not_found(
            format!("argument '{argname}' not found"),
            format!("lookup '{argname}'"),
        ))? {
            VMEvaluable::Bare(rv) => rich_value_to_string(rv),
            VMEvaluable::Field(f) => Ok(f.display_value.clone()),
            VMEvaluable::Pointer(v) => match &v {
                VMValuePointer::Variable { id, name } => {
                    let pv: PrimitiveValue;
                    if let Some(var) = state.local_state.read().variables.get(id) {
                        pv = var.read().clone();
                    } else {
                        pv = state
                            .global_state
                            .read()
                            .variables
                            .get(id)
                            .ok_or(ScratchError::not_found(
                                format!("variable {name} not found"),
                                format!("fetching '{argname}' (id={id})"),
                            ))?
                            .read()
                            .clone()
                    }
                    Ok(pv.into())
                }
                VMValuePointer::List { id, .. } => {
                    let pv: String;
                    if let Some(var) = state.local_state.read().lists.get(id) {
                        pv = var
                            .read()
                            .iter()
                            .map(|e| e.read().clone().into())
                            .collect::<Vec<String>>()
                            .join(" ");
                    } else {
                        pv = state
                            .global_state
                            .read()
                            .lists
                            .get(id)
                            .ok_or(ScratchError::not_found(
                                format!("list {argname} not found"),
                                format!("fetching '{argname}' (id={id})"),
                            ))?
                            .read()
                            .iter()
                            .map(|e| e.read().clone().into())
                            .collect::<Vec<String>>()
                            .join(" ")
                    }
                    Ok(pv)
                }
                VMValuePointer::Broadcast { name, .. } => Ok(name.to_string()),
            },
            VMEvaluable::Block(b) => rich_value_to_string(&eval_exp(b, state)?),
        }
    }

    pub fn argfloat(&self, argname: &str, state: &VMState) -> Result<f64, ScratchError> {
        match self.argraw(argname).ok_or(ScratchError::not_found(format!("argument '{argname}' not found"), format!("lookup '{argname}'")))? {
            VMEvaluable::Bare(rv) => rich_value_to_f64(rv),
            VMEvaluable::Field(f) => f
                .display_value
                .parse::<f64>()
                .map_err(|e| ScratchError::type_error(format!("cannot convert argument to float: {e}"), format!("fetching field '{argname}'"))),
            VMEvaluable::Pointer(v) => match &v {
                VMValuePointer::Variable { id, name } => {
                    let pv: PrimitiveValue;
                    if let Some(var) = state.local_state.read().variables.get(id) {
                        pv = var.read().clone();
                    } else {
                        pv = state.global_state
                            .read()
                            .variables
                            .get(id)
                            .ok_or(ScratchError::not_found(format!("variable '{name}' not found"), format!("fetching '{argname}' (id={id}) as string")))?
                            .read()
                            .clone()
                    }
                    pv.try_into().map_err(|e| ScratchError::type_error(e, format!("fetching argument '{argname}' (id={id})")))
                }
                VMValuePointer::List { id, .. } => Err(ScratchError::type_error(
                    format!("lists cannot be converted into float. Argument '{argname}' points to a list, you may have accidentally dragged a list reporter inside a block slot that only accept strings."),
                    format!("fetching argument '{argname}' (id={id}) as string")
                )),
                VMValuePointer::Broadcast { .. } => Err(ScratchError::type_error(
                    format!("broadcasts cannot be converted into float. Argument '{argname}' points to a broadcast, you may have accidentally dragged a broadcast inside a block slot that only accept strings."),
                    format!("fetching argument '{argname}' as string")
                )),
            },
            VMEvaluable::Block(b) => rich_value_to_f64(&eval_exp(b, state)?),
        }
    }

    pub fn argbool(&self, argname: &str, state: &VMState) -> Result<bool, ScratchError> {
        match self.argraw(argname).ok_or(ScratchError::not_found(format!("argument '{argname}' not found"), format!("lookup '{argname}'")))? {
            VMEvaluable::Bare(rv) => rich_value_to_bool(rv),
            VMEvaluable::Field(f) => if f
                .display_value == "true" {
                    Ok(true)
                } else if f.display_value == "false" {
                    Ok(false)
                } else {
                    Err(ScratchError::type_error(format!("cannot convert argument to float {argname}='{:#?}'", f.display_value), format!("fetching field '{argname}'")))
                }
            VMEvaluable::Pointer(v) => match &v {
                VMValuePointer::Variable { id, name } => {
                    let pv: PrimitiveValue;
                    if let Some(var) = state.local_state.read().variables.get(id) {
                        pv = var.read().clone();
                    } else {
                        pv = state.global_state
                            .read()
                            .variables
                            .get(id)
                            .ok_or(ScratchError::not_found(format!("variable '{name}' not found"), format!("fetching '{argname}' (id={id}) as bool")))?
                            .read()
                            .clone()
                    }
                    pv.try_into().map_err(|e| ScratchError::type_error(e, format!("fetching argument '{argname}' (id={id})")))
                }
                VMValuePointer::List { id, .. } => Err(ScratchError::type_error(
                    format!("lists cannot be converted into float. Argument '{argname}' points to a list, you may have accidentally dragged a list reporter inside a block slot that only accept strings."),
                    format!("fetching argument '{argname}' (id={id}) as string")
                )),
                VMValuePointer::Broadcast { .. } => Err(ScratchError::type_error(
                    format!("broadcasts cannot be converted into float. Argument '{argname}' points to a broadcast, you may have accidentally dragged a broadcast inside a block slot that only accept strings."),
                    format!("fetching argument '{argname}' as string")
                )),
            },
            VMEvaluable::Block(b) => rich_value_to_bool(&eval_exp(b, state)?),
        }
    }

    pub fn argptr(&self, argname: &str) -> Result<VMValuePointer, ScratchError> {
        match self.argraw(argname).ok_or(ScratchError::not_found(
            format!("argument '{argname}' not found"),
            format!("lookup '{argname}'"),
        ))? {
            VMEvaluable::Bare(rv) => Err(ScratchError::type_error(
                format!("cannot convert bare value {argname}={rv:#?} to a pointer"),
                format!("fetfching '{argname}'={rv:#?} as pointer"),
            )),
            VMEvaluable::Field(f) => {
                if let Some(p) = &f.pointer {
                    Ok(p.clone())
                } else {
                    Err(ScratchError::type_error(
                        format!(
                            "field {argname} contains the literal {:#?}, not a pointer",
                            f.display_value
                        ),
                        format!("fetfching '{argname}'={:#?} as pointer", f.display_value),
                    ))
                }
            }
            VMEvaluable::Pointer(v) => Ok(v.clone()),
            VMEvaluable::Block(b) => Err(ScratchError::type_error(
                format!("cannot convert or evaluate block {argname}={b:#?} to a pointer"),
                format!("fetfching '{argname}'={b} as pointer"),
            )),
        }
    }

    /// argfloat with nice error.
    /// utility function.
    pub fn sargfloat(
        &self,
        argname: &str,
        state: &VMState,
        exp: &Expression,
    ) -> Result<f64, ScratchError> {
        self.argfloat(argname, state).map_err(|e| {
            e.push_not_found(
                format!("required argument {argname} not found"),
                format!("block {:?} (id={})", exp.opcode, exp.original_block.obj_id),
            )
        })
    }

    /// argraw with nice error.
    /// utility function.
    pub fn sargraw(&self, argname: &str, exp: &Expression) -> Result<&VMEvaluable, ScratchError> {
        self.argraw(argname)
            .ok_or(ScratchError::not_found(
                format!("argument '{argname}' not found"),
                format!("lookup '{argname}'"),
            ))
            .map_err(|e| {
                e.push_not_found(
                    format!("required argument {argname} not found"),
                    format!("block {:?} (id={})", exp.opcode, exp.original_block.obj_id),
                )
            })
    }

    /// argstr with nice error.
    /// utility function.
    pub fn sargstr(
        &self,
        argname: &str,
        state: &VMState,
        exp: &Expression,
    ) -> Result<String, ScratchError> {
        self.argstr(argname, state).map_err(|e| {
            e.push_not_found(
                format!("required argument {argname} not found"),
                format!("block {:?} (id={})", exp.opcode, exp.original_block.obj_id),
            )
        })
    }

    /// argbool with nice error.
    /// utility function.
    pub fn sargbool(
        &self,
        argname: &str,
        state: &VMState,
        exp: &Expression,
    ) -> Result<bool, ScratchError> {
        self.argbool(argname, state).map_err(|e| {
            e.push_not_found(
                format!("required argument {argname} not found"),
                format!("block {:?} (id={})", exp.opcode, exp.original_block.obj_id),
            )
        })
    }

    pub fn sargptr(&self, argname: &str, exp: &Expression) -> Result<VMValuePointer, ScratchError> {
        self.argptr(argname).map_err(|e| {
            e.push_not_found(
                format!("required argument {argname} not found"),
                format!("{}", exp),
            )
        })
    }
}

impl VMValuePointer {
    pub fn resolve_var(&self, state: &VMState) -> Result<PrimitiveValue, ScratchError> {
        if let VMValuePointer::Variable { name, id } = &self {
            let pv: PrimitiveValue;
            if let Some(var) = state.local_state.read().variables.get(id) {
                pv = var.read().clone();
            } else {
                pv = state
                    .global_state
                    .read()
                    .variables
                    .get(id)
                    .ok_or(ScratchError::not_found(
                        format!("value pointed to by variable pointer '{name}' not found"),
                        format!("resolving variable pointer '{name}' (id={id})"),
                    ))?
                    .read()
                    .clone()
            }
            return pv.try_into().map_err(|e| {
                ScratchError::type_error(e, format!("fetching argument '{name}' (id={id})"))
            });
        }
        Err(ScratchError::type_error(format!("tried to resolve pointer into var, but it does not point to a variable (it pointed to a {self:?})"), format!("resolving into var {self:#?}")))
    }

    pub fn set_var(
        &self,
        state: &VMState,
        value: PrimitiveValue,
    ) -> Result<PrimitiveValue, ScratchError> {
        if let VMValuePointer::Variable { name, id } = &self {
            if let Some(var) = state.local_state.write().variables.get(id) {
                *var.write() = value.clone();
            } else {
                *state
                    .global_state
                    .write()
                    .variables
                    .get_mut(id)
                    .ok_or(ScratchError::not_found(
                        format!("value pointed to by variable pointer '{name}' not found"),
                        format!("resolving variable pointer '{name}' (id={id})"),
                    ))?
                    .write() = value.clone();
            }
            return Ok(value);
        }
        Err(ScratchError::type_error(format!("tried to resolve pointer into var, but it does not point to a variable (it pointed to a {self:?})"), format!("resolving into var {self:#?}")))
    }

    pub fn resolve_list(
        &self,
        state: &VMState,
    ) -> Result<Arc<RwLock<Vec<RwLock<PrimitiveValue>>>>, ScratchError> {
        if let VMValuePointer::List { name, id, .. } = &self {
            let pv: Arc<RwLock<Vec<RwLock<PrimitiveValue>>>>;
            if let Some(var) = state.local_state.read().lists.get(id) {
                pv = Arc::clone(var);
            } else {
                pv = Arc::clone(state.global_state.read().lists.get(id).ok_or(
                    ScratchError::not_found(
                        format!("value pointed to by list pointer  '{name}' not found"),
                        format!("resolving list pointer '{name}' (id={id}) as string"),
                    ),
                )?);
            }
            return Ok(pv);
        }
        Err(ScratchError::type_error(format!("tried to resolve pointer into list, but it does not point to a list (it pointed to a {self:?})"), format!("resolving into list {self:#?}")))
    }

    pub fn resolve_broadcast(&self, state: &VMState) -> Result<String, ScratchError> {
        if let VMValuePointer::Broadcast { name, id } = &self {
            let pv: String;
            if let Some(var) = state.local_state.read().broadcasts.get(id) {
                pv = var.clone();
            } else {
                pv = state
                    .global_state
                    .read()
                    .broadcasts
                    .get(id)
                    .ok_or(ScratchError::not_found(
                        format!("value pointed to by variable pointer '{name}' not found"),
                        format!("resolving variable pointer '{name}' (id={id})"),
                    ))?
                    .clone()
            }
            return Ok(pv);
        }
        Err(ScratchError::type_error(format!("tried to resolve pointer into var, but it does not point to a variable (it pointed to a {self:?})"), format!("resolving into var {self:#?}")))
    }
}

impl VMEvaluable {
    pub fn eval(&self, state: &VMState) -> Result<RichValue, ScratchError> {
        match self {
            VMEvaluable::Bare(rv) => Ok(rv.clone()),
            VMEvaluable::Field(f) => Ok(match &f.pointer {
                None => RichValue::String(f.display_value.clone()),
                Some(p) => {
                    p.resolve_var(state).unwrap_or(
                        PrimitiveValue::String(p.resolve_broadcast(state)?)
                    ).into()
                }
            }),
            VMEvaluable::Pointer(v) => match &v {
                VMValuePointer::Variable { id, name } => {
                    let pv: PrimitiveValue;
                    if let Some(var) = state.local_state.read().variables.get(id) {
                        pv = var.read().clone();
                    } else {
                        pv = state.global_state
                            .read()
                            .variables
                            .get(id)
                            .ok_or(ScratchError::not_found(format!("variable {name} not found"), format!("fetching '{self:?}' (id={id})")))?
                            .read()
                            .clone()
                    }
                    Ok(pv.into())
                }
                VMValuePointer::List { id, .. } => Err(ScratchError::type_error(
                    format!("lists cannot be converted into strings. '{self:?}' points to a list, you may have accidentally dragged a list reporter inside a block slot that only accept strings."),
                    format!("fetching '{self:?}' (id={id}) as string")
                )),
                VMValuePointer::Broadcast { name, ..} => Ok(RichValue::Broadcast(name.to_string())),
            },
            VMEvaluable::Block(b) => eval_exp(b, state),
        }
    }
}

impl VMState {
    pub fn resolve_var(&self, pointer: VMValuePointer) -> Result<PrimitiveValue, ScratchError> {
        pointer.resolve_var(self)
    }

    pub fn set_var(
        &self,
        pointer: VMValuePointer,
        value: PrimitiveValue,
    ) -> Result<PrimitiveValue, ScratchError> {
        pointer.set_var(self, value)
    }
}
