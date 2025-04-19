use bp_scheduler::dynamic_tracking::collision::Collision;
use config::bodies::*;

pub static DEFAULT_COLLISION: Collision = Collision {
    radius: 14.5,
    depth: 10.0,
    min_penetration: 0.25,
    error_tolerance: 0.35,
};

pub fn human_race_female_fusion_girl() -> Vec<Race> {
    vec![Race {
        form_id: 0x13746,
        name: "Fusion Girl".into(),
        sex: Sex::Female,
        penetrator_bone: "Pelvis_skin".into(),
        oral_bone: "HEAD".into(),
        anal_bone: "Pelvis_skin".into(),
        anal_collision: Some(Collision {
            radius: 12.5,
            depth: 8.0,
            min_penetration: 0.25,
            error_tolerance: 0.35,
        }),
        oral_collision: Some(Collision {
            radius: 20.0,
            depth: 12.0,
            min_penetration: 0.25,
            error_tolerance: 0.35,
        }),
        penis_extra_len: 7.0,
    }]
}

pub fn human_race_male_body_talk() -> Vec<Race> {
    vec![Race {
        form_id: 0x13746,
        name: "Body Talk".into(),
        sex: Sex::Male,
        penetrator_bone: "Penis_01".into(),
        oral_bone: "HEAD".into(),
        anal_bone: "Anus_01".into(),
        anal_collision: Some(Collision {
            radius: 12.5,
            depth: 8.0,
            min_penetration: 0.25,
            error_tolerance: 0.35,
        }),
        oral_collision: Some(Collision {
            radius: 20.0,
            depth: 12.0,
            min_penetration: 0.25,
            error_tolerance: 0.35,
        }),
        penis_extra_len: 7.0,
    }]
}

pub fn ultimate_aaf_patch_races() -> Vec<Race> {
    vec![Race {
        form_id: 0x1A009,
        name: "Super Mutant".into(),
        sex: Sex::Male,
        penetrator_bone: "Penis1".into(),
        oral_bone: "HEAD".into(),
        anal_bone: "Pelvis".into(),
        anal_collision: Some(DEFAULT_COLLISION),
        oral_collision: Some(DEFAULT_COLLISION),
        penis_extra_len: 0.0,
    },
    Race {
        form_id: 0x90C33,
        name: "Molerat/FEVHound".into(),
        sex: Sex::Male,
        penetrator_bone: "RaiderDog_Penis".into(),
        oral_bone: "HEAD".into(),
        anal_bone: "Pelvis".into(),
        anal_collision: Some(DEFAULT_COLLISION),
        oral_collision: Some(DEFAULT_COLLISION),
        penis_extra_len: 27.0,
    }]
}
