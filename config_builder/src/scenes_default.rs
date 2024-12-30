use bp_scheduler::config::actions::{ActionRef, Stren, Variable};
use config::{triggers::Trigger, Scene, SceneId, SceneTags};

pub fn default_scene_bone_tracking() -> Vec<Trigger> {
    let default_config: Vec<Trigger> = vec![
        Trigger::Scene(Scene {
            description: "Default Scene".into(),
            scene_id: SceneId::Any,
            tags: SceneTags::Any,
            actions: vec![
                ActionRef::new("vibrate", Stren::Variable(Variable::BoneTrackingPos)),
                ActionRef::new("oscillate", Stren::Variable(Variable::BoneTrackingRate)),
                ActionRef::new("constrict", Stren::Variable(Variable::BoneTrackingDepth)),
            ],
            track_bones: true,
        }),
        Trigger::Scene(Scene {
            description: "Masturbation Scene".into(),
            scene_id: SceneId::Any,
            tags: SceneTags::Tag("masturbation".into()),
            actions: vec![
                ActionRef::new("masturbation", Stren::Constant(40)),
            ],
            track_bones: true,
        }),
    ];
    default_config
}

pub fn scene(description: &str, scene_id: SceneId, actions: Vec<ActionRef>) -> Trigger {
    Trigger::Scene(Scene {
        description: description.into(),
        scene_id,
        tags: SceneTags::Any,
        actions,
        track_bones: false,
    })
}
