use bp_scheduler::config::{actions::Selector, util::read::read_config_dir};
use serde::{Deserialize, Serialize};
use tracing::{debug, info};

pub static CONFIG_DIR: &str = "Data\\F4SE\\Plugins\\Telekinesis2";
pub static PATTERNS_DIR: &str = "Data\\F4SE\\Plugins\\Telekinesis2\\Patterns";
pub static ACTIONS_DIR: &str = "Data\\F4SE\\Plugins\\Telekinesis2\\Actions";
pub static TRIGGERS_DIR: &str = "Data\\F4SE\\Plugins\\Telekinesis2\\Triggers";

pub static LOGGING_SETTINGS: &str = "Logging.json";
pub static DEVICE_SETTINGS: &str = "Devices.json";
pub static BONE_TRACKING: &str = "BoneTracking.json";

pub static VAR_BONE_TRACKING_RATE: &str = "BoneTrackingRate";
pub static VAR_BONE_TRACKING_DEPTH: &str = "BoneTrackingDepth";
pub static VAR_BONE_TRACKING_POS: &str = "BoneTrackingPos";

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TriggerActions(pub Vec<TriggerAction>);

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TriggerAction {
    pub trigger: String,
    pub actions: Vec<ActionRef>,
    pub scene_tags: Selector
}

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq, Hash)]
pub struct ActionRef {
    pub action: String,
    pub strength: Stren,
}

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq, Hash)]
pub enum Stren {
    Constant(i32),
    Variable(String),
    Funscript(i32, String),
    RandomFunscript(i32, Vec<String>)
}

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq, Hash)]
pub enum Variable {
    PlayerActorValue(String),
    BoneTrackingRate,
    BoneTrackingDepth,
    BoneTrackingPos,
}

pub fn read_trigger_actions(trigger_path: &str) -> TriggerActions {
    let trigger_actions = TriggerActions(read_config_dir(trigger_path.into()));
    info!("read {} trigger actions...", trigger_actions.0.len());
    for action in trigger_actions.0.iter() {
        debug!("{:?}", action);
    }
    trigger_actions
}
