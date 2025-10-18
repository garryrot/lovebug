use std::{sync::{atomic::{AtomicBool, Ordering}, Arc}, time::Duration};

use clibf4::bridge::ffi_bridge::{ContainsKeyword, PlayerCharacter_GetSingleton};
use tokio::time::sleep;
use tracing::*;

use crate::{
    process_triggers, 
    Telekinesis
};

pub fn start_kw_thread(tk: &mut Telekinesis, kws: Vec<(String, Arc<AtomicBool>)>) {
    if let Some(existing) = &tk.keyword_update_thread {
        existing.abort();
    }
    tk.keyword_update_thread = Some(tk.client.runtime.spawn(async move {
        debug!("keyword update thread");
        loop {
            sleep(Duration::from_millis(250)).await;
            let mut changed = false;
            for kw in &kws {
                let keyword = &kw.0;
                let old_value = kw.1.load( Ordering::Relaxed );
                let new_value = unsafe { ContainsKeyword(PlayerCharacter_GetSingleton(), &kw.0) };

                if old_value != new_value {
                    debug!(new_value, keyword, "player kw updated");
                    changed = true;
                }
                kw.1.store(new_value, Ordering::Relaxed);
            }
            if changed {
                debug!("evalute triggers");
                Telekinesis::run_static_no_return(|lb| {
                    process_triggers(lb, None, None, &vec![]);
                });
            }
        }
    }));
}
