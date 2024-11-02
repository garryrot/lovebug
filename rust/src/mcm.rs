use bp_scheduler::actuator::*;
use config::body_parts::*;
use ffi_mcm::DevicePage;
use tracing::{debug, info};

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
                actuator: actuator.identifier().to_owned(),
                enabled: actuator.config.clone().unwrap().enabled,
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
        let mut settings = lb.client.device_settings.get_or_create(&data.actuator);
        settings.enabled = data.enabled;
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
        settings.body_parts = body_parts;
        lb.client.device_settings.update_device(settings);
        lb.store_devices();
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
