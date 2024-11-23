use bp_scheduler::config::actions::{ActionRef, Stren, Variable};
use config::{triggers::Trigger, Scene, SceneId, SceneTags};

pub fn default_scene_bone_tracking() -> Vec<Trigger> {
    let default_config: Vec<Trigger> = vec![Trigger::Scene(Scene {
        description: "Scene Default".into(),
        scene_id: SceneId::Any,
        tags: SceneTags::Any,
        actions: vec![
            ActionRef::new("vibrate", Stren::Variable(Variable::BoneTrackingRate)),
            ActionRef::new("linear", Stren::Variable(Variable::BoneTrackingRate)),
            ActionRef::new("oscillate", Stren::Variable(Variable::BoneTrackingRate)),
            ActionRef::new("constrict", Stren::Variable(Variable::BoneTrackingDepth)),
        ],
    })];

    default_config
}

pub fn default_scene_no_bone_tracking() -> Vec<Trigger> {
    let default_config: Vec<Trigger> = vec![Trigger::Scene(Scene {
        description: "Scene Default".into(),
        scene_id: SceneId::Any,
        tags: SceneTags::Any,
        actions: vec![
            ActionRef::new("vibrate", Stren::Constant(80)),
            ActionRef::new("linear", Stren::Constant(80)),
            ActionRef::new("oscillate", Stren::Constant(80)),
            ActionRef::new("constrict", Stren::Constant(50)),
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
