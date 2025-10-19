// shared binary API for Script Extender signalling

#![allow(dead_code)]

pub const API_VERSION: u64 = 1;

#[derive(Clone, Debug)]
pub enum BodyPartFlag {
    Oral = 1,
    Anal = 2,
    Vaginal = 4,
    Penis = 8,
}

#[cxx::bridge]
pub mod ffi_signal {
    enum PenSignalType {
        Start,
        Stop,
        Penetration,
        InnerTurn,
        OuterTurn
    }

    struct PenSignal {
        version: u64,
        signal_type: PenSignalType,
        most_in: f64,
        most_out: f64,
        body_part_flags: u64
    }

    struct TriggerSignal {
        version: u64,
        name: String,
        duration_ms: u64,
        end_trigger: bool,
    }

    struct VarSignal {
        version: u64,
        name: String,
        value: f64
    }
}
