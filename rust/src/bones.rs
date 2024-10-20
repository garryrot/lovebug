use std::time::Duration;

use tokio::{sync::mpsc::unbounded_channel, time::{sleep, Instant}};
use tokio_util::sync::CancellationToken;
use tracing::{debug, error, info};

use ffi_bones::{ActorVec, GetBoneFromActor, GetDistance, NiAVObject};
use crate::{
    bridge::ffi_bridge::{GetSex, IsPlayer, Sex}, ffi::*, Lovebug
};

use collision::Collision;
use bp_scheduler::dynamic_tracking::*;


#[cxx::bridge]
mod ffi_bones {
    #[namespace = "RE"]
    unsafe extern "C++" {
        type Actor = crate::ffi::Actor;
        type NiAVObject;
    }
    
    unsafe extern "C++" {
        include!("Bones.h");
        type ActorVec;
        pub fn GetActor(self: &ActorVec, pos: i32) -> *const Actor;
        pub fn Size(self: &ActorVec) -> i32;
        unsafe fn GetDistance(boneA: *mut NiAVObject, boneB: *mut NiAVObject) -> f32;
        unsafe fn GetBoneFromActor(actor: *const Actor, bone: &str) -> *mut NiAVObject;
    }
    
    extern "Rust" {
        fn lb_dynamic_tracking(actors: &ActorVec);
        fn lb_dynamic_stop();
    }
}

// *Actor
struct UnsafeActorPtr {
    pub actor: *const Actor
}
impl UnsafeActorPtr {
    fn get_genital_bone(&self) -> UnsafeAvObjectPtr {
        unsafe  {
            let sex  = GetSex( self.actor );
            let av_object = match sex {
                Sex::Female => GetBoneFromActor(self.actor, "Pelvis_skin"),
                Sex::Male => GetBoneFromActor(self.actor, "Penis_01"),
                _ => {
                    error!("Unknown sex type {:?}", sex);
                    GetBoneFromActor(self.actor, "Pelvis_skin")
                }
            };
            UnsafeAvObjectPtr {
                av_object,
            }
        }
    }
    fn get_sex(&self) -> Sex {
        unsafe { GetSex(self.actor) }
    }
    fn is_player(&self) -> bool  {
        unsafe { IsPlayer(self.actor) }
    }
}
impl Clone for UnsafeActorPtr {
    fn clone(&self) -> Self {
        Self { actor: self.actor }
    }
}
unsafe impl Send for UnsafeActorPtr {}

// *NiAVObject
struct UnsafeAvObjectPtr {
    pub av_object: *mut NiAVObject
}
impl Clone for UnsafeAvObjectPtr {
    fn clone(&self) -> Self {
        Self { av_object: self.av_object }
    }
}
impl UnsafeAvObjectPtr {
    fn get_distance(&self, other: &UnsafeAvObjectPtr) -> f32 {
        unsafe { GetDistance( self.av_object, other.av_object ) }
    }
}
unsafe impl Send for UnsafeAvObjectPtr {}

fn from_actor_vec(actors: &ActorVec) -> Vec<UnsafeActorPtr> {
    let mut vec_clone = vec![];
    for i in 0..actors.Size() {
        vec_clone.push( UnsafeActorPtr { actor: actors.GetActor(i) });
    }
    vec_clone
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
    let actors_in = from_actor_vec(actor_vec);

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
                let dynamic = DynamicTracking {
                    settings:  DynamicSettings {
                        move_at_start: true,
                        min_resolution_ms: 80,
                        min_duration_ms: 200,
                        default_stroke_ms: 400,
                        default_stroke_in: 0.0,
                        default_stroke_out: 1.0,
                        stroke_window_ms: 2_000,
                    },
                    signals: receiver,
                    devices,
                };
                info!(?dynamic.settings, "control task started with settings");
                let _ = dynamic.track_mirror().await;
            });

            lb.client.runtime.spawn(async move {
                info!("observation task started");
                let actors = actors_in.clone();
                
                if actors.len() != 2 {
                    error!("not exactly 2 actors: {}", actors.len());
                    return;
                }

                for actor in &actors {
                    info!("actor {:?} Player:{}", actor.get_sex(), actor.is_player());
                }

                let collision = Collision {
                    outer_distance: 14.5,
                    depth: 10.0,
                    min_stroke: 0.25,
                };

                sleep(Duration::from_millis(1200)).await;

                let mut penetrating = false;
                let mut dir_inward = false;
                let mut last_distance = f32::MAX;
                let mut most_outward = f32::MAX;
                let mut most_inward = 0.0;

                let bone_a = actors[ 0 ].get_genital_bone();
                let bone_b = actors[ 1 ].get_genital_bone();

                while !cancellation_token.is_cancelled() {     
                    let dist =  bone_a.get_distance(&bone_b);
                    if dist < collision.outer_distance {
                        if ! penetrating {
                            info!("sending penetration {}", dist);
                            let _ = sender.send(TrackingSignal::Penetration(Instant::now()));
                        }
                        penetrating = true;
                    }
            
                    debug!("dist = {}", dist);
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
                        let _ = sender.send(TrackingSignal::InnerTurn(Instant::now(), Margins::new(from, to)));

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

