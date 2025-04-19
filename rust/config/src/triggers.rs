use std::{collections::HashMap, time::Duration};

use events::{Event, TimedEvent};
use tracing::{debug, info};

use crate::{keyword_store::KeywordStore, variable_store::VariableStore, *};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub enum Trigger {
    Scene(Scene),
    Event(Event),
    TimedEvent(TimedEvent)
}

impl Trigger {
    pub fn actions(&self) -> Vec<ActionRef> {
        match self {
            Trigger::Scene(scene) => scene.actions.clone(),
            Trigger::Event(event) => event.actions.clone(),
            Trigger::TimedEvent(timed_event) => timed_event.actions.clone(),
        }
    }

    pub fn duration(&self) -> Duration {
        match self {
            triggers::Trigger::TimedEvent(timed_event) => timed_event.duration,
            _ => Duration::MAX,
        }
    }
}

#[derive(Debug)]
pub struct Triggers {
    /// triggered by process_event
    pub events: Vec<Trigger>,
    /// maps lowercased scene IDs to a specific trigger
    scenes_exact_index: HashMap<String, Scene>,
    /// all remaining scenes
    scenes: Vec<Scene>,
    running: Vec<Trigger>,
}

impl Triggers {
    pub fn default() -> Self {
        Triggers {
            scenes_exact_index: HashMap::new(),
            scenes: vec![],
            events: vec![],
            running: vec![],
        }
    }

    pub fn load_triggers(&mut self, triggers: Vec<Trigger>) {
        let mut scenes_default : Vec<Scene> = vec![];
        for trigger in triggers {
            match trigger {
                Trigger::Scene(scene) => {
                    match scene.scene_id {
                        SceneId::Any => {
                            scenes_default.push(scene);
                        },
                        SceneId::Exact(ref string) => {
                            self.scenes_exact_index.insert(string.to_lowercase(), scene);
                        },
                        SceneId::Contains(_) => {
                            self.scenes.push(scene);
                        },
                    }
                },
                Trigger::Event(event) => {
                    self.events.push(Trigger::Event(event));
                }
                Trigger::TimedEvent(timed_event) => {
                    self.events.push(Trigger::TimedEvent(timed_event));
                },
            }
        }
        self.scenes.append(&mut scenes_default);

        info!("read {} scene triggers...", self.scenes.len());
        info!("read {} event triggers...", self.events.len());
        info!("indexed {} exact scenes...", self.scenes_exact_index.len());
        self.scenes_exact_index.iter().for_each(|(_, scene)| {
            debug!("{:?}", scene);
        });
        for scene in &self.scenes {
            debug!("{:?}", scene);
        }
        for trigger in &self.events {
            debug!("{:?}", trigger);
        }
    }

    pub fn start_events(&mut self, vars: &VariableStore, kws: &KeywordStore, event_name: Option<&str>, scene_name: Option<&str>, scene_tags: &Vec<String>) -> Vec<Trigger> {
        info!(?event_name, "start_events");
        let matched_events: Vec<Trigger> = self.events.iter().filter(|x| {
            match x {
                Trigger::Scene(_) => false,
                Trigger::Event(event) => {
                    event.start.iter().all(|x| x.matches(vars, kws, event_name))
                },
                Trigger::TimedEvent(event) => {
                    event.start.iter().all(|x| x.matches(vars, kws, event_name))
                },
            }
        })
        .cloned()
        .collect();

        let mut started_events = vec![];
        if let Some(scene) = scene_name {
            if let Some(scene_result) = self.start_scene(scene, scene_tags) {
                started_events.push(Trigger::Scene(scene_result));
            }
        }

        for matched_event in matched_events {
            if !self.running.contains(&matched_event) {
                self.running.push(matched_event.clone());
                debug!(?matched_event, "starting"); 
                started_events.push(matched_event);
            } else {
                debug!(?matched_event, "already running");
            }
        }

        started_events
    }

    pub fn start_scene(&self, scene_name: &str, tags: &Vec<String>) -> Option<Scene> {
        let scene_id: String = scene_name.to_lowercase();

        let mut scene = None;
        if self.scenes_exact_index.contains_key(&scene_id) {
            scene = Some(self.scenes_exact_index.get(&scene_id).unwrap().clone());
        } else {
            let lowercased_tags = tags.iter().map(|x| x.to_lowercase()).clone().collect();
            for wildcard in &self.scenes {
                if wildcard.scene_id.matches(&scene_id)
                    && wildcard.scene_tags.matches(&lowercased_tags)
                {
                    scene = Some(wildcard.clone());
                }
            }
        }
        scene
    }

    pub fn stop_events(&mut self, vars: &VariableStore,  kws: &KeywordStore, event_name: Option<&str>) -> Vec<Trigger> {
        let mut stopped = vec![];
        self.running.retain(|x| match x {
            Trigger::Scene(_) => true,
            Trigger::TimedEvent(_) => true,
            Trigger::Event(event) => {
                if event.stop.iter().all(|x| x.matches(vars, kws, event_name)) {
                    debug!(?event, "stopping");
                    stopped.push(Trigger::Event(event.clone()));
                    false
                } else {
                    true
                }
            },
        });
        stopped
    }
}
