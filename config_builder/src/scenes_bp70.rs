use bp_scheduler::config::actions::*;
use config::*;
use triggers::Trigger;

use crate::scene;

pub fn pb70_triggers() -> Vec<Trigger> {
    fn pb70_name(name: &str) -> String {
        format!("[UAP] BP70 - {}", name)
    }

    let vec = vec![
        scene(
            "Cunnilungus",
            SceneId::Exact(pb70_name("Cunnilungus")),
            vec![
                ActionRef::new("cunnilungus", Stren::Constant(60)),
                ActionRef::new("vibrate.nipple", Stren::Constant(20)),
                ActionRef::new("masturbation", Stren::Constant(30)),
            ],
        ),
        scene(
            "Footjob",
            SceneId::Exact(pb70_name("Footjob")),
            vec![ActionRef::new("footjob", Stren::Constant(25))],
        ),
        scene(
            "Footjob Fast",
            SceneId::Exact(pb70_name("Footjob Fast")),
            vec![ActionRef::new("footjob", Stren::Constant(75))],
        ),
        scene(
            "Impregnate Missionary Stage Kiss",
            SceneId::Exact(pb70_name("Impregnate Missionary Stage Kiss")),
            vec![
                ActionRef::new("constrict.penis", Stren::Constant(20)),
                ActionRef::new("constrict.vaginal", Stren::Constant(20)),
            ],
        ),
        scene(
            "Kissing",
            SceneId::Exact(pb70_name("Kissing")),
            vec![],
        ),
        scene(
            "Leg Wrap Cunnilungus",
            SceneId::Exact(pb70_name("Leg Wrap Cunnilungus")),
            vec![ActionRef::new("cunnilungus", Stren::Constant(70))],
        ),
        scene(
            "Missionary Grind (tease)",
            SceneId::Exact(pb70_name("Missionary Grind (tease)")),
            vec![ActionRef::new("penetration.vaginal", Stren::Constant(10))],
        ),
        scene(
            "Missionary Sequence Stage 1",
            SceneId::Exact(pb70_name("Missionary Sequence Stage 1")),
            vec![ActionRef::new("masturbation", Stren::Constant(20))],
        ),
        scene(
            "Mistress",
            SceneId::Exact(pb70_name("Mistress")),
            vec![ActionRef::new("masturbation", Stren::Constant(85))],
        ),
        scene(
            "Mistress 2",
            SceneId::Exact(pb70_name("Mistress 2")),
            vec![ActionRef::new("masturbation", Stren::Constant(60))],
        ),
        scene(
            "Mistress 3",
            SceneId::Exact(pb70_name("Mistress 3")),
            vec![ActionRef::new("cunnilungus", Stren::Constant(50))],
        ),
        scene(
            "Pit Doggy 01 (fingering, kissing)",
            SceneId::Exact(pb70_name("Pit Doggy 01")),
            vec![
                ActionRef::new("cunnilungus", 
                Stren::Constant(40))
            ],
        ),
        scene(
            "Pit Doggy 02 (fingering, kissing)",
            SceneId::Exact(pb70_name("Pit Doggy 02")),
            vec![
                ActionRef::new("cunnilungus", 
                Stren::Constant(60))
            ],
        ),
        scene(
            "Pit Doggy 06",
            SceneId::Exact(pb70_name("Pit Doggy 06")),
            vec![
                ActionRef::new("masturbation", 
                Stren::Constant(80))
            ],
        ),
        scene(
            "Prone Bone 01",
            SceneId::Exact(pb70_name("Prone Bone 01")),
            vec![
                ActionRef::new("masturbation", 
                Stren::Constant(50))
            ],
        ),
        scene(
            "Prone Bone Anal 02",
            SceneId::Exact(pb70_name("Prone Bone Anal 02")),
            vec![
                ActionRef::new("penetration.anal", 
                Stren::Constant(70))
            ],
        ),
        scene(
            "Prone Bone Anal 03",
            SceneId::Exact(pb70_name("Prone Bone Anal 03")),
            vec![
                ActionRef::new("penetration.anal.deep", 
                Stren::Constant(90))
            ],
        ),
        scene(
            "Romantic Cowgirl",
            SceneId::Exact(pb70_name("Romantic Cowgirl")),
            vec![
                ActionRef::new("penetration.vaginal", 
                Stren::Constant(70))
            ],
        ),
        scene(
            "Romantic Missionary 01",
            SceneId::Exact(pb70_name("Romantic Missionary 01")),
            vec![],
        ),
        scene(
            "Spooning 01",
            SceneId::Exact(pb70_name("Spooning 01")),
            vec![],
        ),
        scene(
            "Standing Sequence",
            SceneId::Exact(pb70_name("Standing Sequence Stage 1")),
            vec![],
        )
    ];
    vec
}