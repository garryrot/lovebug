use bp_scheduler::config::actions::*;
use config::body_parts::*;

pub fn default_actions() -> Vec<Action> {
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
        Action::new_with_bone(
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

pub fn scalar_actions() -> Vec<Action> {
    vec![
        Action::new(
            "vibrate.nipple",
            vec![Control::Scalar(
                Selector::BodyParts(vec![TAG_NIPPLE.into()]),
                vec![ScalarActuator::Vibrate],
            )],
        ),
        Action::new(
            "vibrate.vaginal",
            vec![Control::Scalar(
                Selector::BodyParts(vec![TAG_VAGINAL.into()]),
                vec![ScalarActuator::Vibrate],
            )],
        ),
        Action::new(
            "vibrate.clitoral",
            vec![Control::Scalar(
                Selector::BodyParts(vec![TAG_CLIT.into()]),
                vec![ScalarActuator::Vibrate],
            )],
        ),
        Action::new(
            "vibrate.anal",
            vec![Control::Scalar(
                Selector::BodyParts(vec![TAG_ANAL.into()]),
                vec![ScalarActuator::Vibrate],
            )],
        ),
        Action::new(
            "constrict.penis",
            vec![Control::Scalar(
                Selector::BodyParts(vec![TAG_PENIS.into()]),
                vec![ScalarActuator::Inflate],
            )],
        ),
        Action::new(
            "constrict.vaginal",
            vec![Control::Scalar(
                Selector::BodyParts(vec![TAG_VAGINAL.into()]),
                vec![ScalarActuator::Inflate],
            )],
        ),
        Action::new(
            "constrict.anal",
            vec![Control::Scalar(
                Selector::BodyParts(vec![TAG_ANAL.into()]),
                vec![ScalarActuator::Inflate],
            )],
        ),
        Action::new(
            "inflate.vaginal",
            vec![Control::Scalar(
                Selector::BodyParts(vec![TAG_VAGINAL.into()]),
                vec![ScalarActuator::Inflate],
            )],
        ),
        Action::new(
            "inflate.anal",
            vec![Control::Scalar(
                Selector::BodyParts(vec![TAG_ANAL.into()]),
                vec![ScalarActuator::Inflate],
            )],
        ),
    ]
}

pub fn penetration_actions() -> Vec<Action> {
    vec![
        Action {
            name: "cunnilungus".into(),
            do_bone_tracking: false,
            control: vec![Control::Scalar(
                Selector::BodyParts(vec![TAG_VAGINAL.into()]),
                vec![ScalarActuator::Vibrate, ScalarActuator::Constrict],
            )],
        },
        Action {
            name: "masturbation".into(),
            control: vec![
                Control::Scalar(
                    Selector::BodyParts(vec![TAG_PENIS.into()]),
                    vec![
                        ScalarActuator::Vibrate,
                        ScalarActuator::Constrict,
                        ScalarActuator::Oscillate,
                    ],
                ),
                Control::Stroke(
                    Selector::BodyParts(vec![TAG_PENIS.into()]),
                    StrokeRange {
                        min_ms: 150,
                        max_ms: 1500,
                        min_pos: 0.0,
                        max_pos: 0.5,
                    },
                ),
            ],
            do_bone_tracking: false
        },
        Action {
            name: "footjob".into(),
            control: vec![
                Control::Scalar(
                    Selector::BodyParts(vec![TAG_PENIS.into()]),
                    vec![ScalarActuator::Vibrate, ScalarActuator::Oscillate],
                ),
                Control::Stroke(
                    Selector::BodyParts(vec![TAG_PENIS.into()]),
                    StrokeRange {
                        min_ms: 150,
                        max_ms: 1500,
                        min_pos: 0.0,
                        max_pos: 0.5,
                    },
                ),
            ],
            do_bone_tracking: false
        },
        Action::new_with_bone(
            "penetration.vaginal",
            vec![
                Control::Scalar(
                    Selector::BodyParts(vec![TAG_PENIS.into(), TAG_VAGINAL.into()]),
                    vec![
                        ScalarActuator::Vibrate,
                        ScalarActuator::Constrict,
                        ScalarActuator::Oscillate,
                    ],
                ),
                Control::Stroke(
                    Selector::BodyParts(vec![TAG_PENIS.into(), TAG_VAGINAL.into()]),
                    StrokeRange {
                        min_ms: 150,
                        max_ms: 1500,
                        min_pos: 0.0,
                        max_pos: 0.5,
                    },
                ),
            ],
        ),
        Action::new_with_bone(
            "penetration.vaginal.deep",
            vec![
                Control::Scalar(
                    Selector::BodyParts(vec![TAG_PENIS.into(), TAG_VAGINAL.into()]),
                    vec![
                        ScalarActuator::Vibrate,
                        ScalarActuator::Constrict,
                        ScalarActuator::Oscillate,
                    ],
                ),
                Control::Stroke(
                    Selector::BodyParts(vec![TAG_PENIS.into(), TAG_VAGINAL.into()]),
                    StrokeRange {
                        min_ms: 150,
                        max_ms: 1500,
                        min_pos: 0.0,
                        max_pos: 1.0,
                    },
                ),
            ],
        ),
        Action::new_with_bone(
            "penetration.oral.shallow",
            vec![
                Control::Scalar(
                    Selector::BodyParts(vec![TAG_PENIS.into(), TAG_ORAL.into()]),
                    vec![
                        ScalarActuator::Vibrate,
                        ScalarActuator::Constrict,
                        ScalarActuator::Oscillate,
                    ],
                ),
                Control::Stroke(
                    Selector::BodyParts(vec![TAG_PENIS.into(), TAG_ORAL.into()]),
                    StrokeRange {
                        min_ms: 150,
                        max_ms: 1500,
                        min_pos: 0.0,
                        max_pos: 0.3,
                    },
                ),
            ],
        ),
        Action::new_with_bone(
            "penetration.oral",
            vec![
                Control::Scalar(
                    Selector::BodyParts(vec![TAG_PENIS.into(), TAG_ORAL.into()]),
                    vec![
                        ScalarActuator::Vibrate,
                        ScalarActuator::Constrict,
                        ScalarActuator::Oscillate,
                    ],
                ),
                Control::Stroke(
                    Selector::BodyParts(vec![TAG_PENIS.into(), TAG_ORAL.into()]),
                    StrokeRange {
                        min_ms: 150,
                        max_ms: 1500,
                        min_pos: 0.0,
                        max_pos: 0.5,
                    },
                ),
            ],
        ),
        Action::new_with_bone(
            "penetration.oral.deep",
            vec![
                Control::Scalar(
                    Selector::BodyParts(vec![TAG_PENIS.into(), TAG_ORAL.into()]),
                    vec![
                        ScalarActuator::Vibrate,
                        ScalarActuator::Constrict,
                        ScalarActuator::Oscillate,
                    ],
                ),
                Control::Stroke(
                    Selector::BodyParts(vec![TAG_PENIS.into(), TAG_ORAL.into()]),
                    StrokeRange {
                        min_ms: 150,
                        max_ms: 1500,
                        min_pos: 0.0,
                        max_pos: 1.0,
                    },
                ),
            ],
        ),
        Action::new_with_bone(
            "penetration.anal",
            vec![
                Control::Scalar(
                    Selector::BodyParts(vec![TAG_PENIS.into(), TAG_ORAL.into()]),
                    vec![
                        ScalarActuator::Vibrate,
                        ScalarActuator::Constrict,
                        ScalarActuator::Oscillate,
                    ],
                ),
                Control::Stroke(
                    Selector::BodyParts(vec![TAG_PENIS.into(), TAG_ORAL.into()]),
                    StrokeRange {
                        min_ms: 150,
                        max_ms: 1500,
                        min_pos: 0.0,
                        max_pos: 0.5,
                    },
                ),
            ],
        ),
        Action::new_with_bone(
            "penetration.anal.deep",
            vec![
                Control::Scalar(
                    Selector::BodyParts(vec![TAG_PENIS.into(), TAG_ORAL.into()]),
                    vec![
                        ScalarActuator::Vibrate,
                        ScalarActuator::Constrict,
                        ScalarActuator::Oscillate,
                    ],
                ),
                Control::Stroke(
                    Selector::BodyParts(vec![TAG_PENIS.into(), TAG_ORAL.into()]),
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