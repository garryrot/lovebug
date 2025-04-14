
use bp_scheduler::config::actions::{ActionRef, Stren, Variable};
use config::{
    events::{ActorValue, ValueRange, Condition, Event},
    triggers::Trigger,
    variables::{ConfigVariable, PlayerActorValue},
};

pub static VAR_DD_AROUSAL: &str = "DD_AV_Arousal";
pub static VAR_DD_KW_IS_VIBRATING: &str = "DD_kw_Event_IsVibrating";
pub static VAR_DD_INFLATE_STATUS_VAGINAL: &str = "DD_AV_InflateStatusVaginal";
pub static VAR_DD_INFLATE_STATUS_ANAL: &str = "DD_AV_InflateStatusAnal";
pub static VAR_DD_VIBRATE_STRENGTH_VAGINAL: &str = "DD_AV_VibrateStrengthVaginal";
pub static VAR_DD_VIBRATE_STRENGTH_ANAL: &str = "DD_AV_VibrateStrengthAnal";

pub fn dd_variables() -> Vec<ConfigVariable> {
    vec![
        ConfigVariable::PlayerActorValue(PlayerActorValue {
            editor_id: VAR_DD_AROUSAL.into(),
            min: 0.0,
            max: 100.0,
        }),
        ConfigVariable::PlayerActorValue(PlayerActorValue {
            editor_id: VAR_DD_INFLATE_STATUS_VAGINAL.into(),
            min: 0.0,
            max: 6.0,
        }),
        ConfigVariable::PlayerActorValue(PlayerActorValue {
            editor_id: VAR_DD_INFLATE_STATUS_ANAL.into(),
            min: 0.0,
            max: 6.0,
        }),
        ConfigVariable::PlayerActorValue(PlayerActorValue {
            editor_id: VAR_DD_VIBRATE_STRENGTH_VAGINAL.into(),
            min: 0.0,
            max: 5.0,
        }),
        ConfigVariable::PlayerActorValue(PlayerActorValue {
            editor_id: VAR_DD_VIBRATE_STRENGTH_ANAL.into(),
            min: 0.0,
            max: 5.0,
        }),
        ConfigVariable::PlayerKeyword(VAR_DD_KW_IS_VIBRATING.into()),
    ]
}

pub fn dd_events() -> Vec<Trigger> {
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

    let mut vec = vec![];

    for i in 1..6 {
        vec.push(Trigger::Event(Event {
            description: format!("DD: Vibrator Vaginal {}", i),
            start: vec![
                Condition::PlayerHasKeyword(VAR_DD_KW_IS_VIBRATING.into()),
                Condition::ActorValue(ActorValue {
                    editor_id: VAR_DD_VIBRATE_STRENGTH_VAGINAL.into(),
                    value: ValueRange::Equals(i),
                }),
            ],
            stop: vec![ Condition::PlayerWithoutKeyword(VAR_DD_KW_IS_VIBRATING.into()) ],
            actions: vec![ActionRef {
                action: "vibrate.vaginal".into(),
                strength: Stren::RandomFunscript((i * 20) as i32, all_funscripts.clone()),
            }],
        }));

        vec.push(Trigger::Event(Event {
            description: format!("DD: Vibrator Anal {}", i),
            start: vec![
                Condition::PlayerHasKeyword(VAR_DD_KW_IS_VIBRATING.into()),
                Condition::ActorValue(ActorValue {
                    editor_id: VAR_DD_VIBRATE_STRENGTH_ANAL.into(),
                    value: ValueRange::Equals(i),
                }),
            ],
            stop:  vec![ Condition::PlayerWithoutKeyword(VAR_DD_KW_IS_VIBRATING.into()) ],
            actions: vec![ActionRef {
                action: "vibrate.anal".into(),
                strength: Stren::RandomFunscript((i * 20) as i32, all_funscripts.clone()),
            }],
        }));
    }

    vec.push(Trigger::Event(Event {
        description: "DD Vibrator Vaginal (Default)".into(),
        start: vec![
            Condition::PlayerHasKeyword(VAR_DD_KW_IS_VIBRATING.into()),
            Condition::ActorValue(ActorValue {
                editor_id: VAR_DD_VIBRATE_STRENGTH_VAGINAL.into(),
                value: ValueRange::Equals(0),
            }),
        ],
        stop: vec![ Condition::PlayerWithoutKeyword(VAR_DD_KW_IS_VIBRATING.into()) ],
        actions: vec![ActionRef {
            action: "vibrate.vaginal".into(),
            strength: Stren::Constant(10),
        }],
    }));

    vec.push(Trigger::Event(Event {
        description: "DD Vibrator Anal (Default)".into(),
        start: vec![
            Condition::PlayerHasKeyword(VAR_DD_KW_IS_VIBRATING.into()),
            Condition::ActorValue(ActorValue {
                editor_id: VAR_DD_VIBRATE_STRENGTH_ANAL.into(),
                value: ValueRange::Equals(0),
            }),
        ],
        stop: vec![ Condition::PlayerWithoutKeyword(VAR_DD_KW_IS_VIBRATING.into()) ],
        actions: vec![ActionRef {
            action: "vibrate.anal".into(),
            strength: Stren::Constant(10),
        }],
    }));

    vec.push(Trigger::Event(Event {
        description: "DD Inflator".into(),
        start: vec![ Condition::ControlEvent("dd.inflate".into()) ],
        stop: vec![ Condition::ControlEvent("dd.inflate.stop".into()) ],
        actions: vec![
            ActionRef {
                action: "inflate.vaginal".into(),
                strength: Stren::Variable(Variable::PlayerActorValue(
                    "DD_AV_InflateStatusVaginal".into(),
                )),
            },
            ActionRef {
                action: "inflate.anal".into(),
                strength: Stren::Variable(Variable::PlayerActorValue(
                    "DD_AV_InflateStatusAnal".into(),
                )),
            },
        ],
    }));
    vec
}
