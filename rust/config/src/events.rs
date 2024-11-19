use bp_scheduler::config::actions::ActionRef;
use serde::{Deserialize, Serialize};
use serde_hex::{SerHex, StrictPfx};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Event {
    pub description: String,
    pub event_start: EventTrigger,
    pub event_stop: EventTrigger,
    pub actions: Vec<ActionRef>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TimedEvent {
    pub description: String,
    pub event_start: EventTrigger,
    pub duration_ms: u32,
    pub actions: Vec<ActionRef>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EventTrigger {
    pub event: String,
    pub form: Form,
    pub conditions: Vec<EventCondition>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Form {
    Any,
    Player,
    #[serde(with = "SerHex::<StrictPfx>")]
    FormId(u32),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum EventCondition {
    StrArgEquals(String),
    NumArgEquals(f32),
    NumArgGreaterEquals(f32),
    NumArgSmallerEquals(f32),
}
