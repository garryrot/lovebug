use std::{sync::atomic::Ordering, time::Duration};

use bp_scheduler::config::actions::ActionRef;
use serde::{Deserialize, Serialize};
use serde_hex::{SerHex, StrictPfx};

use crate::variables::VariableStore;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct Event {
    pub description: String,
    pub event_start: Condition,
    pub event_stop: Condition,
    pub actions: Vec<ActionRef>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct TimedEvent {
    pub description: String,
    pub event_start: Condition,
    pub duration: Duration,
    pub actions: Vec<ActionRef>
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub enum Comparison { 
    Equal(i64),
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
pub struct ActorValueChange {
    pub variable_id: String,
    pub condition: Comparison
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub enum Condition {
    And(Vec<Condition>),
    Or(Vec<Condition>),
    LovebugEvent(String),
    ActorValue(ActorValueChange)
}

impl Condition {
    pub fn matches(&self, vars: &VariableStore, event: Option<&str>) -> bool {
        match self {
            Condition::And(conds) => conds.iter().all(|x| x.matches(vars, event)),
            Condition::Or(conds) => conds.iter().any(|x| x.matches(vars, event)),
            Condition::LovebugEvent(name) => match event {
                Some(mope) => mope == name,
                None => false,
            },
            Condition::ActorValue(cond) => {
                let var = vars.get(&cond.variable_id).map(|x| x.load(Ordering::Relaxed)).unwrap_or(-1);
                match cond.condition {
                    Comparison::Equal(val) => var == val,
                    Comparison::SmallerThan(val) => var < val,
                    Comparison::SmallerEqualThan(val) => var <= val,
                    Comparison::GreaterThan(val) => var > val,
                    Comparison::GreaterEqualThan(val) => var >= val,
                }
            },
        }
    }
}
