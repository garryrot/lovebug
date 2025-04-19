use bone_tracking::config::{read_bone_tracking_settings, BoneTrackingSettings};
use bones::start_bone_tracking;
use bridge::ffi_bridge::{
    ContainsKeyword, GetFormID, GetPlayerActorValue,
    PlayerCharacter_GetSingleton, TESForm_GetFormByEditorID,
};
use config::{
    keyword_store::KeywordSource, triggers::Trigger, variable_store::VariableSource, variables::ConfigVariable
};
use cxx::{CxxString, CxxVector};
use dd::start_kw_thread;
use input::{read_input_duration, read_input_string};
use keyword_store::KeywordStore;
use lazy_static::lazy_static;
use tokio::task::JoinHandle;
use variable_store::VariableStore;

use std::{
    collections::HashMap,
    sync::{atomic::AtomicI64, Arc, Mutex},
};
use tracing::{debug, error, info};

use crate::bones::ffi_bones::ActorVec;

use bp_scheduler::{
    client::{BpClient, ExecutionResult},
    config::{
        actions::*,
        actuators::*,
        client::*,
        util::{read::*, write::*},
    },
    dynamic_tracking::DynamicTrackingHandle,
    speed::Speed,
};

use ::config::*;
use events::{ffi_event::ModEvent, send_mod_event, start_outgoing_event_thread};
use triggers::Triggers;

pub static CONFIG_DIR: &str = "Data\\F4SE\\Plugins\\Telekinesis2";
pub static PATTERNS_DIR: &str = "Data\\F4SE\\Plugins\\Telekinesis2\\Patterns";
pub static ACTIONS_DIR: &str = "Data\\F4SE\\Plugins\\Telekinesis2\\Actions";
pub static TRIGGERS_DIR: &str = "Data\\F4SE\\Plugins\\Telekinesis2\\Triggers";
pub static VARIABLES_DIR: &str = "Data\\F4SE\\Plugins\\Telekinesis2\\Variables";

pub static LOGGING_SETTINGS: &str = "Logging.json";
pub static DEVICE_SETTINGS: &str = "Devices.json";

pub mod bridge;
mod dd;
mod events;
mod input;
mod logging;
mod mcm;
mod bones;
mod bone_tracking;

#[derive(Debug)]
pub struct Telekinesis {
    client: BpClient,
    triggers: Triggers,

    bone_tracking_config: BoneTrackingSettings,
    dynamic_task: DynamicTrackingHandle,

    // runtime state
    tracking_counter: i32,
    variable_store: VariableStore,
    keyword_store: KeywordStore,
    triggers_running: HashMap<Trigger, i32>,
    keyword_update_thread: Option<JoinHandle<()>>,
}

impl Telekinesis {
    pub fn run_static<F, R>(func: F, default: R) -> R
    where
        F: FnOnce(&mut Telekinesis) -> R,
        R: std::fmt::Debug,
    {
        if let Ok(mut guard) = LB.state.try_lock() {
            match guard.take() {
                Some(mut tk) => {
                    let result = func(&mut tk);
                    guard.replace(tk);
                    debug!("result: {:?}", result);
                    return result;
                }
                None => error!("State empty"),
            }
        } else {
            error!("failed locking mutex");
        }
        default
    }

