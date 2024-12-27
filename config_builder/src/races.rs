use bp_scheduler::dynamic_tracking::collision::Collision;
use config::bodies::*;

pub static DEFAULT_COLLISION: Collision = Collision {
    outer_distance: 14.5,
    depth: 10.0,
    min_stroke: 0.25,
    error_tolerance: 0.35,
};

pub fn human_race_female_fusion_girl() -> Vec<Race> {
    vec![Race {
        form_id: 0x13746,
        name: "Fusion Girl".into(),
        sex: Sex::Female,
        genital_bone: "Pelvis_skin".into(),
        oral_bone: "HEAD".into(),
        anal_bone: "Pelvis_skin".into(),
        genital_collision: Some(Collision {
            outer_distance: 12.5,
            depth: 8.0,
            min_stroke: 0.25,
            error_tolerance: 0.35,
        }),
        oral_collision: Some(Collision {
            outer_distance: 20.0,
            depth: 12.0,
            min_stroke: 0.25,
            error_tolerance: 0.35,
        }),
    }]
}

pub fn human_race_male_body_talk() -> Vec<Race> {
    vec![Race {
        form_id: 0x13746,
        name: "Body Talk".into(),
        sex: Sex::Male,
        genital_bone: "Penis_01".into(),
        oral_bone: "HEAD".into(),
        anal_bone: "Anus_01".into(),
        genital_collision: Some(Collision {
            outer_distance: 12.5,
            depth: 8.0,
            min_stroke: 0.25,
            error_tolerance: 0.35,
        }),
        oral_collision: Some(Collision {
            outer_distance: 20.0,
            depth: 12.0,
            min_stroke: 0.25,
            error_tolerance: 0.35,
        }),
    }]
}

pub fn ultimate_aaf_patch_races() -> Vec<Race> {
    vec![Race {
        form_id: 0x1A009,
        name: "Super Mutant".into(),
        sex: Sex::Male,
        genital_bone: "Penis1".into(),
        oral_bone: "HEAD".into(),
        anal_bone: "Pelvis".into(),
        genital_collision: Some(DEFAULT_COLLISION),
        oral_collision: Some(DEFAULT_COLLISION),
    }]
}
