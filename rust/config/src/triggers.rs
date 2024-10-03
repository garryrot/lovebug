use std::collections::HashMap;

use bp_scheduler::config::read::read_config;
use tracing::{debug, info};

use crate::*;

#[derive(Debug)]
pub struct Triggers {
    /// maps lowercased scene IDs to a specific trigger
    scenes_exact_index: HashMap<String, Scene>,
    /// all remaining scenes
    scenes: Vec<Scene>
}

impl Triggers {

    pub fn default() -> Self {
        Triggers {
            scenes_exact_index: HashMap::new(),
            scenes: vec![],
        }
    }

    // let triggers : Vec<Trigger> = read_config(dir.into());
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
                Trigger::Event(_) => todo!(),
                Trigger::Timed(_) => todo!(),
            }
        }
        self.scenes.append(&mut scenes_default);

        info!("read {} scenes...", self.scenes.len());
        info!("indexed {} exact scenes...", self.scenes_exact_index.len());
        self.scenes_exact_index.iter().for_each(|(_, scene)| {
            debug!("{:?}", scene);
        });
        for scene in &self.scenes {
            debug!("{:?}", scene);
        }
    }

    pub fn find_scene(&self, scene_name: &str, tags: &Vec<String>) -> Option<Scene> {
        let scene_id: String = scene_name.to_lowercase();

        let mut scene : Option<Scene> = None;
        if self.scenes_exact_index.contains_key(&scene_id) {
            scene = Some(self.scenes_exact_index.get(&scene_id).unwrap().clone());
        } else {
            for wildcard in &self.scenes {
                if wildcard.scene_id.matches(&scene_id)
                    && wildcard.tags.matches(&tags)
                {
                    scene = Some(wildcard.clone());
                }
            }
        }
        scene
    }
}
