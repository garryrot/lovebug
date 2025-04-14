use std::time::Duration;

use bp_scheduler::config::actions::ActionRef;
use serde::{Deserialize, Serialize};
use serde_hex::{SerHex, StrictPfx};

use crate::{keyword_store::KeywordStore, variable_store::VariableStore};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct Event {
    pub description: String,
    pub start: Vec<Condition>,
    pub stop: Vec<Condition>,
    pub actions: Vec<ActionRef>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct TimedEvent {
    pub description: String,
    pub start: Vec<Condition>,
    pub duration: Duration,
    pub actions: Vec<ActionRef>
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub enum ValueRange { 
    Equals(i64),
    SmallerThan(i64),
    SmallerEqualThan(i64),
    GreaterThan(i64),
    GreaterEqualThan(i64)
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Form {
    Any,
    #[serde(with = "SerHex::<StrictPfx>")]
    FormId(u32),
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct ActorValue {
    pub editor_id: String,
    pub value: ValueRange
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub enum Condition {
    And(Vec<Condition>),
    Or(Vec<Condition>),
    ControlEvent(String),
    ActorValue(ActorValue),
    PlayerHasKeyword(String),
    PlayerWithoutKeyword(String)
}

impl Condition {
    pub fn matches(&self, vars: &VariableStore, kws: &KeywordStore, event: Option<&str>) -> bool {
        match self {
            Condition::And(conds) => conds.iter().all(|x| x.matches(vars, kws, event)),
            Condition::Or(conds) => conds.iter().any(|x| x.matches(vars, kws, event)),
            Condition::ControlEvent(name) => match event {
                        Some(mope) => mope == name,
                        None => false,
            },
            Condition::PlayerHasKeyword(editor_id) => { kws.has_keyword(editor_id) }
            Condition::PlayerWithoutKeyword(editor_id) => { ! kws.has_keyword(editor_id) },
            Condition::ActorValue(cond) => {
                        let var = vars.get_value(&cond.editor_id).unwrap_or(-1);
                        match cond.value {
                            ValueRange::Equals(val) => var == val,
                            ValueRange::SmallerThan(val) => var < val,
                            ValueRange::SmallerEqualThan(val) => var <= val,
                            ValueRange::GreaterThan(val) => var > val,
                            ValueRange::GreaterEqualThan(val) => var >= val,
                        }
                    },
        }
    }
}
