use anyhow::Result;
use std::fs;

use serde::{Deserialize, Serialize};

// triggers/*.json

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Trigger {
    Scene(TriggerScene),
    Event(TriggerEvent),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TriggerScene {
    enable: bool,
    description: String,
    framework: Framework,
    scene_id: SceneId,
    scene_fragment_id: SceneId,
    tags: SceneTags,
    action: Action,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum SceneId {
    Any,
    Exact(String),
    Contains(String),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum SceneTags {
    And(Vec<String>),
    Or(Vec<String>),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Framework {
    All,
    Sexlab,
    Ostim,
    Love,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TriggerEvent {
    enable: bool,
    event_start: EventTrigger,
    event_stop: EventTrigger,
    action: Action,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EventTrigger {

    form_id: i32,
    name: String,
}

// actions/*.json

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Action {
    File(String),
    Scalar(i32, Vec<ScalarActuators>),
    ScalarDynamic(String, Vec<ScalarActuators>),
    Stroke(),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ScalarActuators {
    Vibrate,
    Oscillate,
    Inflate,
    Constrict,
}

// parse

pub fn read_triggers(trigger_path: &str) -> Result<Vec<Trigger>> {
    let dir: fs::ReadDir = fs::read_dir(trigger_path)?;
    let mut all_items = vec![];
    for entry in dir.into_iter().flatten() {
        let content = fs::read_to_string(entry.file_name()).unwrap_or("".to_owned());
        let mut items = serde_json::from_str(&content).unwrap_or(vec![]);
        all_items.append(&mut items);
    }
    Ok(all_items)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_default_config() {
        let default_config = vec![
            Trigger::Event(TriggerEvent {
                enable: true,
                event_start: EventTrigger {
                    form_id: 0x0101CAFE,
                    name: "VibrateEffectStart".into(),
                },
                event_stop: EventTrigger {
                    form_id: 0x0101CAFE,
                    name: "VibrateEffectStop".into(),
                },
                action: Action::Scalar(
                    100,
                    vec![ScalarActuators::Vibrate, ScalarActuators::Constrict],
                ),
            }),
            Trigger::Scene(TriggerScene {
                enable: false,
                description: "Default scene".into(),
                framework: Framework::All,
                scene_id: SceneId::Any,
                scene_fragment_id: SceneId::Any,
                tags: SceneTags::Or(vec!["Anal".into(), "Vaginal".into()]),
                action: Action::Scalar(
                    100,
                    vec![
                        ScalarActuators::Vibrate,
                        ScalarActuators::Constrict,
                        ScalarActuators::Oscillate,
                    ],
                ),
            }),
        ];
        let strn = serde_json::to_string_pretty(&default_config).unwrap();
        println!("{}", strn);
    }

    #[test]
    fn default_triggers_can_be_read() {
        let result = read_triggers("..\\deploy\\Data\\SKSE\\Plugins\\Lovebug\\Triggers");
    }
}
