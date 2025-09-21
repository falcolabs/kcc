use std::{
    sync::Arc,
    thread,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

use log::debug;
use parking_lot::RwLock;
use rand::Rng;
use scratch_ast::{
    errors::ScratchError,
    model::{BlockType, RichValue},
};

use crate::vm::internals::{Expression, StackExpression, VMGlobalState, VMLocalState, VMThread};

use super::ScratchResult;

const START_OF_2000_TIMESTAMP: u64 = 946684800;
const MILISECS_IN_A_DAY: u64 = 1000 * 60 * 60 * 24;

#[derive(Clone, Debug)]
pub struct VMState {
    pub global_state: Arc<RwLock<VMGlobalState>>,
    pub local_state: Arc<RwLock<VMLocalState>>,
}

pub fn exec_thread(
    thread: VMThread,
    global_state: Arc<RwLock<VMGlobalState>>,
    local_state: Arc<RwLock<VMLocalState>>,
) -> ScratchResult {
    let state = VMState {
        global_state: Arc::clone(&global_state),
        local_state: Arc::clone(&local_state),
    };
    for stmt in thread.code {
        match stmt {
            Expression::Stack(s) => eval_exp(&s, &state)?,
            _ => todo!(),
        };
    }
    Ok(())
}

#[allow(unused)]
pub fn eval_exp(exp: &StackExpression, state: &VMState) -> Result<RichValue, ScratchError> {
    debug!("exec {}", exp);
    match exp.opcode {
        BlockType::MotionMoveSteps => todo!(),
        BlockType::MotionTurnRight => todo!(),
        BlockType::MotionTurnLeft => todo!(),
        BlockType::MotionGoTo => todo!(),
        BlockType::MotionGoToXY => todo!(),
        BlockType::MotionGlideTo => todo!(),
        BlockType::MotionGlideSecsToXY => todo!(),
        BlockType::MotionPointInDirection => todo!(),
        BlockType::MotionPointTowards => todo!(),
        BlockType::MotionChangeXBy => todo!(),
        BlockType::MotionSetX => todo!(),
        BlockType::MotionChangeYBy => todo!(),
        BlockType::MotionSetY => todo!(),
        BlockType::MotionIfOnEdgeBounce => todo!(),
        BlockType::MotionSetRotationStyle => todo!(),
        BlockType::LooksSayForSecs => {
            let msg = exp.sargstr("MESSAGE", state, exp)?;
            let secs = exp.sargfloat("SECS", state, exp)?;
            println!("{}", msg);
            thread::sleep(Duration::from_millis((secs * 1000.0) as u64));
            Ok(RichValue::success())
        }
        BlockType::LooksSay => {
            let msg = exp.sargstr("MESSAGE", state, exp)?;
            println!("{}", msg);
            Ok(RichValue::success())
        }
        BlockType::LooksThinkForSecs => todo!(),
        BlockType::LooksThink => todo!(),
        BlockType::LooksSwitchBackdropTo => todo!(),
        BlockType::LooksSwitchBackdropToAndWait => todo!(),
        BlockType::LooksNextBackdrop => todo!(),
        BlockType::LooksNextCostume => todo!(),
        BlockType::LooksChangeSizeBy => todo!(),
        BlockType::LooksSetSizeTo => todo!(),
        BlockType::LooksChangeEffectBy => todo!(),
        BlockType::LooksSetEffectTo => todo!(),
        BlockType::LooksClearGraphicEffects => todo!(),
        BlockType::LooksShow => todo!(),
        BlockType::LooksHide => todo!(),
        BlockType::LooksGoToFrontBack => todo!(),
        BlockType::LooksGoForwardBackwardLayers => todo!(),
        BlockType::SoundStopallSounds => todo!(),
        BlockType::SoundChangeEffectBy => todo!(),
        BlockType::SoundSetEffectTo => todo!(),
        BlockType::SoundClearEffects => todo!(),
        BlockType::SoundChangeVolumeBy => todo!(),
        BlockType::SoundSetVolumeTo => todo!(),
        BlockType::EventWhenFlagClicked => Ok(RichValue::success()),
        BlockType::EventWhenKeyPressed => todo!(),
        BlockType::EventWhenStageClicked => todo!(),
        BlockType::EventWhenThisSpriteClicked => todo!(),
        BlockType::EventWhenBackdropSwitchesTo => todo!(),
        BlockType::EventWhenGreaterThan => todo!(),
        BlockType::EventWhenBroadcastReceived => Ok(RichValue::success()),
        BlockType::EventBroadcast => todo!(),
        BlockType::EventBroadcastandWait => todo!(),
        BlockType::ControlWait => todo!(),
        BlockType::ControlRepeat => todo!(),
        BlockType::ControlIf => todo!(),
        BlockType::ControlIfElse => todo!(),
        BlockType::ControlStop => todo!(),
        BlockType::ControlCreateCloneOf => todo!(),
        BlockType::ControlStartAsClone => todo!(),
        BlockType::ControlDeleteThisClone => todo!(),
        BlockType::SensingTouchingObject => todo!(),
        BlockType::SensingTouchingColor => todo!(),
        BlockType::SensingColorIsTouchingColor => todo!(),
        BlockType::SensingDistanceTo => todo!(),
        BlockType::SensingKeyPressed => todo!(),
        BlockType::SensingMouseDown => todo!(),
        BlockType::SensingMouseX => todo!(),
        BlockType::SensingMouseY => todo!(),
        BlockType::SensingSetDragMode => todo!(),
        BlockType::SensingResetTimer => todo!(),
        BlockType::SensingDaysSince2000 => Ok(RichValue::Number(
            SystemTime::now()
                .duration_since(UNIX_EPOCH + Duration::from_secs(START_OF_2000_TIMESTAMP))
                .map_err(|e| {
                    ScratchError::internal(
                        format!("time travelled into the past: {e}"),
                        format!(
                            "<vm::intepreter::SensingDaysSince2000> executing block {:?} (id={})",
                            exp.opcode, exp.original_block.obj_id
                        ),
                    )
                })?
                .as_millis() as f64
                / MILISECS_IN_A_DAY as f64,
        )),
        BlockType::SensingUsername => todo!(),
        BlockType::OperatorAdd => {
            let n1 = exp.sargfloat("NUM1", state, exp)?;
            let n2 = exp.sargfloat("NUM2", state, exp)?;
            Ok(RichValue::Number(n1 + n2))
        }
        BlockType::OperatorSubtract => {
            let n1 = exp.sargfloat("NUM1", state, exp)?;
            let n2 = exp.sargfloat("NUM2", state, exp)?;
            Ok(RichValue::Number(n1 - n2))
        }
        BlockType::OperatorMultiply => {
            let n1 = exp.sargfloat("NUM1", state, exp)?;
            let n2 = exp.sargfloat("NUM2", state, exp)?;
            Ok(RichValue::Number(n1 * n2))
        }
        BlockType::OperatorDivide => {
            let n1 = exp.sargfloat("NUM1", state, exp)?;
            let n2 = exp.sargfloat("NUM2", state, exp)?;
            Ok(RichValue::Number(n1 / n2))
        }
        BlockType::OperatorRandom => {
            let mut rng = rand::rng();
            let lower = exp.sargfloat("FROM", state, exp)?;
            let upper = exp.sargfloat("TO", state, exp)?;
            if (lower.fract() == 0.0 && upper.fract() == 0.0) {
                return Ok(RichValue::Integer(
                    rng.random_range(lower as i64..upper as i64),
                ));
            }
            Ok(RichValue::Number(rng.random_range(lower..upper)))
        }
        BlockType::OperatorGt => {
            let n1 = exp.sargfloat("OPERAND1", state, exp)?;
            let n2 = exp.sargfloat("OPERAND2", state, exp)?;
            Ok(RichValue::Boolean(n1 > n2))
        }
        BlockType::OperatorLt => {
            let n1 = exp.sargfloat("OPERAND1", state, exp)?;
            let n2 = exp.sargfloat("OPERAND2", state, exp)?;
            Ok(RichValue::Boolean(n1 < n2))
        }
        BlockType::OperatorEquals => {
            let n1 = exp.sargfloat("OPERAND1", state, exp)?;
            let n2 = exp.sargfloat("OPERAND2", state, exp)?;
            Ok(RichValue::Boolean(n1 == n2))
        }
        BlockType::OperatorAnd => {
            let n1 = exp.sargbool("OPERAND1", state, exp)?;
            let n2 = exp.sargbool("OPERAND2", state, exp)?;
            Ok(RichValue::Boolean(n1 && n2))
        }
        BlockType::OperatorOr => {
            let n1 = exp.sargbool("OPERAND1", state, exp)?;
            let n2 = exp.sargbool("OPERAND2", state, exp)?;
            Ok(RichValue::Boolean(n1 || n2))
        }
        BlockType::OperatorNot => {
            let n1 = exp.sargbool("OPERAND", state, exp)?;
            Ok(RichValue::Boolean(!n1))
        }
        BlockType::OperatorJoin => {
            let n1 = exp.sargstr("STRING1", state, exp)?;
            let n2 = exp.sargstr("STRING2", state, exp)?;
            Ok(RichValue::String(n1 + &n2))
        }
        BlockType::OperatorLetterOf => {
            let n1 = exp.sargfloat("LETTER", state, exp)? as usize;
            let n2 = exp.sargstr("STRING", state, exp)?;
            Ok(RichValue::String(match n2.chars().nth(n1) {
                Some(c) => c.to_string(),
                None => "".to_string(),
            }))
        }
        BlockType::OperatorLength => {
            let s: String = exp.sargstr("STRING", state, exp)?;
            Ok(RichValue::Number(s.len() as f64))
        }
        BlockType::OperatorContains => {
            let n1 = exp.sargstr("STRING1", state, exp)?;
            let n2 = exp.sargstr("STRING2", state, exp)?;
            Ok(RichValue::Boolean(n1.contains(&n2)))
        }
        BlockType::OperatorMod => {
            let n1 = exp.sargfloat("NUM1", state, exp)?;
            let n2 = exp.sargfloat("NUM2", state, exp)?;
            Ok(RichValue::Number(n1 % n2))
        }
        BlockType::OperatorRound => {
            let n1 = exp.sargfloat("NUM", state, exp)?;
            Ok(RichValue::Integer(n1.round() as i64))
        }
        BlockType::OperatorMathop => {
            let n = exp.sargfloat("NUM", state, exp)?;
            let op = exp.sargstr("OPERATOR", state, exp)?;
            match op.as_str() {
                "abs" => Ok(RichValue::Number(n.abs())),
                "floor" => Ok(RichValue::Number(n.floor())),
                "ceiling" => Ok(RichValue::Number(n.ceil())),
                "sqrt" => Ok(RichValue::Number(n.sqrt())),
                "sin" => Ok(RichValue::Number(n.sin())),
                "cos" => Ok(RichValue::Number(n.cos())),
                "tan" => Ok(RichValue::Number(n.tan())),
                "asin" => Ok(RichValue::Number(n.asin())),
                "acos" => Ok(RichValue::Number(n.acos())),
                "atan" => Ok(RichValue::Number(n.atan())),
                "ln" => Ok(RichValue::Number(n.ln())),
                "log" => Ok(RichValue::Number(n.log10())),
                "e ^" => Ok(RichValue::Number(n.exp())),
                "10 ^" => Ok(RichValue::Number(n.powi(10))),
                _ => Err(ScratchError::syntax_error(
                    format!("unknown math operator {op}"),
                    format!("block {:?} (id={})", exp.opcode, exp.original_block.obj_id),
                )),
            }
        }

        BlockType::ControlWait => todo!(),
        BlockType::ControlRepeat => todo!(),
        BlockType::ControlIf => todo!(),
        BlockType::ControlIfElse => todo!(),
        BlockType::ControlStop => todo!(),
        BlockType::ControlCreateCloneOf => todo!(),
        BlockType::ControlStartAsClone => todo!(),
        BlockType::ControlDeleteThisClone => todo!(),
        BlockType::DataSetVariableTo => {
            let value = exp.sargraw("VALUE", exp)?.eval(state)?;
            let var = exp.sargptr("VARIABLE", exp)?;

            state.set_var(var, value.into())?;

            Ok(RichValue::success())
        }
        BlockType::DataChangeVariableBy => {
            let delta: f64 = exp.sargraw("VALUE", exp)?.eval(state)?.try_into()?;
            let var = exp.sargptr("VARIABLE", exp)?;
            let src: f64 = var.resolve_var(state)?.try_into()?;
            state.set_var(var, (src + delta).into())?;

            Ok(RichValue::success())
        }
        BlockType::DataShowVariable => todo!(),
        BlockType::DataHideVariable => todo!(),
        BlockType::DataAddToList => {
            let list = exp.sargptr("LIST", exp)?.resolve_list(state)?;
            let item = exp.sargraw("ITEM", exp)?.eval(state)?;
            list.write().push(RwLock::new(item.into()));
            Ok(RichValue::success())
        }
        BlockType::DataListDeleteElement => {
            let list = exp.sargptr("LIST", exp)?.resolve_list(state)?;
            let index: usize = exp.sargfloat("INDEX", state, exp)? as usize;
            list.write().remove(index);
            Ok(RichValue::success())
        }
        BlockType::DataListClear => {
            let list = exp.argptr("LIST")?.resolve_list(state)?;
            list.write().clear();
            Ok(RichValue::success())
        }
        BlockType::DataListInsertAt => {
            let list = exp.sargptr("LIST", exp)?.resolve_list(state)?;
            let item = exp.sargraw("ITEM", exp)?.eval(state)?;
            let index: usize = exp.sargfloat("INDEX", state, exp)? as usize;
            list.write().insert(index, RwLock::new(item.into()));
            Ok(RichValue::success())
        }
        BlockType::DataListReplaceItem => {
            let list = exp.sargptr("LIST", exp)?.resolve_list(state)?;
            let item = exp.sargraw("ITEM", exp)?.eval(state)?;
            let index: usize = exp.sargfloat("INDEX", state, exp)? as usize;
            std::mem::replace(
                list.write().get_mut(index).ok_or(ScratchError::internal(
                    "failed to get mutable reference of vecitem",
                    "interpreter, DataListReplaceItem",
                ))?,
                RwLock::new(item.into()),
            );
            Ok(RichValue::success())
        }
        BlockType::DataListItemAt => {
            let list = exp.sargptr("LIST", exp)?.resolve_list(state)?;
            let index: usize = exp.sargfloat("INDEX", state, exp)? as usize;
            let result = match list.read().get(index) {
                Some(v) => v.read().clone().into(),
                None => RichValue::String("".to_string()),
            };
            Ok(result)
        }
        BlockType::DataListIndexOf => {
            let list = exp.sargptr("LIST", exp)?.resolve_list(state)?;
            let item: RichValue = exp.sargraw("ITEM", exp)?.eval(state)?;
            for (i, e) in list.read().iter().enumerate() {
                let inner: RichValue = e.read().clone().into();
                if inner == item {
                    return Ok(RichValue::Number(i as f64));
                }
            }
            Ok(RichValue::Number(0.0))
        }
        BlockType::DataListLengthOf => {
            let list = exp.sargptr("LIST", exp)?.resolve_list(state)?;
            let r = list.read().len() as f64;
            Ok(RichValue::Number(r))
        }
        BlockType::DataListContainsItem => {
            let list = exp.sargptr("LIST", exp)?.resolve_list(state)?;
            let item: RichValue = exp.sargraw("ITEM", exp)?.eval(state)?;

            for i in list.read().iter() {
                let inner: RichValue = i.read().clone().into();
                if inner == item {
                    return Ok(RichValue::Boolean(true));
                }
            }
            Ok(RichValue::Boolean(false))
        }
        BlockType::DataListShow => todo!(),
        BlockType::DataListHide => todo!(),

        BlockType::ProceduresDefinition => Ok(RichValue::success()),
        BlockType::ProceduresCall => todo!(),
        BlockType::ArgumentReporterStringNumber => todo!(),
        BlockType::ArgumentReporterBoolean => todo!(),

        BlockType::ProceduresPrototype => todo!(),
        BlockType::ArgumentEditorBoolean => todo!(),
        BlockType::ArgumentEditorStringNumber => todo!(),
        BlockType::Note => todo!(),
        BlockType::MathPositiveNumber => todo!(),
        BlockType::MathWholeNumber => todo!(),
        BlockType::MathInteger => todo!(),
        BlockType::MathAngle => todo!(),
        BlockType::ColourPicker => todo!(),
        BlockType::Text => todo!(),
        BlockType::DataVariable => todo!(),
        BlockType::DataListContents => todo!(),
    }
}
