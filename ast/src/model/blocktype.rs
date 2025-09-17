#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum BlockType {
    #[cfg_attr(feature = "serde", serde(rename = "motion_movesteps"))]
    MotionMoveSteps,
    #[cfg_attr(feature = "serde", serde(rename = "motion_turnright"))]
    MotionTurnRight,
    #[cfg_attr(feature = "serde", serde(rename = "motion_turnleft"))]
    MotionTurnLeft,
    #[cfg_attr(feature = "serde", serde(rename = "motion_goto"))]
    MotionGoTo,
    #[cfg_attr(feature = "serde", serde(rename = "motion_gotoxy"))]
    MotionGoToXY,
    #[cfg_attr(feature = "serde", serde(rename = "motion_glideto"))]
    MotionGlideTo,
    #[cfg_attr(feature = "serde", serde(rename = "motion_glidesecstoxy"))]
    MotionGlideSecsToXY,
    #[cfg_attr(feature = "serde", serde(rename = "motion_pointindirection"))]
    MotionPointInDirection,
    #[cfg_attr(feature = "serde", serde(rename = "motion_pointtowards"))]
    MotionPointTowards,
    #[cfg_attr(feature = "serde", serde(rename = "motion_changexby"))]
    MotionChangeXBy,
    #[cfg_attr(feature = "serde", serde(rename = "motion_setx"))]
    MotionSetX,
    #[cfg_attr(feature = "serde", serde(rename = "motion_changeyby"))]
    MotionChangeYBy,
    #[cfg_attr(feature = "serde", serde(rename = "motion_sety"))]
    MotionSetY,
    #[cfg_attr(feature = "serde", serde(rename = "motion_ifonedgebounce"))]
    MotionIfOnEdgeBounce,
    #[cfg_attr(feature = "serde", serde(rename = "motion_setrotationstyle"))]
    MotionSetRotationStyle,

    #[cfg_attr(feature = "serde", serde(rename = "looks_sayforsecs"))]
    LooksSayForSecs,
    #[cfg_attr(feature = "serde", serde(rename = "looks_say"))]
    LooksSay,
    #[cfg_attr(feature = "serde", serde(rename = "looks_thinkforsecs"))]
    LooksThinkForSecs,
    #[cfg_attr(feature = "serde", serde(rename = "looks_think"))]
    LooksThink,
    #[cfg_attr(feature = "serde", serde(rename = "looks_switchbackdropto"))]
    LooksSwitchBackdropTo,
    #[cfg_attr(feature = "serde", serde(rename = "looks_switchbackdroptoandwait"))]
    LooksSwitchBackdropToAndWait,
    #[cfg_attr(feature = "serde", serde(rename = "looks_nextbackdrop"))]
    LooksNextBackdrop,
    #[cfg_attr(feature = "serde", serde(rename = "looks_nextcostume"))]
    LooksNextCostume,
    #[cfg_attr(feature = "serde", serde(rename = "looks_changesizeby"))]
    LooksChangeSizeBy,
    #[cfg_attr(feature = "serde", serde(rename = "looks_setsizeto"))]
    LooksSetSizeTo,
    #[cfg_attr(feature = "serde", serde(rename = "looks_changeeffectby"))]
    LooksChangeEffectBy,
    #[cfg_attr(feature = "serde", serde(rename = "looks_seteffectto"))]
    LooksSetEffectTo,
    #[cfg_attr(feature = "serde", serde(rename = "looks_cleargraphiceffects"))]
    LooksClearGraphicEffects,
    #[cfg_attr(feature = "serde", serde(rename = "looks_show"))]
    LooksShow,
    #[cfg_attr(feature = "serde", serde(rename = "looks_hide"))]
    LooksHide,
    #[cfg_attr(feature = "serde", serde(rename = "looks_gotofrontback"))]
    LooksGoToFrontBack,
    #[cfg_attr(feature = "serde", serde(rename = "looks_goforwardbackwardlayers"))]
    LooksGoForwardBackwardLayers,

    #[cfg_attr(feature = "serde", serde(rename = "sound_stopallsounds"))]
    SoundStopallSounds,
    #[cfg_attr(feature = "serde", serde(rename = "sound_changeeffectby"))]
    SoundChangeEffectBy,
    #[cfg_attr(feature = "serde", serde(rename = "sound_seteffectto"))]
    SoundSetEffectTo,
    #[cfg_attr(feature = "serde", serde(rename = "sound_cleareffects"))]
    SoundClearEffects,
    #[cfg_attr(feature = "serde", serde(rename = "sound_changevolumeby"))]
    SoundChangeVolumeBy,
    #[cfg_attr(feature = "serde", serde(rename = "sound_setvolumeto"))]
    SoundSetVolumeTo,

    #[cfg_attr(feature = "serde", serde(rename = "event_whenflagclicked"))]
    EventWhenFlagClicked,
    #[cfg_attr(feature = "serde", serde(rename = "event_whenkeypressed"))]
    EventWhenKeyPressed,
    #[cfg_attr(feature = "serde", serde(rename = "event_whenstageclicked"))]
    EventWhenStageClicked,
    #[cfg_attr(feature = "serde", serde(rename = "event_whenthisspriteclicked"))]
    EventWhenThisSpriteClicked,
    #[cfg_attr(feature = "serde", serde(rename = "event_whenbackdropswitchesto"))]
    EventWhenBackdropSwitchesTo,
    #[cfg_attr(feature = "serde", serde(rename = "event_whengreaterthan"))]
    EventWhenGreaterThan,
    #[cfg_attr(feature = "serde", serde(rename = "event_whenbroadcastreceived"))]
    EventWhenBroadcastReceived,
    #[cfg_attr(feature = "serde", serde(rename = "event_broadcast"))]
    EventBroadcast,
    #[cfg_attr(feature = "serde", serde(rename = "event_broadcastandwait"))]
    EventBroadcastandWait,

    #[cfg_attr(feature = "serde", serde(rename = "control_wait"))]
    ControlWait,
    #[cfg_attr(feature = "serde", serde(rename = "control_repeat"))]
    ControlRepeat,
    #[cfg_attr(feature = "serde", serde(rename = "control_if"))]
    ControlIf,
    #[cfg_attr(feature = "serde", serde(rename = "control_if_else"))]
    ControlIfElse,
    #[cfg_attr(feature = "serde", serde(rename = "control_stop"))]
    ControlStop,
    #[cfg_attr(feature = "serde", serde(rename = "control_create_clone_of"))]
    ControlCreateCloneOf,
    #[cfg_attr(feature = "serde", serde(rename = "control_start_as_clone"))]
    ControlStartAsClone,
    #[cfg_attr(feature = "serde", serde(rename = "control_delete_this_clone"))]
    ControlDeleteThisClone,

    #[cfg_attr(feature = "serde", serde(rename = "sensing_touchingobject"))]
    SensingTouchingObject,
    #[cfg_attr(feature = "serde", serde(rename = "sensing_touchingcolor"))]
    SensingTouchingColor,
    #[cfg_attr(feature = "serde", serde(rename = "sensing_coloristouchingcolor"))]
    SensingColorIsTouchingColor,
    #[cfg_attr(feature = "serde", serde(rename = "sensing_distanceto"))]
    SensingDistanceTo,
    #[cfg_attr(feature = "serde", serde(rename = "sensing_keypressed"))]
    SensingKeyPressed,
    #[cfg_attr(feature = "serde", serde(rename = "sensing_mousedown"))]
    SensingMouseDown,
    #[cfg_attr(feature = "serde", serde(rename = "sensing_mousex"))]
    SensingMouseX,
    #[cfg_attr(feature = "serde", serde(rename = "sensing_mousey"))]
    SensingMouseY,
    #[cfg_attr(feature = "serde", serde(rename = "sensing_setdragmode"))]
    SensingSetDragMode,
    #[cfg_attr(feature = "serde", serde(rename = "sensing_resettimer"))]
    SensingResetTimer,
    #[cfg_attr(feature = "serde", serde(rename = "sensing_dayssince2000"))]
    SensingDaysSince2000,
    #[cfg_attr(feature = "serde", serde(rename = "sensing_username"))]
    SensingUsername,

    #[cfg_attr(feature = "serde", serde(rename = "operator_add"))]
    OperatorAdd,
    #[cfg_attr(feature = "serde", serde(rename = "operator_subtract"))]
    OperatorSubtract,
    #[cfg_attr(feature = "serde", serde(rename = "operator_multiply"))]
    OperatorMultiply,
    #[cfg_attr(feature = "serde", serde(rename = "operator_divide"))]
    OperatorDivide,
    #[cfg_attr(feature = "serde", serde(rename = "operator_random"))]
    OperatorRandom,
    #[cfg_attr(feature = "serde", serde(rename = "operator_gt"))]
    OperatorGt,
    #[cfg_attr(feature = "serde", serde(rename = "operator_lt"))]
    OperatorLt,
    #[cfg_attr(feature = "serde", serde(rename = "operator_equals"))]
    OperatorEquals,
    #[cfg_attr(feature = "serde", serde(rename = "operator_and"))]
    OperatorAnd,
    #[cfg_attr(feature = "serde", serde(rename = "operator_or"))]
    OperatorOr,
    #[cfg_attr(feature = "serde", serde(rename = "operator_not"))]
    OperatorNot,
    #[cfg_attr(feature = "serde", serde(rename = "operator_join"))]
    OperatorJoin,
    #[cfg_attr(feature = "serde", serde(rename = "operator_letter_of"))]
    OperatorLetterOf,
    #[cfg_attr(feature = "serde", serde(rename = "operator_length"))]
    OperatorLength,
    #[cfg_attr(feature = "serde", serde(rename = "operator_contains"))]
    OperatorContains,
    #[cfg_attr(feature = "serde", serde(rename = "operator_mod"))]
    OperatorMod,
    #[cfg_attr(feature = "serde", serde(rename = "operator_round"))]
    OperatorRound,
    #[cfg_attr(feature = "serde", serde(rename = "operator_mathop"))]
    OperatorMathop,

    // data_variable and data_listcontents are listed in the hidden section.
    // They are represented using the enum format: [13, "listName"] for lists,
    // and [12, "varName"] for variables.
    #[cfg_attr(feature = "serde", serde(rename = "data_setvariableto"))]
    DataSetVariableTo,
    #[cfg_attr(feature = "serde", serde(rename = "data_changevariableby"))]
    DataChangeVariableBy,
    #[cfg_attr(feature = "serde", serde(rename = "data_showvariable"))]
    DataShowVariable,
    #[cfg_attr(feature = "serde", serde(rename = "data_hidevariable"))]
    DataHideVariable,

    #[cfg_attr(feature = "serde", serde(rename = "data_addtolist"))]
    DataAddToList,
    #[cfg_attr(feature = "serde", serde(rename = "data_deleteoflist"))]
    DataListDeleteElement,
    #[cfg_attr(feature = "serde", serde(rename = "data_deletealloflist"))]
    DataListClear,
    #[cfg_attr(feature = "serde", serde(rename = "data_insertatlist"))]
    DataListInsertAt,
    #[cfg_attr(feature = "serde", serde(rename = "data_replaceitemoflist"))]
    DataListReplaceItem,
    #[cfg_attr(feature = "serde", serde(rename = "data_itemoflist"))]
    DataListItemAt,
    #[cfg_attr(feature = "serde", serde(rename = "data_itemnumoflist"))]
    DataListIndexOf,
    #[cfg_attr(feature = "serde", serde(rename = "data_lengthoflist"))]
    DataListLengthOf,
    #[cfg_attr(feature = "serde", serde(rename = "data_listcontainsitem"))]
    DataListContainsItem,
    #[cfg_attr(feature = "serde", serde(rename = "data_showlist"))]
    DataListShow,
    #[cfg_attr(feature = "serde", serde(rename = "data_hidelist"))]
    DataListHide,

    #[cfg_attr(feature = "serde", serde(rename = "procedures_definition"))]
    ProceduresDefinition,
    #[cfg_attr(feature = "serde", serde(rename = "procedures_call"))]
    ProceduresCall,
    #[cfg_attr(feature = "serde", serde(rename = "argument_reporter_string_number"))]
    ArgumentReporterStringNumber,
    #[cfg_attr(feature = "serde", serde(rename = "argument_reporter_boolean"))]
    ArgumentReporterBoolean,

    // Hidden, but still legal blocks
    #[cfg_attr(feature = "serde", serde(rename = "procedures_prototype"))]
    ProceduresPrototype,
    #[cfg_attr(feature = "serde", serde(rename = "argument_editor_boolean"))]
    ArgumentEditorBoolean,
    #[cfg_attr(feature = "serde", serde(rename = "argument_editor_string_number"))]
    ArgumentEditorStringNumber,
    #[cfg_attr(feature = "serde", serde(rename = "note"))]
    Note,
    #[cfg_attr(feature = "serde", serde(rename = "math_positive_number"))]
    MathPositiveNumber,
    #[cfg_attr(feature = "serde", serde(rename = "math_whole_number"))]
    MathWholeNumber,
    #[cfg_attr(feature = "serde", serde(rename = "math_integer"))]
    MathInteger,
    #[cfg_attr(feature = "serde", serde(rename = "math_angle"))]
    MathAngle,
    #[cfg_attr(feature = "serde", serde(rename = "colour_picker"))]
    ColourPicker,
    #[cfg_attr(feature = "serde", serde(rename = "text"))]
    Text,
    #[cfg_attr(feature = "serde", serde(rename = "data_variable"))]
    DataVariable,
    #[cfg_attr(feature = "serde", serde(rename = "data_listcontents"))]
    DataListContents,
}
