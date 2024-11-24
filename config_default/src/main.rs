use std::{
    fs::{self},
    path::Path,
};

use bodies::Race;

use actions::*;
use config::*;
use dd::{dd_events, dd_variables};
use races::*;
use scenes_bp70::pb70_triggers;
use scenes_default::*;

use serde::Serialize;

use bp_scheduler::{config::client::LoggingSettings, dynamic_tracking::DynamicSettings};

mod actions;
mod dd;
mod races;
mod scenes_bp70;
mod scenes_default;

fn main() {
    fn package_dir(fomod_package: &str) -> String {
        format!("../deploy/{}/F4SE/Plugins/Telekinesis2", fomod_package)
    }

    mod_default(&package_dir("00 Default"));

    package_default_scene_bone_tracking(&package_dir("10 BoneTrackingDefaultScene"));
    package_default_scene_no_bone_tracking(&package_dir("11 NoBoneTrackingDefaultScene"));

    package_female_body_fusion_girl(&package_dir("20 FemaleBodyFusionGirls"));
    package_male_body_body_talk(&package_dir("30 MaleBodyBodyTalk"));
}

fn package_female_body_fusion_girl(config_dir: &str) {
    write_file(
        format!("{}/Races/HumanRaceFemale.json", config_dir),
        human_race_female_fusion_girl(),
    );
}

fn package_male_body_body_talk(config_dir: &str) {
    write_file(
        format!("{}/Races/HumanRaceMale.json", config_dir),
        human_race_male_body_talk(),
    );
}

fn package_default_scene_bone_tracking(config_dir: &str) {
    write_file(
        format!("{}/Triggers/Default.json", config_dir),
        default_scene_bone_tracking(),
    );
    write_file(
        format!("{}/Actions/SexActs.json", config_dir),
        sex_acts(true),
    );
}

fn package_default_scene_no_bone_tracking(config_dir: &str) {
    write_file(
        format!("{}/Triggers/Default.json", config_dir),
        default_scene_no_bone_tracking(),
    );
    write_file(
        format!("{}/Actions/SexActs.json", config_dir),
        sex_acts(false),
    );
}

fn mod_default(config_dir: &str) {
    // Actions
    for action in [
        ("Default.json", default_actions()),
        ("Devices.json", devices()),
    ] {
        let path = format!("{}/Actions/{}", config_dir, action.0);
        write_file(path, action.1);
    }

    // Races
    for body in [
        ("OtherRaces.json", ultimate_aaf_patch_races())
    ] {
        write_file(format!("{}/Races/{}", config_dir, body.0), body.1);
    }

    // Variables
    for variable in [("DD.json", dd_variables())] {
        write_file(format!("{}/Variables/{}", config_dir, variable.0), variable.1);
    }

    // Triggers
    let triggers = vec![
        ("Scenes_BP70.json", pb70_triggers()),
        ("DD.json", dd_events()),
    ];
    for trigger in triggers {
        let path = format!("{}/Triggers/{}", config_dir, trigger.0);
        write_file(path, trigger.1);
    }

    write_file(
        format!("{}/DefaultRaceFemale.json", config_dir),
        Race::default(),
    );
    write_file(
        format!("{}/DefaultRaceMale.json", config_dir),
        Race::default(),
    );
    write_file(
        format!("{}/BoneTracking.json", config_dir),
        DynamicSettings::default(),
    );
    write_file(
        format!("{}/Logging.json", config_dir),
        LoggingSettings::default(),
    );
}

fn write_file<T>(file: String, content: T)
where
    T: Serialize,
    T: Clone,
{
    let path = Path::new(&file).parent().unwrap().to_str().unwrap();
    fs::create_dir_all(path).unwrap();

    let _ = fs::remove_file(file.clone());
    fs::write(file, serde_json::to_string_pretty(&content).unwrap()).unwrap();
}
