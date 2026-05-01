mod config;
mod events;
mod logging;
mod mcm;
mod signals;
mod variable_store;
mod body_parts;
mod event_thread;

use lazy_static::lazy_static;
use std::{
    collections::HashMap, sync::{Arc, Mutex}, time::Duration
};

use tokio::{
    sync::mpsc::{UnboundedSender, unbounded_channel},
    time::Instant,
};
use tracing::{debug, error, info, warn};

use buttplug::core::message::ActuatorType;

use bp_scheduler::{
    client::{BpClient, ExecutionResult},
    config::{
        actions::*,
        actuators::*,
        client::*,
        util::{read::*, write::*},
    },
    dynamic_tracking::{
        *,
    },
    filter::Filter,
    speed::Speed,
};
use variable_store::VariableStore;

use config::*;
use events::{ffi_event::ModEvent, send_mod_event};
use signals::{ BodyPartFlag, ffi_signal::*};

use event_thread::start_outgoing_event_thread;

#[derive(Debug)]
pub struct Telekinesis {
    client: BpClient,
    dynamic_task: BoneTrackingStats,
    sender: Option<UnboundedSender<TrackingSignal>>,
    stroker_settings: StrokerSettings,
    variable_store: VariableStore,
    trigger_actions: HashMap<String, TriggerAction>,
    trigger_handles: HashMap<String, i32>
}

impl Telekinesis {
    pub fn is_loaded() -> bool {
        match LB.state.try_lock() {
            Ok(guard) => guard.is_some(),
            Err(_) => {
                error!("failed locking mutex");
                false
            }
        }
    }

    pub fn run_static<F, R>(func: F, default: R) -> R
    where
        F: FnOnce(&mut Telekinesis) -> R,
        R: std::fmt::Debug,
    {
        match LB.state.try_lock() {
            Ok(mut guard) => {
                if let Some(tk) = guard.as_mut() {
                    let result = func(tk);
                    debug!(?result);
                    result
                } else {
                    error!("State empty");
                    default
                }
            }
            Err(_) => {
                error!("failed locking mutex");
                default
            }
        }
    }

    pub fn run_static_no_return<F>(func: F)
    where
        F: FnOnce(&mut Telekinesis),
    {
        match LB.state.try_lock() {
            Ok(mut guard) => {
                if let Some(tk) = guard.as_mut() {
                    func(tk);
                } else {
                    error!("State empty");
                }
            }
            Err(_) => {
                error!("failed locking mutex");
            }
        }
    }

    pub fn run_static_destroy<F>(func: F)
    where
        F: FnOnce(&mut Telekinesis),
    {
        if let Ok(mut guard) = LB.state.try_lock() {
            match guard.take() {
                Some(mut tk) => {
                    func(&mut tk);
                }
                None => error!("State empty"),
            }
        }
    }

    pub fn store_devices(&mut self) {
        try_write(&self.client.device_settings, CONFIG_DIR, DEVICE_SETTINGS);
    }
}

#[derive(Debug)]
pub struct LbApi {
    pub state: Arc<Mutex<Option<Telekinesis>>>,
}

lazy_static! {
    static ref LB: LbApi = {
        LbApi {
            state: Arc::new(Mutex::new(None)),
        }
    };
}

#[cxx::bridge]
mod ffi {
    extern "Rust" {
        fn lb_is_loaded() -> bool;
        fn lb_connect(
            connection: i32,
            port: &str,
            host: &str,
            bluetooth: bool,
            xinput: bool,
            serial: bool,
        ) -> bool;
        fn lb_disconnect();

        // --- signal sinks ---
        type PenSignal;
        fn lb_recv_signal(sig: &PenSignal);

        type TriggerSignal;
        fn lb_recv_trigger(sig: &TriggerSignal);

        // --- direct commands ---
        // TODO currently unused
        fn lb_action(action_name_unsanitized: &str, speed: i32, time_secs: f32) -> i32;
        // fn lb_update(id: i32, speed: i32) -> bool;
        // fn lb_stop(id: i32) -> bool;
        // fn lb_control(
        //     qry: &str,
        //     arg0: i32,
        //     arg1: f32,
        //     arg2: &str,
        //     arg3: &CxxVector<CxxString>,
        // ) -> i32;
    }
}

pub fn lb_is_loaded() -> bool {
    Telekinesis::is_loaded()
}

pub fn lb_disconnect() {
    Telekinesis::run_static_destroy(|lb| {
        lb.client.stop_all();
        lb.client.disconnect();
    });
}

