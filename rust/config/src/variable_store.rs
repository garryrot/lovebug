use std::{
    collections::HashMap,
    sync::{
        atomic::{AtomicI64, Ordering},
        Arc,
    },
};
use std::fmt::Debug;

use tracing::{debug, error, info};

use crate::variables::PlayerActorValue;

pub trait VariableSource {
    fn get_player_actor_value(&self, editor_id: &str) -> f32;
    fn get_form_id(&self, editor_id: &str) -> u32;
}

#[derive(Clone, Debug)]
pub struct VariableState {
    actor_value: PlayerActorValue,
    actual_rounded: Arc<AtomicI64>,
    normalized: Arc<AtomicI64>
}

#[derive(Clone)]
pub struct VariableStore {
    variables: HashMap<i64, VariableState>,
    external_variables: HashMap<String, Arc<AtomicI64>>
}

impl Debug for VariableStore {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("VariableStore")
            .field("variables", &self.variables)
            .field("external_variables", &self.external_variables)
            .finish()
    }
}

impl VariableStore {
    pub fn init(
        source: Box<dyn VariableSource + Sync>,
        actor_values: Vec<PlayerActorValue>,
        externals: Vec<(String, Arc<AtomicI64>)>,
    ) -> Self {
        let mut variables = HashMap::new();
        let mut external_variables = HashMap::new();
        let variable_source = Arc::new(source);

        for actor_value in actor_values {
            let form_id = variable_source.get_form_id(&actor_value.editor_id);
            if form_id > 0 {
                let actual_value = variable_source.get_player_actor_value(&actor_value.editor_id);
                let normalized = normalize(actor_value.min, actor_value.max, actual_value);
                info!(
                    actual_value, normalized,
                    "observing player AV {} {:#X}", actor_value.editor_id, form_id
                );

                variables.insert(form_id as i64, VariableState {
                    actor_value: actor_value,
                    normalized: Arc::new(AtomicI64::new( normalized )),
                    actual_rounded: Arc::new(AtomicI64::new( actual_value as i64)),
                });
            } else {
                error!("player AV {} not found", &actor_value.editor_id);
            }
        }
        for external in externals {
            external_variables.insert(external.0, external.1);
        }

        VariableStore {
            variables,
            external_variables,
        }
    }

    pub fn update(&self, form_id: u32, value: f32) -> bool {
        if let Some(state) = self.variables.get(&(form_id as i64)) {
            let normalized = &state.actor_value.normalize(value);
            debug!(
                value,
                normalized, "observed player AV updated {} ", 
                state.actor_value.editor_id
            );
            state.normalized.store(*normalized, Ordering::Relaxed);
            state.actual_rounded.store(value as i64, Ordering::Relaxed);
            true
        } else {
            false
        }
    }

    pub fn get_value(&self, editor_id: &str) -> Option<i64> {
        for external_variable in &self.external_variables {
            if external_variable.0 == editor_id {
                return Some( external_variable.1.load(Ordering::Relaxed) );
            }
        }
        for (_form_id, state) in &self.variables {
            if state.actor_value.editor_id == editor_id {
                return Some(state.actual_rounded.load(Ordering::Relaxed));
            }
        }
        None
    }

    pub fn get_normalized(&self, editor_id: &str) -> Option<Arc<AtomicI64>> {
        for external_variable in &self.external_variables {
            if external_variable.0 == editor_id {
                return Some(external_variable.1.clone());
            }
        }
        for (_form_id, state) in &self.variables {
            if state.actor_value.editor_id == editor_id {
                return Some(state.normalized.clone());
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
