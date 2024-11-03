use std::fs::{self};

use bodies::Race;

use actions::*;
use races::*;
use config::*;
use scenes_default::*;
use scenes_bp70::pb70_triggers;

use triggers::Trigger;

use serde::Serialize;

use bp_scheduler::{config::{client::ClientSettings}, dynamic_tracking::DynamicSettings};

mod actions;
mod races;
mod scenes_bp70;
mod scenes_default;

fn main() {
    let config_dir = "deploy/Data/F4SE/Plugins/Telekinesis2";

    // Triggers
    let triggers: Vec<(&str, Vec<Trigger>)> = vec![
        ("Default.json", default_trigger()),
        ("BP70.json", pb70_triggers())
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
    write_file(format!("../{}/DefaultRaceMale.json", config_dir), Race::default());
    write_file(format!("../{}/DefaultRaceFemale.json", config_dir), Race::default());
    write_file(format!("../{}/BoneTracking.json", config_dir) , DynamicSettings::default());
    write_file(format!("../{}/Connection.json", config_dir) , ClientSettings::default());
    // write_file(format!("../{}/Devices.json", config_dir) , ActuatorSettings::default());
}

fn write_file<T>(file: String, content: T)
where
    T: Serialize,
    T: Clone,
{
    let _ = fs::remove_file(file.clone());
    fs::write(file, serde_json::to_string_pretty(&content).unwrap()).unwrap();
}
