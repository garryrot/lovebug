use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ConfigVariable {
    PlayerActorValue(PlayerActorValue),
    Internal(BuiltIn)
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum BuiltIn {
    BoneTrackingRate,
    BoneTrackingDepth
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PlayerActorValue {
    pub variable_id: String,
    pub editor_id: String,
    pub min: f32,
    pub max: f32
}
