use std::time::Duration;

use bp_scheduler::config::actions::{ActionRef, Stren, Variable};
use config::{events::{Condition, Event, Form}, triggers::Trigger, variables::{ConfigVariable, PlayerActorValue}};

pub static VAR_DD_AROUSAL: &str = "DD_AV_Arousal";
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
    ]
}

pub fn dd_events() -> Vec<Trigger> {
    let vec = vec![
        Trigger::TimedEvent(TimedEvent {
            description: "DD Vibrators (controlled by Actor Value)".into(),
            event_start: Condition::LovebugEvent("dd.vibrator.anal".into()),
            duration: Duration::from_millis(80_000), // 65 is max, we do a bit extra
            actions: vec![ActionRef {
                action: "vibrate.anal".into(),
                strength: Stren::Variable(Variable::PlayerActorValue(
                    "DD_AV_VibrateStrengthAnal".into(),
                )),
            }],
        }),
        Trigger::TimedEvent(TimedEvent {
            description: "DD Vibrators (controlled by Actor Value)".into(),
            event_start: Condition::LovebugEvent("dd.vibrator.vaginal".into()),
            duration: Duration::from_millis(80_000), // 65 is max, we do a bit extra
            actions: vec![ActionRef {
                action: "vibrate.vaginal".into(),
                strength: Stren::Variable(Variable::PlayerActorValue(
                    "DD_AV_VibrateStrengthVaginal".into(),
                )),
            }],
        }),
        Trigger::TimedEvent(TimedEvent {
            description: "DD Vibrators (controlled by Actor Value)".into(),
            event_start: Condition::LovebugEvent("dd.vibrator".into()),
            duration: Duration::from_millis(65_000),
            actions: vec![
                ActionRef {
                    action: "vibrate".into(),
                    strength: Stren::Variable(Variable::PlayerActorValue(
                        "DD_AV_VibrateStrengthAnal".into(),
                    )),
                },
                ActionRef {
                    action: "vibrate.vaginal".into(),
                    strength: Stren::Variable(Variable::PlayerActorValue(
                        "DD_AV_VibrateStrengthVaginal".into(),
                    )),
                },
            ],
        }),
        Trigger::TimedEvent(TimedEvent {
            description: "DD Inflators (controlled by Actor Value)".into(),
            event_start: Condition::LovebugEvent("dd.inflate".into()),
            duration: Duration::from_secs(9_999_999),
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
        })
    ];
    vec
}
