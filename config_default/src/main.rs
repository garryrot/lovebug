use std::fs::{self};

use bodies::Race;

use actions::*;
use config::*;
use events::{Event, EventTrigger, Form, StopCondition};
use races::*;
use scenes_bp70::pb70_triggers;
use scenes_default::*;

use triggers::Trigger;

use serde::Serialize;

use bp_scheduler::{
    config::{
        actions::{ActionRef, Stren, Variable},
        client::LoggingSettings,
    },
    dynamic_tracking::DynamicSettings,
};
use variables::{ConfigVariable, PlayerActorValue};

mod actions;
mod races;
mod scenes_bp70;
mod scenes_default;

pub static VAR_DD_AROUSAL: &str = "DD_AV_Arousal";
pub static VAR_DD_INFLATE_STATUS_VAGINAL: &str = "DD_AV_InflateStatusVaginal";
pub static VAR_DD_INFLATE_STATUS_ANAL: &str = "DD_AV_InflateStatusAnal";
pub static VAR_DD_VIBRATE_STRENGTH_VAGINAL: &str = "DD_AV_VibrateStrengthVaginal";
pub static VAR_DD_VIBRATE_STRENGTH_ANAL: &str = "DD_AV_VibrateStrengthAnal";

fn main() {
    let config_dir = "deploy/Data/F4SE/Plugins/Telekinesis2";

    // Actions
    let actions = vec![
        ("Default.json", default_actions()),
        ("ScalarActions.json", scalar_actions()),
        ("PenetrationActions.json", penetration_actions()),
    ];
    for action in actions {
        let path = format!("../{}/Actions/{}", config_dir, action.0);
        write_file(path, action.1);
    }

    // Races
    let bodies: Vec<(&str, Vec<bodies::Race>)> = vec![
        ("HumanRaceFemale.json", human_race_female_fusion_girl()),
        ("HumanRaceMale.json", human_race_male_body_talk()),
        ("OtherRaces.json", ultimate_aaf_patch_races()),
    ];
    for body in bodies {
        let path = format!("../{}/Races/{}", config_dir, body.0);
        write_file(path, body.1);
    }
    
    // Variables
    let variables = vec![("DD.json", dd_variables())];
    for variable in variables {
        let path = format!("../{}/Variables/{}", config_dir, variable.0);
        write_file(path, variable.1);
    }
    
    // Triggers
    let triggers: Vec<(&str, Vec<Trigger>)> = vec![
        ("Scenes.json", default_trigger()),
        ("Scenes_BP70.json", pb70_triggers()),
        ("DD.json", dd_events()),
    ];
    for trigger in triggers {
        let path = format!("../{}/Triggers/{}", config_dir, trigger.0);
        write_file(path, trigger.1);
    }

    write_file(
        format!("../{}/DefaultRaceMale.json", config_dir),
        Race::default(),
    );
    write_file(
        format!("../{}/DefaultRaceFemale.json", config_dir),
        Race::default(),
    );
    write_file(
        format!("../{}/BoneTracking.json", config_dir),
        DynamicSettings::default(),
    );
    write_file(
        format!("../{}/Logging.json", config_dir),
        LoggingSettings::default(),
    );
}

fn dd_variables() -> Vec<ConfigVariable> {
    vec![
        ConfigVariable::PlayerActorValue(PlayerActorValue {
            variable_id: VAR_DD_AROUSAL.into(),
            editor_id: VAR_DD_AROUSAL.into(),
            min: 0.0,
            max: 100.0,
        }),
        ConfigVariable::PlayerActorValue(PlayerActorValue {
            variable_id: VAR_DD_INFLATE_STATUS_VAGINAL.into(),
            editor_id: VAR_DD_INFLATE_STATUS_VAGINAL.into(),
            min: 0.0,
            max: 6.0,
        }),
        ConfigVariable::PlayerActorValue(PlayerActorValue {
            variable_id: VAR_DD_INFLATE_STATUS_ANAL.into(),
            editor_id: VAR_DD_INFLATE_STATUS_ANAL.into(),
            min: 0.0,
            max: 6.0,
        }),
        ConfigVariable::PlayerActorValue(PlayerActorValue {
            variable_id: VAR_DD_VIBRATE_STRENGTH_VAGINAL.into(),
            editor_id: VAR_DD_VIBRATE_STRENGTH_VAGINAL.into(),
            min: 0.0,
            max: 5.0,
        }),
        ConfigVariable::PlayerActorValue(PlayerActorValue {
            variable_id: VAR_DD_VIBRATE_STRENGTH_ANAL.into(),
            editor_id: VAR_DD_VIBRATE_STRENGTH_ANAL.into(),
            min: 0.0,
            max: 5.0,
        }),
    ]
}

fn dd_events() -> Vec<Trigger> {
    let vec = vec![
        Trigger::Event(Event {
            description: "DD Vibrators (controlled by Actor Value)".into(),
            event_start: EventTrigger {
                event: "dd.vibrator.anal".into(),
                form: Form::Any,
                conditions: vec![],
            },
            event_stop: StopCondition::ElapsedMs(65_000),
            actions: vec![ActionRef {
                action: "vibrate.anal".into(),
                strength: Stren::Variable(Variable::PlayerActorValue(
                    "DD_AV_VibrateStrengthAnal".into(),
                )),
            }],
        }),
        Trigger::Event(Event {
            description: "DD Vibrators (controlled by Actor Value)".into(),
            event_start: EventTrigger {
                event: "dd.vibrator.vaginal".into(),
                form: Form::Any,
                conditions: vec![],
            },
            event_stop: StopCondition::ElapsedMs(65_000),
            actions: vec![ActionRef {
                action: "vibrate.vaginal".into(),
                strength: Stren::Variable(Variable::PlayerActorValue(
                    "DD_AV_VibrateStrengthVaginal".into(),
                )),
            }],
        }),
        Trigger::Event(Event {
            description: "DD Vibrators (controlled by Actor Value)".into(),
            event_start: EventTrigger {
                event: "dd.vibrator".into(),
                form: Form::Any,
                conditions: vec![],
            },
            event_stop: StopCondition::ElapsedMs(65_000),
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
        Trigger::Event(Event {
            description: "DD Inflators (controlled by Actor Value)".into(),
            event_start: EventTrigger {
                event: "dd.inflate.vaginal".into(),
                form: Form::Any,
                conditions: vec![],
            },
            event_stop: StopCondition::Never,
            actions: vec![
                ActionRef {
                    action: "inflate".into(),
                    strength: Stren::Variable(Variable::PlayerActorValue(
                        "DD_AV_InflateStatusVaginal".into(),
                    )),
                }
            ],
        }),
        Trigger::Event(Event {
            description: "DD Inflators (controlled by Actor Value)".into(),
            event_start: EventTrigger {
                event: "dd.inflate.anal".into(),
                form: Form::Any,
                conditions: vec![],
            },
            event_stop: StopCondition::Never,
            actions: vec![
                ActionRef {
                    action: "inflate".into(),
                    strength: Stren::Variable(Variable::PlayerActorValue(
                        "DD_AV_InflateStatusAnal".into(),
                    )),
                },
            ],
        })
    ];
    vec
}

fn write_file<T>(file: String, content: T)
where
    T: Serialize,
    T: Clone,
{
    let _ = fs::remove_file(file.clone());
    fs::write(file, serde_json::to_string_pretty(&content).unwrap()).unwrap();
}
