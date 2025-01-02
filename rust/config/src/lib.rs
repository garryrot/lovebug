use serde::{Deserialize, Serialize};

use bp_scheduler::config::actions::*;

pub mod bodies;
pub mod body_parts;
pub mod events;
pub mod triggers;
pub mod variables;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Scene {
    pub description: String,
    pub scene_id: SceneId,
    pub scene_tags: Selector,
    pub actions: Vec<ActionRef>,
    pub track_bones: bool,
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
pub enum Framework {
    All,
    AAF,
    Sexlab,
    Ostim,
    Love,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_milkmod_config() {
        let default_config = vec![
            Trigger::Event(Event {
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
                action: vec!["milkmod.feedingstage".into()],
                body_parts: BodyParts::Tags(vec!["Anal".into()]),
            }),
            Trigger::Event(Event {
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
                action: vec!["milkmod.milkingstage".into()],
                body_parts: BodyParts::Tags(vec!["Anal".into(), "Nipple".into()]),
            }),
            Trigger::Event(Event {
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
                action: vec!["milkmod.fuckingmachinestage".into()],
                body_parts: BodyParts::Tags(vec!["Anal".into(), "Vaginal".into()]),
            }),
            Trigger::Timed(TimedEvent {
                description: "Milk Mod: Start Milking Machine".into(),
                event_start: EventTrigger {
                    form: Form::All,
                    event: "MilkQuest.StartMilkingMachine".into(),
                    conditions: vec![],
                },
                duration_ms: 10_000,
                action: vec![],
                body_parts: BodyParts::Tags(vec!["Anal".into(), "Vaginal".into(), "Nipple".into()]),
            })
        ];
        let strn = serde_json::to_string_pretty(&default_config).unwrap();
        println!("{}", strn);
    }
}

