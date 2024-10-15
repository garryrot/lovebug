use std::time::Duration;

use collision::Collision;
use tokio::{sync::mpsc::unbounded_channel, time::{sleep, Instant}};
use tokio_util::sync::CancellationToken;
use tracing::{error, info};

use crate::Lovebug;

use bp_scheduler::dynamic_tracking::*;
use ffi_bones::*;

#[cxx::bridge]
mod ffi_bones {
    #[namespace = "RE"]
    unsafe extern "C++" {
        type Actor = crate::ffi::Actor;
        type NiAVObject;
    }

    #[derive(Debug)]
    enum Sex {
        None = -1,
        Male = 0,
        Female = 1,
        Total = 2
    }
    
    unsafe extern "C++" {
        type ActorVec = crate::ffi::ActorVec;

        include!("Bones.h");
        unsafe fn GetDistance(boneA: *mut NiAVObject, boneB: *mut NiAVObject) -> f32;
        unsafe fn GetBoneFromActor(actor: *const Actor, bone: &str) -> *mut NiAVObject;
        unsafe fn IsPlayer(actor: *const Actor) -> bool;
        unsafe fn GetSex(actor: *const Actor) -> Sex;
    }
    
    extern "Rust" {
        fn lb_dynamic_tracking(actors: &ActorVec);
        fn lb_dynamic_stop();
    }
}

/// Just for sending the array of actor pointers to a differen thread 
struct UnsafeActorVec {
    pub payload: Vec<*const Actor>
}

unsafe impl Send for UnsafeActorVec {}

impl UnsafeActorVec {
    fn move_hack(self) -> Self {
        let mut clone = vec![];
        for actor in self.payload {
            clone.push(actor);
        }
        Self {
            payload: clone
        }
    }
    fn from_actor_vec(actors: &ActorVec) -> Self {
        let mut vec_clone = vec![];
        for i in 0..actors.Size() {
            vec_clone.push(actors.GetActor(i));
        }
        Self {
            payload: vec_clone
        }
    }
}

pub fn lb_dynamic_stop() {
    info!("lb_dynamic_stop");
    Lovebug::run_static(
    |lb| {
        if let Some(token) = lb.dynamic_task.take() {
            token.cancel();
        }
    }, () );
}

pub fn lb_dynamic_tracking(actor_vec: &ActorVec) {
    info!("lb_dynamic_tracking Actors={}", actor_vec.Size());
    let actors_in: UnsafeActorVec = UnsafeActorVec::from_actor_vec(actor_vec);

    Lovebug::run_static(
        |lb| {
            if let Some(token) = lb.dynamic_task.take() {
                token.cancel();
            }
            let devices = lb.client.buttplug.devices();
            let cancellation_token = CancellationToken::new();
            lb.dynamic_task = Some(cancellation_token.clone());

            let (sender, receiver) = unbounded_channel::<TrackingSignal>();
            lb.client.runtime.spawn(async move {
                info!("control task started");
                let dynamic = DynamicTracking {
                    settings: DynamicSettings::default(),
                    signals: receiver,
                    devices,
                };
                let _ = dynamic.track_mirror().await;
            });

            lb.client.runtime.spawn(async move {
                info!("observation task started");
                let actors = actors_in.move_hack();
                
                if actors.payload.len() != 2 {
                    error!("not exactly 2 actors: {}", actors.payload.len());
                    return;
                }

                for actor in &actors.payload {
                    info!("actor {:?} Player:{}", unsafe { GetSex(*actor) }, unsafe { IsPlayer(*actor) });
                }

                let collision = Collision {
                    outer_distance: 14.5,
                    depth: 10.0,
                    min_stroke: 0.25,
                };

                let mut penetrating = false;
                let mut dir_inward = false;
                let mut last_distance = f32::MAX;
                let mut most_outward = f32::MAX;
                let mut most_inward = 0.0;

                while !cancellation_token.is_cancelled() {     
                    // TODO: Test if this can be moved upwards
                    fn get_genital_bone(actor: *const Actor) -> *mut NiAVObject {
                        unsafe  {
                            let sex  = GetSex( actor );
                            match sex {
                                Sex::Female => GetBoneFromActor(actor, "Pelvis_skin"),
                                Sex::Male => GetBoneFromActor(actor, "Penis_01"),
                                _ => {
                                    error!("Unknown sex type {:?}", sex);
                                    GetBoneFromActor(actor, "Pelvis_skin")
                                }
                            }
                        }
                    }
                    let bone_a = get_genital_bone( actors.payload[ 0 ] );
                    let bone_b = get_genital_bone( actors.payload[ 1 ] );

                    let dist = unsafe { GetDistance( bone_a, bone_b ) };
                    if dist < collision.outer_distance {
                        if ! penetrating {
                            info!("sending penetration {}", dist);
                            sender.send(TrackingSignal::Penetration(Instant::now())).unwrap(); // TODO remove unwrap
                        }
                        penetrating = true;
                    }

            
                    info!("dist = {}", dist);
                    let diff = last_distance - dist;
                    if diff > 0.0 && !dir_inward {
                        most_outward = dist;
                        info!(most_outward, most_inward);

                        let (from, to) = collision.get_stroke_range(most_outward, most_inward );
                        let _ = sender.send(TrackingSignal::OuterTurn(Instant::now(), Margins::new(from, to)));

                        info!(from, to, dist, "sending outward complete");
                        dir_inward = true;
                        penetrating = false;
                    } else if diff < 0.0 && dir_inward {
                        info!(most_outward, most_inward);

                        let (from, to) = collision.get_stroke_range(most_outward, most_inward );
                        let _ = sender.send(TrackingSignal::InnerTurn(Instant::now(), Margins::new(from, to))); // TODO remove unwrap

                        info!(from, to, dist, "sending inward complete");
                        most_inward = dist;
                        dir_inward = false;
                        penetrating = false;
                    }
          
                    last_distance = dist;
                    sleep(Duration::from_millis(50)).await;
                }
            });
        },
        (),
    );
}

