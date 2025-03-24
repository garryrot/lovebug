use bp_scheduler::config::actions::{ActionRef, Stren, Variable};
use config::{
    events::{ActorValueChange, Comparison, Condition, Event}, triggers::Trigger, variables::{ConfigVariable, PlayerActorValue}
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
        event_start: Condition::ActorValue(ActorValueChange { 
            variable_id: VAR_AV_VIBRATION_STRENGTH_NIPPLE.to_owned(), 
            condition: Comparison::GreaterThan(0)
        }),
        event_stop: Condition::ActorValue(ActorValueChange { 
            variable_id: VAR_AV_VIBRATION_STRENGTH_NIPPLE.to_owned(), 
            condition: Comparison::Eq(0) 
        }),
        actions: vec![ActionRef {
            action: "vibrate.nipple".into(),
            strength: Stren::Variable(Variable::PlayerActorValue(
                VAR_AV_VIBRATION_STRENGTH_NIPPLE.into(),
            )),
        }],
    })]
}
