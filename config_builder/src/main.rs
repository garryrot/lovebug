use std::{
    fs::{self},
    path::Path,
};


use actions::*;

use serde::Serialize;

use bp_scheduler::config::actions::{Action, Control, Selector, StrokeRange};

mod actions;


fn main() {
    fn get_dir(fomod_package: &str) -> String {
        format!("../deploy/{}/F4SE/Plugins/Telekinesis2", fomod_package)
    }

    pack_default(&get_dir("00 Default"));
    pack_nr(&get_dir("41 NukaRide"));
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
}

fn pack_nr(config_dir: &str) {
    write_file(
        format!("{}/Actions/NukaRide.json", config_dir),
        nr_actions(),
    );
}


pub fn nr_actions() -> Vec<Action> {
    vec![
        Action {
            name: "nr.shockchair.penetration".into(),
            control: vec![
                Control::Stroke(
                    Selector::body_parts(vec!["penis".into(), "vaginal".into()]),
                    StrokeRange {
                        min_ms: 400,
                        max_ms: 400,
                        min_pos: 0.9,
                        max_pos: 1.0,
                    },
                ),
            ],
        }
    ]
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
