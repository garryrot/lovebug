use bp_scheduler::dynamic_tracking::collision::Collision;
use config::bodies::*;

pub static DEFAULT_COLLISION: Collision = Collision {
    outer_distance: 14.5,
    depth: 10.0,
    min_stroke: 0.25,
    error_tolerance: 0.35,
};

pub fn human_race() -> Vec<Race> {
    vec![Race {
        form_id: 0x13746,
        name: "Race Human".into(),
        male: Some(BodyType {
            name: "Body Talk".into(),
            is_female: false,
            genital_bone: Bone {
                name: "Penis_01".into(),
                collision: Some(DEFAULT_COLLISION),
            },
            oral_bone: Bone {
                name: "HEAD".into(),
                collision: None,
            },
            anal_bone: Bone {
                name: "Anus_01".into(),
                collision: None,
            },
        }),
        female: Some(BodyType {
            name: "Fusion Girl".into(),
            is_female: true,
            genital_bone: Bone {
                name: "Pelvis_skin".into(),
                collision: None,
            },
            oral_bone: Bone {
                name: "HEAD".into(),
                collision: Some(Collision {
                    outer_distance: 25.0,
                    depth: 12.0,
                    min_stroke: 0.25,
                    error_tolerance: 0.35,
                }),
            },
            anal_bone: Bone {
                name: "Pelvis_skin".into(),
                collision: None,
            },
        }),
    }]
}

pub fn other_races() -> Vec<Race> {
    vec![Race {
        form_id: 0x1A009,
        name: "Super Mutant".into(),
        male: Some(BodyType {
            name: "Super Mutant".into(),
            is_female: false,
            genital_bone: Bone {
                name: "Penis1".into(),
                collision: Some(Collision {
                    outer_distance: 14.5,
                    depth: 10.0,
                    min_stroke: 0.25,
                    error_tolerance: 0.35,
                }),
            },
            oral_bone: Bone {
                name: "HEAD".into(),
                collision: None,
            },
            anal_bone: Bone {
                name: "Pelvis".into(),
                collision: None,
            },
        }),
        female: None,
    }]
}

