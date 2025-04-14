use serde::{Deserialize, Serialize};

use bp_scheduler::config::actions::*;

pub mod bodies;
pub mod body_parts;
pub mod events;
pub mod triggers;
pub mod variables;
pub mod variable_store;
pub mod keyword_store;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct Scene {
    pub description: String,
    pub scene_id: SceneId,
    pub scene_tags: Selector,
    pub actions: Vec<ActionRef>,
    pub track_bones: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
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

#[cfg(test)]
mod tests {
    use crate::{events::{ActorValueChange, Comparison, Condition, Event}, keyword_store::{KeywordSource, KeywordStore}, triggers::Trigger, variable_store::{VariableSource, VariableStore}, variables::PlayerActorValue};

    struct KwFTest {}
    impl KeywordSource for KwFTest {
        fn player_has_keyword(&self, _editor_id: &str) -> bool {
            true
        }
    }

    struct FakeVSForTest {}
    impl VariableSource for FakeVSForTest {
        fn get_player_actor_value(&self, _editor_id: &str) -> f32 {
            5.0
        }
        fn get_form_id(&self, _editor_id: &str) -> u32 {
            1
        }
    }

    #[test]
    fn test_start_condition() {
        let vs = VariableStore::init(Box::new(FakeVSForTest {}), vec![PlayerActorValue { editor_id: "VAR_X".into(), min: 0.0, max: 5.0 }], vec![]);
        let kws = KeywordStore::init(Box::new(KwFTest {}), vec![ "KW_Y".into() ]);
        let x = Event {
            description: format!("DD: Vibrator Vaginal {}", 5),
            start: vec![
                Condition::HasKeyword("KW_Y".into()),
                Condition::ActorValue(ActorValueChange {
                    variable_id: "VAR_X".into(),
                    condition: Comparison::Equal(5),
                }),
            ],
            stop: vec![ Condition::HasNotKeyword("KW_Y".into()) ],
            actions: vec![],
        };

        let matches_condition = x.start.iter().all(|x| x.matches(&vs, &kws, None));
        assert!(matches_condition, "condition matches with actual value");
    }
}

