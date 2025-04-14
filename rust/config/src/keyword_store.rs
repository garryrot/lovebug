use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::fmt::Debug;

use tracing::info;

pub trait KeywordSource {
    fn player_has_keyword(&self, editor_id: &str) -> bool;
}

pub struct KeywordStore {
    keywords: Vec<(String, Arc<AtomicBool>)>,
}

impl Debug for KeywordStore {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("KeywordStore").field("keywords", &self.keywords).finish()
    }
}

impl KeywordStore {
    pub fn init(keyword_source: Box<dyn KeywordSource + Sync>, keyword_editor_ids: Vec<String>) -> Self {
        let mut keywords = vec![];
        for editor_id in keyword_editor_ids {
            let has_keyword = keyword_source.player_has_keyword(&editor_id);
            info!(has_keyword, editor_id, "observing player KW");
            let tuple = (
                editor_id,
                Arc::new(AtomicBool::new(has_keyword)),
            );
            keywords.push(tuple);
        }
        KeywordStore {
            keywords
        }
    }

    pub fn has_keyword(&self, editor_id: &str) -> bool {
        self.keywords.iter().any(|x| x.0 == editor_id && x.1.load(Ordering::Relaxed))
    }

    pub fn clone_keywords(&self) -> Vec<(String, Arc<AtomicBool>)> {
        self.keywords.clone()
    }
}
