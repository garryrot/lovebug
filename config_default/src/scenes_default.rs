use bp_scheduler::config::actions::{ActionRef, Strength};
use config::{triggers::Trigger, Scene, SceneId, SceneTags};

pub fn default_trigger() -> Vec<Trigger> {
    let default_config: Vec<Trigger> = vec![Trigger::Scene(Scene {
        description: "Scene Default".into(),
        scene_id: SceneId::Any,
        tags: SceneTags::Any,
        actions: vec![
            ActionRef::new(
                "vibrate",
                Strength::RandomFunscript(50, vec!["Blowjob".into(), "Deepthroat".into()]),
            ),
            ActionRef::new(
                "linear", Strength::Constant(50)
            ),
            ActionRef::new(
                "constrict", Strength::Constant(50)
            ),
            ActionRef::new(
                "oscillate", Strength::Constant(50)
            ),
        ],
    })];

    default_config
}

pub fn scene(description: &str, scene_id: SceneId, actions: Vec<ActionRef>) -> Trigger {
    Trigger::Scene(Scene {
        description: description.into(),
        scene_id,
        tags: SceneTags::Any,
        actions,
    })
}