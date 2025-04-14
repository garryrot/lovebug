use bp_scheduler::config::actions::{ActionRef, Stren, Variable};
use config::{
    events::{ActorValue, ValueRange, Condition, Event},
    triggers::Trigger,
    variables::{ConfigVariable, PlayerActorValue},
};

pub static VAR_AV_VIBRATION_STRENGTH_NIPPLE: &str = "AV_VibrationStrengthNipple";
pub static VAR_AV_VIBRATION_STRENGTH_VAGINAL: &str = "AV_VibrationStrengthVaginal";

pub fn av_variables() -> Vec<ConfigVariable> {
    vec![
        ConfigVariable::PlayerActorValue(PlayerActorValue {
            editor_id: VAR_AV_VIBRATION_STRENGTH_NIPPLE.into(),
            min: 0.0,
            max: 100.0,
        }),
        ConfigVariable::PlayerActorValue(PlayerActorValue {
            editor_id: VAR_AV_VIBRATION_STRENGTH_VAGINAL.into(),
            min: 0.0,
            max: 100.0,
        }),
    ]
}

pub fn av_events() -> Vec<Trigger> {
    vec![Trigger::Event(Event {
        description: "Actual Vibrators: Bullet Vibrator Nipple".into(),
        start: vec![Condition::ActorValue(ActorValue {
            editor_id: VAR_AV_VIBRATION_STRENGTH_NIPPLE.to_owned(),
            value: ValueRange::GreaterThan(0),
        })],
        stop: vec![Condition::ActorValue(ActorValue {
            editor_id: VAR_AV_VIBRATION_STRENGTH_NIPPLE.to_owned(),
            value: ValueRange::Equals(0),
        })],
        actions: vec![ActionRef {
            action: "vibrate.nipple".into(),
            strength: Stren::Variable(Variable::PlayerActorValue(
                VAR_AV_VIBRATION_STRENGTH_NIPPLE.into(),
            )),
        }],
    })]
}