pub fn lb_connect(
    connection: i32,
    port: &str,
    host: &str,
    bluetooth: bool,
    xinput: bool,
    serial: bool,
) -> bool {
    // TODO: Do this in to background thread to avoid small UI stutter
    if let Ok(mut guard) = LB.state.try_lock() {
        let settings = ClientSettings {
            connection: match connection {
                0 => ConnectionType::InProcess,
                1 => ConnectionType::WebSocket(format!("{}:{}", host, port)),
                _ => ConnectionType::Test,
            },
            in_process_features: InProcessFeatures {
                bluetooth,
                serial,
                xinput,
            },
            pattern_path: String::from(PATTERNS_DIR),
        };
        info!(?settings, "lb_connect");
        let client = BpClient::connect(
            settings,
            read_or_default::<ActuatorSettings>(CONFIG_DIR, DEVICE_SETTINGS),
        );
        if let Err(e) = client {
            error!(?e, "connection error");
            return false;
        }

        let bone_track = BoneTrackingStats::default();

        let variable_store = VariableStore::init(vec![
            (VAR_BONE_TRACKING_RATE.into(), bone_track.cur_avg_ms.clone()),
            (VAR_BONE_TRACKING_DEPTH.into(), bone_track.cur_avg_depth.clone()),
            (VAR_BONE_TRACKING_POS.into(), bone_track.cur_pos.clone()),
        ]);

        let mut trigger_actions = HashMap::new();
        for trigger in read_trigger_actions(TRIGGERS_DIR).0 {
            trigger_actions.insert(trigger.trigger.to_ascii_lowercase(), trigger);
        }

        let mut lb = Telekinesis {
            client: client.unwrap(),
            dynamic_task: bone_track,
            variable_store,
            stroker_settings: read_or_default(CONFIG_DIR, BONE_TRACKING),
            sender: None,
            trigger_actions,
            trigger_handles: HashMap::new(),
        };

        lb.client.read_actions(ACTIONS_DIR);

        start_outgoing_event_thread(&lb.client);

        if lb.client.scan_for_devices() {
            send_mod_event(ModEvent::new("Tele_ConnectionSuccess", "", 0.0));
        } else {
            send_mod_event(ModEvent::new("Tele_ConnectionError", "", 0.0));
        };

        guard.replace(lb);
    } else {
        error!("init failed");
    }
    true
}

pub fn lb_recv_trigger(trig: &TriggerSignal) {
    info!("lb_recv_trigger {}", trig.name);
    let evt_name = trig.name.to_ascii_lowercase();
    let duration = if trig.duration_ms > 0 { Duration::from_millis(trig.duration_ms) } else { Duration::MAX };
    let stop_event = trig.end_trigger;

    Telekinesis::run_static_no_return(|lb| {
        if let Some(trigger) = lb.trigger_actions.get(&evt_name) {
            debug!(?trigger, "found trigger");
            let action_refs = get_actions_from_refs(lb, trigger.actions.clone());
            if !stop_event {
                let result = lb.client.execute_actions(action_refs, vec![], Speed::max(), duration, -1);
                lb.trigger_handles.insert(evt_name, result.handle);
                send_action_events(&result);
            } else if let Some(handle) = lb.trigger_handles.get(&evt_name) {
                lb.client.stop(*handle);
            }
        } else {
            warn!(evt_name, "trigger not found")
        }
    });
}

