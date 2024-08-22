use anyhow::Result;
use std::fs;

use serde::{Deserialize, Serialize};
use serde_hex::{SerHex,StrictPfx};

use bp_scheduler::settings::actions::*;

// triggers/*.json

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Trigger {
    Scene(Scene),
    Event(Event),
    Timed(TimedEvent)
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Scene {
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
    Tag(String),
    And(Vec<Box<SceneTags>>),
    Or(Vec<Box<SceneTags>>),
}

impl SceneTags {
    pub fn tag(name: &str) -> Box<SceneTags> {
        Box::new(SceneTags::Tag(name.into()))
    }
    pub fn and(items: Vec<Box<SceneTags>>) -> Box<SceneTags> {
        Box::new(SceneTags::And(items))
    }
    pub fn or(items: Vec<Box<SceneTags>>) -> Box<SceneTags> {
        Box::new(SceneTags::Or(items))
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Framework {
    All,
    Sexlab,
    Ostim,
    Love,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Event {
    enable: bool,
    description: String,
    event_start: EventTrigger,
    event_stop: EventTrigger,
    action: Vec<String>,
    body_parts: BodyParts
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TimedEvent {
    enable: bool,
    description: String,
    event_start: EventTrigger,
    duration_ms: u32,
    action: Vec<String>,
    body_parts: BodyParts
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EventTrigger {
    form: Form,
    event: String,
    conditions: Vec<EventCondition>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum EventCondition {
    StrArgEquals(String),
    NumArgEquals(f32),
    NumArgGreaterEquals(f32),
    NumArgSmallerEquals(f32),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Form {
    All,
    Player,
    #[serde(with = "SerHex::<StrictPfx>")]
    FormId(u32)
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
            Trigger::Event(Event {
                enable: true,
                description: "Milk Mod: Feeding Stage".into(),
                event_start: EventTrigger {
                    form: Form::All,
                    event: "MilkQuest.FeedingStage".into(),
                    conditions: vec![],
                },
                event_stop: EventTrigger {
                    form: Form::All,
                    event: "MilkQuest.MilkingStage".into(),
                    conditions: vec![],
                },
                action: vec![
                    "milkmod.feedingstage".into()
                ],
                body_parts: BodyParts::Tags(
                    vec![
                        "Anal".into()
                    ]
                )
            }),
            Trigger::Event(Event {
                enable: true,
                description: "Milk Mod: Milking Stage".into(),
                event_start: EventTrigger {
                    form: Form::All,
                    event: "MilkQuest.MilkingStage".into(),
                    conditions: vec![],
                },
                event_stop: EventTrigger {
                    form: Form::All,
                    event: "MilkQuest.FuckMachineStage".into(),
                    conditions: vec![],
                },
                action: vec![
                    "milkmod.milkingstage".into()
                ],
                body_parts: BodyParts::Tags(
                    vec![
                        "Anal".into(),
                        "Nipple".into()
                    ]
                )
            }),
            Trigger::Event(Event {
                enable: true,
                description: "Milk Mod: Fucking Machine Stage".into(),
                event_start: EventTrigger {
                    form: Form::All,
                    event: "MilkQuest.FuckMachineStage".into(),
                    conditions: vec![],
                },
                event_stop: EventTrigger {
                    form: Form::All,
                    event: "MilkQuest.StartMilkingMachine".into(),
                    conditions: vec![],
                },
                action: vec![
                    "milkmod.fuckingmachinestage".into()
                ],
                body_parts: BodyParts::Tags(
                    vec![
                        "Anal".into(),
                        "Vaginal".into()
                    ]
                )
            }),
            Trigger::Timed(TimedEvent {
                enable: true,
                description: "Milk Mod: Start Milking Machine".into(),
                event_start: EventTrigger {
                    form: Form::All,
                    event: "MilkQuest.StartMilkingMachine".into(),
                    conditions: vec![],
                },
                duration_ms: 10_000,
                action: vec![],
                body_parts: BodyParts::Tags(
                    vec![
                        "Anal".into(),
                        "Vaginal".into(),
                        "Nipple".into()
                    ]
                )
            }),

            Trigger::Scene(Scene {
                enable: false,
                description: "Default scene".into(),
                framework: Framework::All,
                scene_id: SceneId::Any,
                scene_fragment_id: SceneId::Any,
                tags: SceneTags::Or(vec![
                    SceneTags::tag("Anal"),
                    SceneTags::tag("Vaginal"),
                    SceneTags::and(vec![
                        SceneTags::tag("Foo"),
                        SceneTags::tag("Bar")
                    ]),
                ]),
                action: Action { name: "something".into(), control: vec![] },
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
