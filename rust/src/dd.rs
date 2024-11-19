use std::time::Duration;

use tokio::time::sleep;
use tracing::*;

use crate::{
    bridge::ffi_bridge::{ContainsKeyword, PlayerCharacter_GetSingleton}, lb_process_event, lb_stop, Telekinesis
};


/// Workaround for legacy dd devices
/// This monitors the vibration keyword and start/stops an event, because
/// there is no other way to do this.
///
/// - DD will update the actor values for vibration whenever a plug starts
/// vibrating
/// - It will not reset it to 0 when the plug stop vibrating
/// - This monitors if "DD_kw_Event_IsVibrating" keyword is on the player and
///   turns off all the plugs by resetting the variable to 0
pub fn start_dd_workaround(tk: &mut Telekinesis) {
    info!("start_dd_workaround");
    let mut current_handle = -1;

    if let Some(existing) = &tk.variable_update_thread {
        existing.abort();
    }
    tk.variable_update_thread = Some(tk.client.runtime.spawn(async move {
        info!("dd workaround thread");
        let mut was_vibrating = false;
        loop {
            sleep(Duration::from_millis(200)).await;
            let is_vibrating = unsafe {
                ContainsKeyword(PlayerCharacter_GetSingleton(), "DD_kw_Event_IsVibrating")
            };
            if is_vibrating && ! was_vibrating {
                debug!("enabling dd vibrator");
                was_vibrating = true;
                current_handle = lb_process_event("DD.Vibrator", "", 0.0);
            } else if !is_vibrating && was_vibrating {
                debug!("disabling dd vibrator");
                was_vibrating = false;
                lb_stop(current_handle);
            }
        }
    }));
}
