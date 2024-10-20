use std::fs::{self};

use bp70::get_pb70_triggers;
use bp_scheduler::config::actions::*;
use config::*;
use serde::Serialize;

mod bp70;

fn main() {
    let config_dir = "deploy/Data/F4SE/Plugins/Lovebug";

    let triggers: Vec<(&str, Vec<Trigger>)> = vec![
        ("Default.json", get_default_trigger()),
        ("BP70.json", get_pb70_triggers())
    ];
    let actions = vec![
        ("Default.json", get_default_actions()),
        ("BodyParts.json", get_body_actions()),
        ("Penetration.json", get_penetration_actions()),
    ];

    for trigger in triggers {
        let path = format!("../{}/Triggers/{}", config_dir, trigger.0);
        write_file(path, trigger.1);
    }
    for action in actions {
        let path = format!("../{}/Actions/{}", config_dir, action.0);
        write_file(path, action.1);
    }
}

pub fn scene(description: &str, scene_id: SceneId, actions: Vec<ActionRef>) -> Trigger {
    Trigger::Scene(Scene {
        description: description.into(),
        scene_id,
        tags: SceneTags::Any,
        actions,
    })
}

fn get_default_trigger() -> Vec<Trigger> {
    let default_config: Vec<Trigger> = vec![Trigger::Scene(Scene {
        description: "Scene Default".into(),
        scene_id: SceneId::Any,
        tags: SceneTags::Any,
        actions: vec![
            ActionRef::new(
                "vibrate",
                Strength::RandomFunscript(50, vec!["Blowjob".into(), "Deepthroat".into()]),
            ),
            ActionRef::new("linear", Strength::Constant(50)),
            ActionRef::new("constrict", Strength::Constant(50)),
            ActionRef::new("oscillate", Strength::Constant(50)),
        ],
    })];

    /*
           Nipple,
           Penis,
           Vagina,
           Anal
    */

    default_config
}


fn get_default_actions() -> Vec<Action> {
    vec![
        Action::new(
            "vibrate",
            vec![Control::Scalar(
                Selector::All,
                vec![ScalarActuator::Vibrate],
            )],
        ),
        Action::new(
            "constrict",
            vec![Control::Scalar(
                Selector::All,
                vec![ScalarActuator::Constrict],
            )],
        ),
        Action::new(
            "inflate",
            vec![Control::Scalar(
                Selector::All,
                vec![ScalarActuator::Inflate],
            )],
        ),
        Action::new(
            "scalar",
            vec![Control::Scalar(
                Selector::All,
                vec![
                    ScalarActuator::Vibrate,
                    ScalarActuator::Constrict,
                    ScalarActuator::Oscillate,
                    ScalarActuator::Inflate,
                ],
            )],
        ),
        Action::new(
            "linear",
            vec![Control::Stroke(
                Selector::All,
                StrokeRange {
                    min_ms: 100,
                    max_ms: 1500,
                    min_pos: 0.0,
                    max_pos: 1.0,
                },
            )],
        ),
        Action::new(
            "oscillate",
            vec![Control::Scalar(
                Selector::All,
                vec![ScalarActuator::Oscillate],
            )],
        ),
    ]
}

