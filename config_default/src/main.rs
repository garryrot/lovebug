use std::fs::{self};

use bp_scheduler::config::actions::*;
use config::*;
use serde::Serialize;

fn main() {
    let config_dir = "deploy/Data/F4SE/Plugins/Lovebug";

    let triggers = vec![
        ("Default.json", get_default_trigger()),
    ];
    let actions = vec![
        ("Default.json", get_default_actions())
    ];

    for trigger in triggers {
        let path = format!("../{}/Triggers/{}", config_dir, trigger.0);
        write_file( path, trigger.1 );
    }
    for action in actions {
        let path = format!("../{}/Actions/{}", config_dir, action.0);
        write_file( path, action.1 );
    }
}

fn get_default_trigger() -> Vec<Trigger> {
    let default_config: Vec<Trigger> = vec![Trigger::Scene(Scene {
        enabled: true,
        description: "Scene Default".into(),
        framework: Framework::All,
        scene_id: SceneId::Any,
        tags: SceneTags::Any,
        actions: vec![
            "vibrate".into(),
            "linear.stroke".into(),
            "constrict".into(),
            "oscillate.stroke".into(),
        ],
    })];

    default_config
}

fn get_default_actions() -> Vec<Action> {
    vec![
        Action::build(
            "vibrate",
            vec![
                Control::Scalar(
                Selector::All,
                Strength::Constant(100),
                vec![ScalarActuators::Vibrate],
            )],
        ),
        Action::build(
            "constrict",
            vec![Control::Scalar(
                Selector::All,
                Strength::Constant(100),
                vec![ScalarActuators::Constrict],
            )],
        ),
        Action::build(
            "inflate",
            vec![Control::Scalar(
                Selector::All,
                Strength::Constant(100),
                vec![ScalarActuators::Constrict],
            )],
        ),
        Action::build(
            "scalar",
            vec![Control::Scalar(
                Selector::All,
                Strength::Constant(100),
                vec![
                    ScalarActuators::Vibrate,
                    ScalarActuators::Constrict,
                    ScalarActuators::Oscillate,
                    ScalarActuators::Inflate,
                ],
            )],
        ),
        Action::build(
            "linear.stroke",
            vec![Control::Stroke(
                Selector::All,
                Strength::Constant(100),
                StrokeRange {
                    min_ms: 100,
                    max_ms: 1500,
                    min_pos: 0.0,
                    max_pos: 1.0,
                },
            )],
        ),
        Action::build(
            "oscillate.stroke",
            vec![Control::Scalar(
                Selector::All,
                Strength::Constant(100),
                vec![ScalarActuators::Oscillate],
            )],
        ),
    ]
}


fn write_file<T> (file: String, content: T) where
    T: Serialize,
    T: Clone
{
    fs::remove_file(file.clone()).unwrap();
    fs::write(file, serde_json::to_string_pretty(&content).unwrap()).unwrap();
}