use bp_scheduler::actuator::*;
use config::body_parts::*;
use ffi_mcm::DevicePage;
use tracing::{debug, error, info};

use crate::Telekinesis;

#[cxx::bridge]
mod ffi_mcm {
    #[derive(Debug, Clone)]
    pub struct DevicePage {
        // info
        pub index: i32,
        pub actuator: String,
        pub error: String,
        // read/write
        pub enabled: bool,
        pub anal: bool,
        pub clitoral: bool,
        pub nipple: bool,
        pub penis: bool,
        pub oral: bool,
        pub vaginal: bool,
    }

    extern "Rust" {
        fn lb_actuator_get(index: u32) -> DevicePage;
        fn lb_actuator_set(actuator: DevicePage) -> bool;
        fn lb_actuator_len() -> u32;
    }
}

pub fn lb_actuator_len() -> u32 {
    Telekinesis::run_static(|lb| { 
        lb.client.buttplug.devices().flatten_actuators().len() as u32
    }, 0)
}

pub fn lb_actuator_get(index: u32) -> DevicePage {
    debug!(index, "lb_actuator_get");
    Telekinesis::run_static(|lb| {
        let actuators = lb.client.buttplug.devices().flatten_actuators().load_config( &mut lb.client.device_settings );

        if actuators.len() > index as usize {
            let actuator = actuators[ index as usize ].as_ref();
            let settings = actuator.config.clone().unwrap();
            DevicePage {
                index: index as i32,
                actuator: settings.actuator_config_id,
                enabled: settings.enabled,
                error: "".to_owned(),
                anal: settings.body_parts.contains(&TAG_ANAL.to_owned()),
                clitoral: settings.body_parts.contains(&TAG_CLIT.to_owned()),
                nipple: settings.body_parts.contains(&TAG_NIPPLE.to_owned()),
                penis: settings.body_parts.contains(&TAG_PENIS.to_owned()),
                oral: settings.body_parts.contains(&TAG_ORAL.to_owned()),
                vaginal: settings.body_parts.contains(&TAG_VAGINAL.to_owned()),
            }
        } else {
            DevicePage::default()
        }
    }, DevicePage::default())
}

pub fn lb_actuator_set(data: DevicePage) -> bool {
    info!(?data, "lb_actuator_set");
    Telekinesis::run_static(|lb| {
        let actuators = lb.client.buttplug.devices().flatten_actuators().load_config( &mut lb.client.device_settings );

        let i = data.index;
        if let Some(ref mut actuator) = actuators.get(i as usize) {
            if actuator.config.is_none() {
                error!("no config");
                return false;
            }

            let mut actuator_clone = actuator.as_ref().clone();
            if actuator_clone.config.clone().unwrap().actuator_config_id != data.actuator {
                error!("name mismatch, index changed?");
                return false;
            }

            match actuator_clone.config {
                Some(ref mut setting) => {
                    setting.enabled = data.enabled;
                    let mut body_parts = vec![];
                    if data.anal {
                        body_parts.push(TAG_ANAL.to_owned());
                    }
                    if data.clitoral {
                        body_parts.push(TAG_CLIT.to_owned());
                    }
                    if data.nipple {
                        body_parts.push(TAG_NIPPLE.to_owned());
                    }
                    if data.oral {
                        body_parts.push(TAG_ORAL.to_owned());
                    }
                    if data.vaginal {
                        body_parts.push(TAG_VAGINAL.to_owned());
                    }
                    if data.penis {
                        body_parts.push(TAG_PENIS.to_owned());
                    }
                    setting.body_parts = body_parts;
                    lb.client.device_settings.update_device(setting.clone());
                    lb.store_devices();
                },
                None => {
                    error!( actuator.index_in_device, identifier=actuator.identifier(), "actuator has no config");
                },
            }
        }
        true
    }, false)
}

impl Default for DevicePage {
    fn default() -> Self {
        Self {
            index: -1,
            actuator: Default::default(),
            enabled: Default::default(),
            error: Default::default(),
            anal: Default::default(),
            clitoral: Default::default(),
            nipple: Default::default(),
            penis: Default::default(),
            oral: Default::default(),
            vaginal: Default::default(),
        }
    }
}
