use std::{
    fs::{self},
    time::Duration,
};

use bodies::Race;

use actions::*;
use config::*;
use events::{Event, EventTrigger, Form, TimedEvent};
use races::*;
use scenes_bp70::pb70_triggers;
use scenes_default::*;

use triggers::Trigger;

use serde::Serialize;

use bp_scheduler::{
    config::{
        actions::{ActionRef, Stren, Variable},
        client::ClientSettings,
    },
    dynamic_tracking::DynamicSettings,
};
use variables::{ConfigVariable, PlayerActorValue};

mod actions;
mod races;
mod scenes_bp70;
mod scenes_default;

fn main() {
    let config_dir = "deploy/Data/F4SE/Plugins/Telekinesis2";

    // Variables
    let variables = vec![("DD.json", dd_variables())];

    // Triggers
    let triggers: Vec<(&str, Vec<Trigger>)> = vec![
        ("Scenes.json", default_trigger()),
        ("Scenes_BP70.json", pb70_triggers()),
        ("Events_DD.json", dd_events()),
    ];
    for trigger in triggers {
        let path = format!("../{}/Triggers/{}", config_dir, trigger.0);
        write_file(path, trigger.1);
    }

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
        format!("../{}/Connection.json", config_dir),
        ClientSettings::default(),
    );
}

fn dd_variables() -> Vec<ConfigVariable> {
    vec![
        // TODO
    ]
}

fn dd_events() -> Vec<Trigger> {
    let vec = vec![
        Trigger::Event(Event {
            description: "DD Vibrators (controlled by Actor Value)".into(),
            event_start: EventTrigger {
                event: "DD.Vibrator".into(),
                form: Form::Any,
                conditions: vec![],
            },
            event_stop: EventTrigger {
                event: "never".into(),
                form: Form::Any,
                conditions: vec![],
            },
            actions: vec![
                ActionRef {
                    action: "vibrate.anal".into(),
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
                event: "DD.Inflator".into(),
                form: Form::Any,
                conditions: vec![],
            },
            event_stop: EventTrigger {
                event: "never".into(),
                form: Form::Any,
                conditions: vec![],
            },
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
        }),
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
