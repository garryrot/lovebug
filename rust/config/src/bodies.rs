use bp_scheduler::dynamic_tracking::collision::Collision;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum Sex {
    Male,
    Female,
    None
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Race {
    pub form_id: u32,
    pub name: String,
    pub sex: Sex,
    pub penetrator_bone: String,
    pub strapon_extra_length: f32, // hacky but whatever
    pub oral_bone: String,
    pub anal_bone: String,
    pub anal_collision: Option<Collision>,
    pub oral_collision: Option<Collision>
}

impl Default for Race {
    fn default() -> Self {
        let collision = Collision {
            outer_distance: 14.5,
            depth: 10.0,
            min_stroke: 0.25,
            error_tolerance: 0.35,
        };
        Race {
            form_id: 0,
            name: "Default Race".into(),
            sex: Sex::Female,
            penetrator_bone: "Pelvis".into(),
            oral_bone: "HEAD".into(),
            anal_bone: "Pelvis".into(),
            anal_collision: Some(collision),
            oral_collision: Some(collision),
            strapon_extra_length: 8.0,
        }
    }
}

impl Race {
    pub fn female_default() -> Race {
        let collision = Collision {
            outer_distance: 14.5,
            depth: 10.0,
            min_stroke: 0.25,
            error_tolerance: 0.35,
        };
        Race {
            form_id: 0,
            name: "Default Female".into(),
            sex: Sex::Female,
            penetrator_bone: "Pelvis".into(),
            oral_bone: "HEAD".into(),
            anal_bone: "Pelvis".into(),
            anal_collision: Some(collision),
            oral_collision: Some(collision),
            strapon_extra_length: 8.0,
        }
    }

    pub fn male_default() -> Race {
        let collision = Collision {
            outer_distance: 14.5,
            depth: 10.0,
            min_stroke: 0.25,
            error_tolerance: 0.35,
        };
        Race {
            form_id: 0,
            name: "Default Male".into(),
            sex: Sex::Male,
            penetrator_bone: "Pelvis".into(),
            oral_bone: "HEAD".into(),
            anal_bone: "Pelvis".into(),
            anal_collision: Some(collision),
            oral_collision: Some(collision),
            strapon_extra_length: 8.0,
        }
    }
}
