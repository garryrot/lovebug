use bp_scheduler::dynamic_tracking::collision::Collision;
use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Race {
    pub form_id: u32,
    pub name: String,
    pub male: Option<BodyType>,
    pub female: Option<BodyType>,
}

impl Default for Race {
    fn default() -> Self {
        let col = Collision {
            outer_distance: 14.5,
            depth: 10.0,
            min_stroke: 0.25,
            error_tolerance: 0.35,
        };
        Race {
            form_id: 0,
            name: "Default Race".into(),
            male: Some(BodyType {
                name: "Default Male".into(),
                is_female: false,
                genital_bone: Bone {
                    name: "Pelvis".into(),
                    collision: Some(col),
                },
                oral_bone: Bone {
                    name: "HEAD".into(),
                    collision: Some(col),
                },
                anal_bone: Bone {
                    name: "Pelvis".into(),
                    collision: Some(col),
                },
            }),
            female: Some(BodyType {
                name: "Default Female".into(),
                is_female: false,
                genital_bone: Bone {
                    name: "Pelvis".into(),
                    collision: Some(col),
                },
                oral_bone: Bone {
                    name: "HEAD".into(),
                    collision: Some(col),
                },
                anal_bone: Bone {
                    name: "Pelvis".into(),
                    collision: Some(col),
                },
            }),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BodyType {
    pub name: String,
    pub is_female: bool,
    pub genital_bone: Bone,
    pub oral_bone: Bone,
    pub anal_bone: Bone
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Bone {
    pub name: String,
    pub collision: Option<Collision>
}
