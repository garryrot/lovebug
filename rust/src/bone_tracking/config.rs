use bp_scheduler::{config::util::read::{read_config_dir, read_or_default}, dynamic_tracking::StrokerSettings};
use config::bodies::Race;

pub static CONFIG_DIR: &str = "Data\\F4SE\\Plugins\\Telekinesis2";
pub static BONE_TRACKING: &str = "BoneTracking.json";
pub static RACES_DIR: &str = "Data\\F4SE\\Plugins\\Telekinesis2\\Races";
pub static DEFAULT_RACE_MALE: &str = "DefaultRaceMale.json";
pub static DEFAULT_RACE_FEMALE: &str = "DefaultRaceFemale.json";

#[derive(Debug, Clone)]
pub struct BoneTrackingSettings {
    pub stroker_settings: StrokerSettings,
    pub races: Vec<Race>,
    pub default_race_male: Option<Race>,
    pub default_race_female: Option<Race>,
    pub consider_player_passive: bool
}

pub fn read_bone_tracking_settings() -> BoneTrackingSettings {
    BoneTrackingSettings { 
        stroker_settings: read_or_default(CONFIG_DIR, BONE_TRACKING), 
        races: read_config_dir(RACES_DIR.into()), 
        default_race_male: Some(read_or_default(CONFIG_DIR, DEFAULT_RACE_MALE)), 
        default_race_female: Some(read_or_default(CONFIG_DIR, DEFAULT_RACE_FEMALE)), 
        consider_player_passive: true
    }
}
