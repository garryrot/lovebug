use bp_scheduler::config::actions::{ActionRef, Selector, Stren, Variable};
use config::{triggers::Trigger, Scene, SceneId};

pub fn default_scene_bone_tracking() -> Vec<Trigger> {
    let default_config: Vec<Trigger> = vec![
        Trigger::Scene(Scene {
            description: "Default Scene (Bone Tracked)".into(),
            scene_id: SceneId::Any,
            scene_tags: Selector::Any,
            actions: vec![
                ActionRef::new("vibrate", Stren::Variable(Variable::BoneTrackingPos)),
                ActionRef::new("oscillate", Stren::Variable(Variable::BoneTrackingRate)),
                ActionRef::new("constrict", Stren::Variable(Variable::BoneTrackingDepth)),
            ],
            track_bones: true
        }),
        Trigger::Scene(Scene {
            description: "Default Masturbation Scene".into(),
            scene_id: SceneId::Any,
            scene_tags: Selector::Tag("masturbation".into()),
            actions: vec![ActionRef::new("masturbation", Stren::Constant(40))], 
            track_bones: true
        }),
    ];
    default_config
}

pub fn default_scene_no_bone_tracking() -> Vec<Trigger> {
    let all_funscripts = vec![
        "Tease".into(),
        "Cruel-Tease".into(),
        "On-Off".into(),
        "On-Off-Fast".into(),
        "Sawtooth".into(),
        "Sawtooth-Fast".into(),
        "Square".into(),
        "Wub-Wub-Wub".into(),
    ];

    let default_config: Vec<Trigger> = vec![
        Trigger::Scene(Scene {
            description: "Default Scene (No Bone Tracking)".into(),
            scene_id: SceneId::Any,
            scene_tags: Selector::Any,
            actions: vec![     
                ActionRef::new("vibrate", Stren::RandomFunscript(100, all_funscripts.clone()) ),
                ActionRef::new("oscillate", Stren::RandomFunscript(100, all_funscripts.clone()) ),
                ActionRef::new("constrict", Stren::RandomFunscript(100, all_funscripts) ),
                ActionRef::new("linear", Stren::Constant(50) )
            ],
            track_bones: false
        }),
        Trigger::Scene(Scene {
            description: "Default Masturbation Scene".into(),
            scene_id: SceneId::Any,
            scene_tags: Selector::Tag("masturbation".into()),
            actions: vec![ActionRef::new("masturbation", Stren::Constant(40))], 
            track_bones: false
        }),
    ];
    default_config
}

pub fn scene(description: &str, scene_id: SceneId, actions: Vec<ActionRef>) -> Trigger {
    Trigger::Scene(Scene {
        description: description.into(),
        scene_id,
        scene_tags: Selector::Any,
        actions,
        track_bones: false,
    })
}
