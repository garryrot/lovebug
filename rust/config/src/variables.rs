use serde::{Deserialize, Serialize};

use bp_scheduler::dynamic_tracking::DynamicTrackingHandle;
use tracing::debug;

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
    pub editor_id: String,
    pub min: f32,
    pub max: f32
}

use std::{
    collections::HashMap,
    sync::{
        atomic::{AtomicI64, Ordering},
        Arc,
    },
};

#[derive(Clone, Debug)]
pub struct VariableStore {
                      // form_id -> (Editor_Name, StoredValue, MinMax)
    variables: HashMap<i64, (String, Arc<AtomicI64>, ConfigVariable)>,
}

impl VariableStore {
    pub fn new(
        variables: Vec<(ConfigVariable, i64, f32)>,
        tracking_handles: &DynamicTrackingHandle,
    ) -> Self {
        let mut hashmap = HashMap::new();
        for tuple in variables {
            if let ConfigVariable::PlayerActorValue(val) = &tuple.0 {
                hashmap.insert(
                    tuple.1 as i64,
                    (
                        val.editor_id.clone(),
                        Arc::new(AtomicI64::new(normalize(val.min, val.max, tuple.2))),
                        tuple.0.clone(),
                    ),
                );
            }
        }
        hashmap.insert(
            -1,
            (
                "BoneTrackingRate".into(),
                tracking_handles.cur_avg_ms.clone(),
                ConfigVariable::Internal(BuiltIn::BoneTrackingRate),
            ),
        );
        hashmap.insert(
            -2,
            (
                "BoneTrackingDepth".into(),
                tracking_handles.cur_avg_depth.clone(),
                ConfigVariable::Internal(BuiltIn::BoneTrackingDepth),
            ),
        );
        VariableStore { variables: hashmap }
    }

    pub fn update(&self, form_id: u32, value: f32) -> Option<String> {
        if let Some(entry) = self.variables.get(&(form_id as i64)) {
            if let ConfigVariable::PlayerActorValue(val) = &entry.2 {
                let new_val = normalize(val.min, val.max, value);
                debug!(new_val, val.editor_id, "monitored actor value changed");
                entry.1.store(new_val, Ordering::Relaxed);
                return Some(val.editor_id.clone());
            }
        }
        None
    }

    pub fn get(&self, name: &str) -> Option<Arc<AtomicI64>> {
        for variable in &self.variables {
            if variable.1 .0 == name {
                return Some(variable.1 .1.clone());
            }
        }
        None
    }
}

pub fn normalize(min: f32, max: f32, val: f32) -> i64 {
    let mut x = val;
    if x < min {
        x = min;
    }
    if x > max {
        x = max;
    }
    (((x - min) / (max - min)) * 100.0) as i64
}
