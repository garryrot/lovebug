use bp_scheduler::config::actions::{Action, ActionRef, Control, Selector, Stren, StrokeRange};
use config::{body_parts::{TAG_PENIS, TAG_VAGINAL}, triggers::Trigger, SceneId};

use crate::scene;

pub fn nr_events() -> Vec<Trigger> {
    vec![
        scene(
            "Nuka Ride: Mule Anal Insertion",
            SceneId::Exact("NR_Mule_Clothed_PT".into()),
            vec![ActionRef {
                action: "penetration.anal.deep".into(),
                strength: Stren::Funscript(80, "Square".into()),
            }],
        ),
        scene(
            "Nuka Ride: Institute Shocking Device",
            SceneId::Exact("NR_Institute_PT".into()),
            vec![
                ActionRef::new("vibrate", Stren::Funscript(100, "Sawtooth-Fast".into())),
                ActionRef::new("constrict", Stren::Funscript(40, "On-Off".into())),
                ActionRef::new("nr.shockchair.penetration", Stren::Constant(100)),
            ],
        ),
        scene(
            "Nuka Ride: FMM Human Couch 2",
            SceneId::Exact("NR_FMM_Human_Couch02_PT".into()),
            vec![ActionRef::new("masturbation", Stren::Constant(40))],
        ),
    ]
}

pub fn nr_actions() -> Vec<Action> {
    vec![
        Action {
            name: "nr.shockchair.penetration".into(),
            do_bone_tracking: false,
            control: vec![
                Control::Stroke(
                    Selector::BodyParts(vec![TAG_PENIS.into(), TAG_VAGINAL.into()]),
                    StrokeRange {
                        min_ms: 400,
                        max_ms: 400,
                        min_pos: 0.9,
                        max_pos: 1.0,
                    },
                ),
            ],
    }]
}