    pub fn run_static_no_return<F>(func: F)
    where
        F: FnOnce(&mut Telekinesis),
    {
        if let Ok(mut guard) = LB.state.try_lock() {
            match guard.take() {
                Some(mut tk) => {
                    func(&mut tk);
                    guard.replace(tk);
                }
                None => error!("State empty"),
            }
        } else {
            error!("failed locking mutex");
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
    unsafe extern "C++" {
        type ActorVec = crate::bones::ffi_bones::ActorVec;
    }

    extern "Rust" {
        fn lb_connect(
            connection: i32,
            port: &str,
            host: &str,
            bluetooth: bool,
            xinput: bool,
            serial: bool,
        ) -> bool;
        fn lb_disconnect();
        fn lb_action(action: &str, speed: i32, time_sec: f32) -> i32;
        fn lb_scene(
            scene: &str,
            scene_tags: &CxxVector<CxxString>,
            speed: i32,
            time_sec: f32,
            actors: &ActorVec,
        ) -> i32;
        fn lb_update(id: i32, speed: i32) -> bool;
        fn lb_stop(id: i32) -> bool;
        fn lb_process_actor_value(form_id: u32, value: f32);
        fn lb_process_event(event_name: &str, str_arg: &str, num_arg: f32) -> i32;
    }
}

pub struct Fo4KeywordSource {}

impl KeywordSource for Fo4KeywordSource {
    fn player_has_keyword(&self, editor_id: &str) -> bool {
        unsafe { ContainsKeyword(PlayerCharacter_GetSingleton(), &editor_id) }
    }
}

pub struct Fo4VariableSource {}

impl VariableSource for Fo4VariableSource {
    fn get_player_actor_value(&self, editor_id: &str) -> f32 {
        unsafe { GetPlayerActorValue(editor_id) }
    }
    fn get_form_id(&self, editor_id: &str) -> u32 {
        unsafe { GetFormID(TESForm_GetFormByEditorID(editor_id)) }
    }
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
        if let Err(a) = client {
            error!(?a, "connection error");
            return false;
        }

        let bone_track = DynamicTrackingHandle::default();

        let mut actor_values = vec![];
        let mut keywords = vec![];
        for var in read_variables() {
            match var {
                ConfigVariable::PlayerActorValue(player_actor_value) => {
                    actor_values.push(player_actor_value.clone())
                }
                ConfigVariable::PlayerKeyword(keyword) => keywords.push(keyword.clone()),
                _ => {}
            }
        }
        let variable_store = VariableStore::init(
            Box::new(Fo4VariableSource {}),
            actor_values,
            vec![
                ("BoneTrackingRate".into(), bone_track.cur_avg_ms.clone()),
                ("BoneTrackingDepth".into(), bone_track.cur_avg_depth.clone()),
            ],
        );
        let mut triggers = Triggers::default();
        triggers.load_triggers(read_config_dir(TRIGGERS_DIR.into()));

        let mut lb = Telekinesis {
            client: client.unwrap(),
            triggers,
            dynamic_task: bone_track,
            bone_tracking_config: read_bone_tracking_settings(),
            tracking_counter: 0,
            variable_store,
            triggers_running: HashMap::new(),
            keyword_store: KeywordStore::init(Box::new(Fo4KeywordSource {}), keywords),
            keyword_update_thread: None
        };

        lb.client.read_actions(ACTIONS_DIR);

        start_outgoing_event_thread(&lb.client);

        if lb.client.scan_for_devices() {
            send_mod_event(ModEvent::new("Tele_ConnectionSuccess", "", 0.0));
        } else {
            send_mod_event(ModEvent::new("Tele_ConnectionError", "", 0.0));
        };

        let cloned_kws = lb.keyword_store.clone_keywords();
        start_kw_thread(&mut lb, cloned_kws);
        guard.replace(lb);
    } else {
        error!("init failed");
    }
    true
}

fn read_variables() -> Vec<config::variables::ConfigVariable> {
    let vars = read_config_dir(VARIABLES_DIR.into());
    for var in &vars {
        debug!(?var, "read variable");
    }
    info!("read {} variables...", vars.len());
    vars
}

pub fn lb_disconnect() {
    Telekinesis::run_static_destroy(|lb| {
        lb.client.stop_all();
        lb.client.disconnect();
    });
}

fn lb_action(action_name: &str, speed: i32, time_secs: f32) -> i32 {
    info!(action_name, speed, time_secs, "lb_action");
    Telekinesis::run_static(
        |lb| {
            let actions = get_actions_from_refs(
                lb,
                vec![ActionRef {
                    action: action_name.into(),
                    strength: Stren::Constant(100),
                }],
            );
            let results = lb.client.execute_actions(
                actions,
                vec![],
                Speed::new(speed.into()),
                read_input_duration(time_secs),
                -1,
            );
            send_action_events(&results);
            results.handle
        },
        -1,
    );
    -1
}

fn lb_scene(
    scene_name: &str,
    scene_tags: &CxxVector<CxxString>,
    speed: i32,
    time_secs: f32,
    actor_vec: &ActorVec,
) -> i32 {
    let tags = read_input_string(scene_tags);
    info!(scene_name, speed, time_secs, ?tags, "lb_scene");
    Telekinesis::run_static(
        |lb| {
            let (handle, triggers) = process_triggers(lb, None, Some(scene_name), &tags);
            for trigger in triggers {
                if let Trigger::Scene(scene) = trigger {
                    send_mod_event(ModEvent::new("Tele_Scene", &scene.description, 0.0));
                    if scene.track_bones {
                        start_bone_tracking(lb, actor_vec);
                    }
                }
            }
            handle
        },
        -1,
    )
}

fn lb_process_event(event_name: &str, str_arg: &str, num_arg: f32) -> i32 {
    info!(event_name, str_arg, num_arg, "lb_process_event");
    Telekinesis::run_static(
        |lb| {
            let (handle, _) = process_triggers(lb, Some(event_name), None, &vec![]);
            handle
        },
        -1,
    )
}

fn lb_process_actor_value(form_id: u32, value: f32) {
    Telekinesis::run_static_no_return(|lb| {
        if lb.variable_store.update(form_id, value) {
            process_triggers(lb, None, None, &vec![]);
        }
    });
}

fn process_triggers(
    lb: &mut Telekinesis,
    event_name: Option<&str>,
    scene_name: Option<&str>,
    scene_tags: &Vec<String>,
) -> (i32, Vec<Trigger>) {
    debug!(?event_name, ?scene_name, ?scene_tags, "process_triggers");
    let started = lb
        .triggers
        .start_events(&lb.variable_store, &lb.keyword_store, event_name, scene_name, scene_tags);

    let mut handle = -1;
    for trigger in started.iter() {
        let actions = get_actions_from_refs(lb, trigger.actions());
        let duration = trigger.duration();
        let results = lb
            .client
            .execute_actions(actions, vec![], Speed::max(), duration, handle);
        send_action_events(&results);
        handle = results.handle;
        debug!(handle, ?trigger, "started trigger");
        lb.triggers_running.insert(trigger.clone(), handle);
    }

    let stop_iter = lb.triggers.stop_events(&lb.variable_store, &lb.keyword_store, event_name);
    for trigger in stop_iter.into_iter() {
        if let Some(handle) = lb.triggers_running.get(&trigger) {
            lb.client.stop(*handle);
            debug!(handle, ?trigger, "stopped trigger");
        } else {
            error!(?trigger, "no handle found")
        }
    }

    (handle, started)
}

pub fn lb_update(handle: i32, speed: i32) -> bool {
    info!(handle, speed, "lb_update");
    Telekinesis::run_static(
        |lb| lb.client.update(handle, Speed::new(speed.into())),
        false,
    )
}

pub fn lb_stop(handle: i32) -> bool {
    info!(handle, "lb_stop");
    Telekinesis::run_static(|lb| lb.client.stop(handle), false)
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
                Stren::Variable(var) => Strength::Variable(match var {
                    Variable::BoneTrackingRate => lb.dynamic_task.cur_avg_ms.clone(),
                    Variable::BoneTrackingDepth => lb.dynamic_task.cur_avg_depth.clone(),
                    Variable::BoneTrackingPos => lb.dynamic_task.cur_pos.clone(),
                    Variable::PlayerActorValue(name) => {
                        if let Some(var) = lb.variable_store.get_normalized(&name) {
                            var.clone()
                        } else {
                            error!(name, "unknown player actor value");
                            Arc::new(AtomicI64::new(0))
                        }
                    }
                }),
                Stren::Funscript(x, y) => Strength::Funscript(x, y),
                Stren::RandomFunscript(x, y) => Strength::RandomFunscript(x, y),
            };
            result.push((strn, action.clone()));
        }
    }
    result
}
