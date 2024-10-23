use bp_scheduler::dynamic_tracking::{collision::Collision, DynamicSettings};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BodySet {
    pub male_body_type: String,
    pub female_body_type: String,
    pub dynamic: DynamicSettings
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BodyType {
    pub name: String,   // BodyTalk (Male Bodies), FusionGirl
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