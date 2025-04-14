use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ConfigVariable {
    PlayerActorValue(PlayerActorValue),
    PlayerKeyword(String),
    Internal(BuiltIn),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum BuiltIn {
    BoneTrackingRate,
    BoneTrackingDepth,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PlayerActorValue {
    pub editor_id: String,
    pub min: f32,
    pub max: f32,
}

impl PlayerActorValue {
    pub fn normalize(&self, val: f32) -> i64 {
        let mut x = val;
        if x < self.min {
            x = self.min;
        }
        if x > self.max {
            x = self.max;
        }
        (((x - self.min) / (self.max - self.min)) * 100.0) as i64
    }    
}