fn get_penetration_actions() -> Vec<Action> {
    vec![
        Action::new(
            "cunnilungus",
            vec![Control::Scalar(
                Selector::BodyParts(vec!["vagina".into()]),
                vec![ScalarActuator::Vibrate, ScalarActuator::Constrict],
            )],
        ),
        Action::new(
            "masturbation",
            vec![
                Control::Scalar(
                    Selector::BodyParts(vec!["penis".into()]),
                    vec![
                        ScalarActuator::Vibrate,
                        ScalarActuator::Constrict,
                        ScalarActuator::Oscillate,
                    ],
                ),
                Control::Stroke(
                    Selector::BodyParts(vec!["penis".into()]),
                    StrokeRange {
                        min_ms: 150,
                        max_ms: 1500,
                        min_pos: 0.0,
                        max_pos: 0.5,
                    },
                ),
            ],
        ),
        Action::new(
            "footjob",
            vec![
                Control::Scalar(
                    Selector::BodyParts(vec!["penis".into()]),
                    vec![ScalarActuator::Vibrate, ScalarActuator::Oscillate],
                ),
                Control::Stroke(
                    Selector::BodyParts(vec!["penis".into()]),
                    StrokeRange {
                        min_ms: 150,
                        max_ms: 1500,
                        min_pos: 0.0,
                        max_pos: 0.5,
                    },
                ),
            ],
        ),
        Action::new(
            "penetration.vaginal",
            vec![
                Control::Scalar(
                    Selector::BodyParts(vec!["penis".into(), "vagina".into()]),
                    vec![
                        ScalarActuator::Vibrate,
                        ScalarActuator::Constrict,
                        ScalarActuator::Oscillate,
                    ],
                ),
                Control::Stroke(
                    Selector::BodyParts(vec!["penis".into(), "vagina".into()]),
                    StrokeRange {
                        min_ms: 150,
                        max_ms: 1500,
                        min_pos: 0.0,
                        max_pos: 0.5,
                    },
                ),
            ],
        ),
        Action::new(
            "penetration.vaginal.deep",
            vec![
                Control::Scalar(
                    Selector::BodyParts(vec!["penis".into(), "vagina".into()]),
                    vec![
                        ScalarActuator::Vibrate,
                        ScalarActuator::Constrict,
                        ScalarActuator::Oscillate,
                    ],
                ),
                Control::Stroke(
                    Selector::BodyParts(vec!["penis".into(), "vagina".into()]),
                    StrokeRange {
                        min_ms: 150,
                        max_ms: 1500,
                        min_pos: 0.0,
                        max_pos: 1.0,
                    },
                ),
            ],
        ),
        Action::new(
            "penetration.oral.shallow",
            vec![
                Control::Scalar(
                    Selector::BodyParts(vec!["penis".into(), "oral".into()]),
                    vec![
                        ScalarActuator::Vibrate,
                        ScalarActuator::Constrict,
                        ScalarActuator::Oscillate,
                    ],
                ),
                Control::Stroke(
                    Selector::BodyParts(vec!["penis".into(), "oral".into()]),
                    StrokeRange {
                        min_ms: 150,
                        max_ms: 1500,
                        min_pos: 0.0,
                        max_pos: 0.3,
                    },
                ),
            ],
        ),
        Action::new(
            "penetration.oral",
            vec![
                Control::Scalar(
                    Selector::BodyParts(vec!["penis".into(), "oral".into()]),
                    vec![
                        ScalarActuator::Vibrate,
                        ScalarActuator::Constrict,
                        ScalarActuator::Oscillate,
                    ],
                ),
                Control::Stroke(
                    Selector::BodyParts(vec!["penis".into(), "oral".into()]),
                    StrokeRange {
                        min_ms: 150,
                        max_ms: 1500,
                        min_pos: 0.0,
                        max_pos: 0.5,
                    },
                ),
            ],
        ),
        Action::new(
            "penetration.oral.deep",
            vec![
                Control::Scalar(
                    Selector::BodyParts(vec!["penis".into(), "oral".into()]),
                    vec![
                        ScalarActuator::Vibrate,
                        ScalarActuator::Constrict,
                        ScalarActuator::Oscillate,
                    ],
                ),
                Control::Stroke(
                    Selector::BodyParts(vec!["penis".into(), "oral".into()]),
                    StrokeRange {
                        min_ms: 150,
                        max_ms: 1500,
                        min_pos: 0.0,
                        max_pos: 1.0,
                    },
                ),
            ],
        ),
        Action::new(
            "penetration.anal",
            vec![
                Control::Scalar(
                    Selector::BodyParts(vec!["penis".into(), "oral".into()]),
                    vec![
                        ScalarActuator::Vibrate,
                        ScalarActuator::Constrict,
                        ScalarActuator::Oscillate,
                    ],
                ),
                Control::Stroke(
                    Selector::BodyParts(vec!["penis".into(), "oral".into()]),
                    StrokeRange {
                        min_ms: 150,
                        max_ms: 1500,
                        min_pos: 0.0,
                        max_pos: 0.5,
                    },
                ),
            ],
        ),
        Action::new(
            "penetration.anal.deep",
            vec![
                Control::Scalar(
                    Selector::BodyParts(vec!["penis".into(), "oral".into()]),
                    vec![
                        ScalarActuator::Vibrate,
                        ScalarActuator::Constrict,
                        ScalarActuator::Oscillate,
                    ],
                ),
                Control::Stroke(
                    Selector::BodyParts(vec!["penis".into(), "oral".into()]),
                    StrokeRange {
                        min_ms: 150,
                        max_ms: 1500,
                        min_pos: 0.0,
                        max_pos: 1.0,
                    },
                ),
            ],
        ),
    ]
}

fn get_body_actions() -> Vec<Action> {
    vec![
        Action::new(
            "vibrate.nipple",
            vec![Control::Scalar(
                Selector::BodyParts(vec!["nipple".into()]),
                vec![ScalarActuator::Vibrate],
            )],
        ),
        Action::new(
            "vibrate.vagina",
            vec![Control::Scalar(
                Selector::BodyParts(vec!["vagina".into()]),
                vec![ScalarActuator::Vibrate],
            )],
        ),
        Action::new(
            "vibrate.clitoral",
            vec![Control::Scalar(
                Selector::BodyParts(vec!["clitoral".into()]),
                vec![ScalarActuator::Vibrate],
            )],
        ),
        Action::new(
            "vibrate.anal",
            vec![Control::Scalar(
                Selector::BodyParts(vec!["anal".into()]),
                vec![ScalarActuator::Vibrate],
            )],
        ),
        Action::new(
            "constrict.penis",
            vec![Control::Scalar(
                Selector::BodyParts(vec!["constrict".into()]),
                vec![ScalarActuator::Inflate],
            )],
        ),
        Action::new(
            "constrict.vagina",
            vec![Control::Scalar(
                Selector::BodyParts(vec!["constrict".into()]),
                vec![ScalarActuator::Inflate],
            )],
        ),
        Action::new(
            "constrict.anal",
            vec![Control::Scalar(
                Selector::BodyParts(vec!["constrict".into()]),
                vec![ScalarActuator::Inflate],
            )],
        ),
        Action::new(
            "inflate.vagina",
            vec![Control::Scalar(
                Selector::BodyParts(vec!["vagina".into()]),
                vec![ScalarActuator::Inflate],
            )],
        ),
        Action::new(
            "inflate.anal",
            vec![Control::Scalar(
                Selector::BodyParts(vec!["anal".into()]),
                vec![ScalarActuator::Inflate],
            )],
        ),
    ]
}

fn write_file<T>(file: String, content: T)
where
    T: Serialize,
    T: Clone,
{
    let _ = fs::remove_file(file.clone());
    fs::write(file, serde_json::to_string_pretty(&content).unwrap()).unwrap();
}
