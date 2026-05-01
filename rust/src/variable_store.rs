use std::fmt::Debug;
use std::{
    collections::HashMap,
    sync::{
        atomic::{AtomicI64, Ordering},
        Arc,
    },
};

use tracing::debug;

#[derive(Clone)]
pub struct VariableStore {
    vars: HashMap<String, Arc<AtomicI64>>,
}

impl Debug for VariableStore {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("VariableStore")
            .field("vars", &self.vars)
            .finish()
    }
}

impl VariableStore {
    pub fn init(vars: Vec<(String, Arc<AtomicI64>)>) -> Self {
        let mut map = HashMap::new();
        for var in vars {
            map.insert(var.0, var.1);
        }
        Self {
            vars: map,
        }
    }

    pub fn get(&mut self, name: &str) -> Arc<AtomicI64> {
        let value = self
            .vars
            .entry(name.to_owned())
            .or_insert(Arc::new(AtomicI64::new(0)))
            .clone();
        debug!(?value, name, "variable get");
        value
    }

    pub fn set(&mut self, name: String, value: i64) {
        match self.vars.get(&name) {
            Some(val) => {
                debug!(value, name, "variable set");
                val.store(value, Ordering::Relaxed)
            }
            None => {
                debug!(value, name, "variable init");
                self.vars.insert(name, Arc::new(AtomicI64::new(value)));
            }
        }
    }
}