pub fn lb_recv_signal(sig: &PenSignal) {
    debug!("lb_recv_signal");

    Telekinesis::run_static_no_return(|lb| {
        match sig.signal_type {
            PenSignalType::Start => {
                info!("PenSignalType::Start");
                let (sender, receiver) = unbounded_channel();
                lb.sender = Some(sender);

                let devices = lb.client.buttplug.devices();
                let setting_clone = lb.stroker_settings.clone();
                let dynamic_task_clone = lb.dynamic_task.clone();
                let mut tags = vec![];
                fn has_flag(sig: &PenSignal, with: BodyPartFlag) -> bool {
                    sig.body_part_flags | (with as u64) > 0
                }
                if has_flag(sig, BodyPartFlag::Anal) {
                    tags.push(Box::new(Selector::Tag("anal".to_owned())));
                }
                if has_flag(sig, BodyPartFlag::Oral) {
                    tags.push(Box::new(Selector::Tag("oral".to_owned())));
                }
                if has_flag(sig, BodyPartFlag::Vaginal) {
                    tags.push(Box::new(Selector::Tag("vaginal".to_owned())));
                }
                if has_flag(sig, BodyPartFlag::Penis) {
                    tags.push(Box::new(Selector::Tag("penis".to_owned())));
                }

                let (_, actuators) =
                    Filter::new(lb.client.device_settings.clone(), devices.as_slice())
                        .load_config(&mut lb.client.device_settings)
                        .connected()
                        .enabled()
                        .with_actuator_types(&[ActuatorType::Position])
                        .with_selector(&Selector::Or(tags))
                        .result();

                lb.client.runtime.spawn(async move {
                    info!("starting bone tracking");
                    let mut dynamic = DynamicTracking {
                        settings: setting_clone,
                        signals: receiver,
                        actuators,
                        status: dynamic_task_clone,
                    };
                    info!(?dynamic.settings, "success! moving stroker");
                    let _ = dynamic.track_mirror().await;
                    info!("bone tracking finished");
                });
            }
            PenSignalType::Stop => {
                debug!("PenSignalType::Stop");
                lb.sender
                    .as_ref()
                    .inspect(|x| { 
                        if x.send(TrackingSignal::Stop).is_err() {
                            error!("queue gone");
                        }
                    } );
            }
            PenSignalType::InnerTurn => {
                debug!("PenSignalType::InnerTurn");
                let signal = TrackingSignal::InnerTurn(
                    Instant::now(),
                    Margins {
                        most_in: sig.most_in,
                        most_out: sig.most_out,
                    },
                );
                lb.sender.as_ref().inspect(|x| { 
                    if x.send(signal).is_err() {
                        error!("queue gone");
                    }
                });
            }
            PenSignalType::OuterTurn => {
                debug!("PenSignalType::OuterTurn");
                let signal = TrackingSignal::OuterTurn(
                    Instant::now(),
                    Margins {
                        most_in: sig.most_in,
                        most_out: sig.most_out,
                    },
                );
                lb.sender.as_ref().inspect(|x| { 
                    if x.send(signal).is_err() {
                        error!("queue gone");
                    }
                });
            }
            PenSignalType::Penetration => {
                debug!("PenSignalType::Penetration");
                let signal = TrackingSignal::Penetration(Instant::now());
                lb.sender.as_ref().inspect(|x| { 
                    if x.send(signal).is_err() {
                        error!("queue gone");
                    } 
                });
            }
            _ => {}
        }
    });
}

fn send_action_events(results: &ExecutionResult) {
    for result in &results.actions {
        let action_name = result.0.clone();
        if !result.1.is_empty() {
            let devices = result
                .1
                .iter()
                .map(|x| {
                    x.config
                        .as_ref()
                        .map(|y| y.actuator_config_id.clone())
                        .unwrap_or(x.identifier().to_owned())
                })
                .collect::<Vec<String>>()
                .join(", ");

            send_mod_event(ModEvent::new(
                "Tele_Action",
                &format!("{}: {}", action_name, devices),
                results.handle as f64,
            ));
        }
    }
}

fn get_actions_from_refs(
    lb: &mut Telekinesis,
    action_refs: Vec<ActionRef>,
) -> Vec<(Strength, Action)> {
    let mut result = vec![];
    for action_ref in action_refs {
        if let Some(action) = lb
            .client
            .actions
            .0
            .iter()
            .find(|x| x.name == action_ref.action)
        {
            let strn = match action_ref.strength {
                Stren::Constant(x) => Strength::Constant(x),
                Stren::Variable(var) => Strength::Variable(lb.variable_store.get(&var)),
                Stren::Funscript(x, y) => Strength::Funscript(x, y),
                Stren::RandomFunscript(x, y) => Strength::RandomFunscript(x, y),
            };
            result.push((strn, action.clone()));
        }
    }
    result
}

fn lb_action(action_name_unsanitized: &str, speed: i32, time_secs: f32) -> i32 {
    let action_name = action_name_unsanitized.to_ascii_lowercase();
    info!(
        action_name,
        action_name_unsanitized, speed, time_secs, "lb_action"
    );
    Telekinesis::run_static(
        |lb| {
            let actions = get_actions_from_refs(
                lb,
                vec![ActionRef {
                    action: action_name,
                    strength: Stren::Constant(100),
                }],
            );
            let results = lb.client.execute_actions(
                actions,
                vec![],
                Speed::new(speed.into()),
                if time_secs > 0.0 {
                        Duration::from_millis((time_secs * 1000.0) as u64)
                    } else {
                        Duration::MAX
                    },
                -1,
            );
            send_action_events(&results);
            results.handle
        },
        -1,
    );
    -1
}
