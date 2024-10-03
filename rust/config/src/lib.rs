use serde::{Deserialize, Serialize};
use serde_hex::{SerHex,StrictPfx};

use bp_scheduler::config::actions::*;

pub mod triggers;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Trigger {
    Scene(Scene),
    Event(Event),
    Timed(TimedEvent)
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Scene {
    pub description: String,
    pub scene_id: SceneId,
    pub tags: SceneTags,
    pub actions: Vec<ActionRef>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Event {
    pub enable: bool,
    pub description: String,
    pub event_start: EventTrigger,
    pub event_stop: EventTrigger,
    pub action: Vec<String>,
    pub body_parts: BodyParts
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TimedEvent {
    pub enable: bool,
    pub description: String,
    pub event_start: EventTrigger,
    pub duration_ms: u32,
    pub action: Vec<String>,
    pub body_parts: BodyParts
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum SceneId {
    Any,
    Exact(String),
    Contains(String),
}

impl SceneId {
    pub fn matches(&self, scene_name: &str) -> bool {
        match self {
            SceneId::Any => true,
            SceneId::Exact(scene) => scene == scene_name,
            SceneId::Contains(needle) => scene_name.contains(needle),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum SceneTags {
    Any,
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
    pub fn matches( &self, tags: &Vec<String>) -> bool {
        match self {
            SceneTags::Any => true,
            SceneTags::Tag(tag) => tags.contains(tag),
            SceneTags::And(items) => items.iter().all( |x| x.matches(tags)),
            SceneTags::Or(items) => items.iter().any( |x| x.matches(tags)),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Framework {
    All,
    AAF,
    Sexlab,
    Ostim,
    Love
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EventTrigger {
    pub form: Form,
    pub event: String,
    pub conditions: Vec<EventCondition>,
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_milkmod_config() {
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
                description: "Default scene".into(),
                scene_id: SceneId::Any,
                tags: SceneTags::Or(vec![
                    SceneTags::tag("Anal"),
                    SceneTags::tag("Vaginal"),
                    SceneTags::and(vec![
                        SceneTags::tag("Foo"),
                        SceneTags::tag("Bar")
                    ]),
                ]),
                actions: vec![ "something".into() ],
            }),
        ];
        let strn = serde_json::to_string_pretty(&default_config).unwrap();
        println!("{}", strn);
    }
}
