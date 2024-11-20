use std::{
    collections::HashMap,
    sync::{
        atomic::{AtomicI64, Ordering},
        Arc,
    },
};

use config::variables::{BuiltIn, ConfigVariable};
use tracing::{debug, error, info};

use crate::{bones::DynamicTrackingHandle, bridge::ffi_bridge::{GetFormID, GetPlayerActorValue, TESForm_GetFormByEditorID}, Telekinesis};

pub trait VariableSource {
    fn read(&self, tk: &mut Telekinesis) -> Option<i64>;
}

#[derive(Clone, Debug)]
pub struct VariableStore {
    variables: HashMap<i64, (String, Arc<AtomicI64>, ConfigVariable)>,
}

impl VariableStore {
    pub fn new(variables: Vec<ConfigVariable>, tracking_handles: &DynamicTrackingHandle) -> Self {
        let mut hashmap = HashMap::new();
        for variable in variables {
            if let ConfigVariable::PlayerActorValue(val) = &variable {
                let form_id = unsafe { 
                    GetFormID(TESForm_GetFormByEditorID(&val.variable_id))
                };
                if form_id > 0 {
                    info!("observing actor value {} {:#X} on player", val.editor_id, form_id);
                    hashmap.insert(
                        form_id as i64,
                        (val.variable_id.clone(), Arc::new(AtomicI64::new(0)), variable.clone()),
                    );
                } else {
                    error!("could not find editor id {}", val.variable_id);
                }
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
                tracking_handles.cur_depth.clone(),
                ConfigVariable::Internal(BuiltIn::BoneTrackingDepth),
            ),
        );
        VariableStore { variables: hashmap }
    }

    pub fn init_actor_values(&self) {
        for entry in &self.variables {
            if let ConfigVariable::PlayerActorValue(val) = &entry.1.2 {
                let editor_id = &entry.1.0;
                let initial_val = normalize(val.min, val.max, unsafe { GetPlayerActorValue(editor_id) });
                debug!(initial_val, val.variable_id, "monitored actor value initialized");
                entry.1.1.store(initial_val, Ordering::Relaxed);
            }
        }
    }

    pub fn update(&self, form_id: u32, value: f32) -> Option<String> {
        let fid = form_id as i64;
        if let Some(entry) = self.variables.get(&fid) {
            if let ConfigVariable::PlayerActorValue(val) = &entry.2 {
                let new_val = normalize(val.min, val.max, value);
                debug!(new_val, val.variable_id, "monitored actor value changed");
                entry.1.store(new_val, Ordering::Relaxed);
                return Some(val.variable_id.clone());
            }
        }
        None
    }

    pub fn get(&self, name: &str) -> Option<Arc<AtomicI64>> {
        for variable in &self.variables {
            if variable.1.0 == name {
                return Some(variable.1.1.clone());
            }
        }
        None
    }
}

fn normalize(min: f32, max: f32, val: f32) -> i64 {
    let mut x = val;
    if x < min {
        x = min;
    }
    if x > max {
        x = max;
    }
    (((x - min) / (max - min)) * 100.0) as i64
}
