use bp_scheduler::client::BpClient;
use buttplug::client::ButtplugClientEvent;
use futures_util::StreamExt;
use tracing::info;

use crate::events::{ffi_event::ModEvent, send_mod_event};

/// Sends outgoing events which can be consumed in-game by
///  - RegisterForModEvent (standard signature) on SKSE
///  - RegisterForExternalEvent on F4SE
/// Events can be sent by adding to the queue in the main struct
pub fn start_outgoing_event_thread(client: &BpClient) {
    let mut events = client.buttplug.event_stream();

    client.runtime.spawn(async move {
        while let Some(evt) = events.next().await {
            info!("got event: {:?}", evt);
            match evt {
                ButtplugClientEvent::DeviceAdded(device) => send_mod_event(ModEvent::new(
                    "Tele_DeviceAdded",
                    device.name().as_str(),
                    0.0,
                )),
                ButtplugClientEvent::DeviceRemoved(device) => send_mod_event(ModEvent::new(
                    "Tele_DeviceRemoved",
                    device.name().as_str(),
                    0.0,
                )),
                ButtplugClientEvent::Error(buttplug_error) => send_mod_event(ModEvent::new(
                    "Tele_ConnectionError",
                    format!("{:?}", buttplug_error).as_str(),
                    0.0,
                )),
                _ => {}
            };
        }
    });
}
