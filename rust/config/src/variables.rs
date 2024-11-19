
#[derive(Clone, Debug)]
pub enum ConfigVariable {
    PlayerActorValue(PlayerActorValue),
    Internal(BuiltIn)
}

#[derive(Clone, Debug)]
pub enum BuiltIn {
    BoneTrackingRate,
    BoneTrackingDepth
}

#[derive(Clone, Debug)]
pub struct PlayerActorValue {
    pub variable_id: String,
    pub editor_id: String,
    pub min: f32,
    pub max: f32
}
