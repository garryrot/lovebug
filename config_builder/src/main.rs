use std::{
    fs::{self},
    path::Path,
};

use av::{av_events, av_variables};
use bodies::Race;

use actions::*;
use config::*;
use dd::{dd_events, dd_variables};
use nuka_ride::{nr_actions, nr_events};
use races::*;
use scenes_bp70::pb70_triggers;
use scenes_default::*;

use serde::Serialize;

use bp_scheduler::{config::client::LoggingSettings, dynamic_tracking::StrokerSettings};

mod av;
mod actions;
mod dd;
mod nuka_ride;
mod races;
mod scenes_bp70;
mod scenes_default;

fn main() {
    fn get_dir(fomod_package: &str) -> String {
        format!("../deploy/{}/F4SE/Plugins/Telekinesis2", fomod_package)
    }

    pack_default(&get_dir("00 Default"));
    pack_nr(&get_dir("41 NukaRide"));
    pack_dd(&get_dir("42 DD"));
    pack_bp70(&get_dir("51 AAF BP70"));

    pack_fusion_girl(&get_dir("20 FemaleBodyFusionGirls"));
    pack_body_talk(&get_dir("30 MaleBodyBodyTalk"));
}

fn pack_default(config_dir: &str) {
    for action in [
        ("Default.json", default_actions()),
        ("BodyParts.json", body_part_actions()),
        ("SexActs.json", sex_acts()),
    ] {
        let path = format!("{}/Actions/{}", config_dir, action.0);
        write_file(path, action.1);
    }

    for body in [("OtherRaces.json", ultimate_aaf_patch_races())] {
        write_file(format!("{}/Races/{}", config_dir, body.0), body.1);
    }

    let triggers = vec![("Default.json", default_scene_bone_tracking())];
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
        StrokerSettings::default(),
    );
    write_file(
        format!("{}/Logging.json", config_dir),
        LoggingSettings::default(),
    );
}

fn pack_nr(config_dir: &str) {
    write_file(
        format!("{}/Actions/NukaRide.json", config_dir),
        nr_actions(),
    );
    write_file(
        format!("{}/Triggers/NukaRide.json", config_dir),
        nr_events(),
    );
}

fn pack_dd(config_dir: &str) {
    write_file(format!("{}/Triggers/DD.json", config_dir), dd_events());
    write_file(format!("{}/Variables/DD.json", config_dir), dd_variables());

    write_file(format!("{}/Variables/AV.json", config_dir), av_variables());
    write_file(format!("{}/Triggers/AV.json", config_dir), av_events());
}

fn pack_bp70(config_dir: &str) {
    write_file(
        format!("{}/Triggers/Scenes_BP70.json", config_dir),
        pb70_triggers(),
    );
}

fn pack_fusion_girl(config_dir: &str) {
    write_file(
        format!("{}/Races/HumanRaceFemale.json", config_dir),
        human_race_female_fusion_girl(),
    );
}

fn pack_body_talk(config_dir: &str) {
    write_file(
        format!("{}/Races/HumanRaceMale.json", config_dir),
        human_race_male_body_talk(),
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
