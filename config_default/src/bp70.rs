use bp_scheduler::config::actions::*;
use config::*;

use crate::scene;

pub fn get_pb70_triggers() -> Vec<Trigger> {
    fn pb70_name(name: &str) -> String {
        format!("[UAP] BP70 - {}", name)
    }

    let vec = vec![
        scene(
            "Blowjob",
            SceneId::Exact(pb70_name("Blowjob")),
            vec![ActionRef::new("penetration.oral", Strength::Constant(80))],
        ),
        scene(
            "Cowgirl",
            SceneId::Exact(pb70_name("Cowgirl")),
            vec![ActionRef::new(
                "penetration.vaginal",
                Strength::Constant(70),
            )],
        ),
        scene(
            "Cowgirl Sequence 1",
            SceneId::Exact(pb70_name("Cowgirl Sequence 1")),
            vec![ActionRef::new(
                "penetration.vaginal",
                Strength::Constant(25),
            )],
        ),
        scene(
            "Cowgirl Sequence 2",
            SceneId::Exact(pb70_name("Cowgirl Sequence 2")),
            vec![ActionRef::new(
                "penetration.vaginal",
                Strength::Constant(40),
            )],
        ),
        scene(
            "Cowgirl Sequence 3",
            SceneId::Exact(pb70_name("Cowgirl Sequence 3")),
            vec![ActionRef::new(
                "penetration.vaginal.deep",
                Strength::Constant(60),
            )],
        ),
        scene(
            "Cowgirl Sequence 4",
            SceneId::Exact(pb70_name("Cowgirl Sequence 4")),
            vec![ActionRef::new(
                "penetration.vaginal.deep",
                Strength::Constant(70),
            )],
        ),
        scene(
            "Cowgirl Sequence 5",
            SceneId::Exact(pb70_name("Cowgirl Sequence 5")),
            vec![ActionRef::new(
                "penetration.vaginal.deep",
                Strength::Constant(90),
            )],
        ),
        scene(
            "Cunnilungus",
            SceneId::Exact(pb70_name("Cowgirl Sequence 5")),
            vec![
                ActionRef::new("cunnilungus", Strength::Constant(60)),
                ActionRef::new("vibrate.nipple", Strength::Constant(20)),
                ActionRef::new("masturbation", Strength::Constant(30)),
            ],
        ),
        scene(
            "Doggy",
            SceneId::Exact(pb70_name("Doggy")),
            vec![ActionRef::new(
                "penetration.vaginal",
                Strength::Constant(70),
            )],
        ),
        scene(
            "Footjob",
            SceneId::Exact(pb70_name("Footjob")),
            vec![ActionRef::new("footjob", Strength::Constant(25))],
        ),
        scene(
            "Footjob Fast",
            SceneId::Exact(pb70_name("Footjob Fast")),
            vec![ActionRef::new("footjob", Strength::Constant(75))],
        ),
        scene(
            "Impregnate Cowgirl Kiss",
            SceneId::Exact(pb70_name("Impregnate Cowgirl Kiss")),
            vec![ActionRef::new("constrict.penis", Strength::Constant(20))],
        ),
        scene(
            "Impregnate Cowgirl Start Fucking",
            SceneId::Exact(pb70_name("Impregnate Cowgirl Start Fucking")),
            vec![ActionRef::new(
                "penetration.vaginal",
                Strength::Constant(30),
            )],
        ),
        scene(
            "Impregnate Cowgirl Fuck Faster and Deeper",
            SceneId::Exact(pb70_name("Impregnate Cowgirl Fuck Faster and Deeper")),
            vec![ActionRef::new(
                "penetration.vaginal.deep",
                Strength::Constant(50),
            )],
        ),
        scene(
            "Impregnate Cowgirl Thrust Into Her",
            SceneId::Exact(pb70_name("Impregnate Cowgirl Thrust Into Her")),
            vec![ActionRef::new(
                "penetration.vaginal.deep",
                Strength::Constant(75),
            )],
        ),
        scene(
            "Impregnate Cowgirl Orgasm",
            SceneId::Exact(pb70_name("Impregnate Cowgirl Orgasm")),
            vec![ActionRef::new(
                "penetration.vaginal.deep",
                Strength::Constant(100),
            )],
        ),
        scene(
            "Impregnate Missionary Stage Kiss",
            SceneId::Exact(pb70_name("Impregnate Missionary Stage Kiss")),
            vec![
                ActionRef::new("constrict.penis", Strength::Constant(20)),
                ActionRef::new("constrict.vagina", Strength::Constant(20)),
            ],
        ),
        scene(
            "Impregnate Missionary Stage Grab His Back",
            SceneId::Exact(pb70_name("Impregnate Missionary Stage Grab His Back")),
            vec![ActionRef::new("penetration.vaginal", Strength::Constant(60))],
        ),
        scene(
            "Impregnate Missionary Stage Grab His Back 2",
            SceneId::Exact(pb70_name("Impregnate Missionary Stage Grab His Back 2")),
            vec![ActionRef::new("penetration.vaginal", Strength::Constant(70))],
        ),
        scene(
            "Impregnate Missionary Stage Grab His Back 3",
            SceneId::Exact(pb70_name("Impregnate Missionary Stage Grab His Back 3")),
            vec![ActionRef::new("penetration.vaginal.deep", Strength::Constant(75))],
        ),
        scene(
            "Impregnate Missionary Stage Grab His Back 4",
            SceneId::Exact(pb70_name("Impregnate Missionary Stage Grab His Back 4")),
            vec![ActionRef::new("penetration.vaginal.deep", Strength::Constant(100))],
        ),
        scene(
            "Kissing",
            SceneId::Exact(pb70_name("Kissing")),
            vec![],
        ),
        scene(
            "Laying Blowjob",
            SceneId::Exact(pb70_name("Laying Blowjob")),
            vec![ActionRef::new("penetration.oral", Strength::Constant(70))],
        ),
        scene(
            "Leg Wrap Cunnilungus",
            SceneId::Exact(pb70_name("Leg Wrap Cunnilungus")),
            vec![ActionRef::new("cunnilungus", Strength::Constant(70))],
        ),
        scene(
            "Lotus",
            SceneId::Exact(pb70_name("Lotus")),
            vec![ActionRef::new("penetration.vaginal", Strength::Constant(60))],
        ),
        scene(
            "Missionary",
            SceneId::Exact(pb70_name("Missionary")),
            vec![ActionRef::new("penetration.vaginal", Strength::Constant(60))],
        ),
        scene(
            "Missionary Grind (tease)",
            SceneId::Exact(pb70_name("Missionary Grind (tease)")),
            vec![ActionRef::new("penetration.vaginal", Strength::Constant(10))],
        ),
        scene(
            "Missionary Grind 2",
            SceneId::Exact(pb70_name("Missionary Grind 2")),
            vec![ActionRef::new("penetration.vaginal", Strength::Constant(40))],
        ),
        scene(
            "Missionary Grind 3",
            SceneId::Exact(pb70_name("Missionary Grind 3")),
            vec![ActionRef::new("penetration.vaginal.deep", Strength::Constant(60))],
        ),
        scene(
            "Missionary Grind 4",
            SceneId::Exact(pb70_name("Missionary Grind 4")),
            vec![ActionRef::new("penetration.vaginal.deep", Strength::Constant(80))],
        ),
        scene(
            "Missionary Sequence Stage 1",
            SceneId::Exact(pb70_name("Missionary Sequence Stage 1")),
            vec![ActionRef::new("masturbation", Strength::Constant(20))],
        ),
        scene(
            "Missionary Sequence Stage 2",
            SceneId::Exact(pb70_name("Missionary Sequence Stage 2")),
            vec![ActionRef::new("penetration.vaginal", Strength::Constant(60))],
        ),
        scene(
            "Missionary Sequence Stage 3", // Is this correct?
            SceneId::Exact(pb70_name("Missionary Sequence Stage 2")),
            vec![ActionRef::new("penetration.vaginal.deep", Strength::Constant(80))],
        ),
        scene(
            "Mistress",
            SceneId::Exact(pb70_name("Mistress")),
            vec![ActionRef::new("masturbation", Strength::Constant(85))],
        ),
        scene(
            "Mistress 2",
            SceneId::Exact(pb70_name("Mistress 2")),
            vec![ActionRef::new("masturbation", Strength::Constant(60))],
        ),
        scene(
            "Mistress 3",
            SceneId::Exact(pb70_name("Mistress 3")),
            vec![ActionRef::new("cunnilungus", Strength::Constant(50))],
        ),
        scene(
            "Mistress 4",
            SceneId::Exact(pb70_name("Mistress 4")),
            vec![ActionRef::new("penetration.vaginal", Strength::Constant(75))],
        ),
        scene(
            "Mistress 5",
            SceneId::Exact(pb70_name("Mistress 5")),
            vec![ActionRef::new("penetration.vaginal.deep", Strength::Constant(80))],
        ),
        scene(
            "Pit Doggy 01 (fingering, kissing)",
            SceneId::Exact(pb70_name("Pit Doggy 01")),
            vec![
                ActionRef::new("cunnilungus", 
                Strength::Constant(40))
            ],
        ),
        scene(
            "Pit Doggy 02 (fingering, kissing)",
            SceneId::Exact(pb70_name("Pit Doggy 02")),
            vec![
                ActionRef::new("cunnilungus", 
                Strength::Constant(60))
            ],
        ),
        scene(
            "Pit Doggy 03",
            SceneId::Exact(pb70_name("Pit Doggy 03")),
            vec![
                ActionRef::new("penetration.vaginal", 
                Strength::Constant(60))
            ],
        ),
        scene(
            "Pit Doggy 04",
            SceneId::Exact(pb70_name("Pit Doggy 04")),
            vec![
                ActionRef::new("penetration.vaginal.deep", 
                Strength::Constant(60))
            ],
        ),
        scene(
            "Pit Doggy 05",
            SceneId::Exact(pb70_name("Pit Doggy 05")),
            vec![
                ActionRef::new("penetration.vaginal.deep", 
                Strength::Constant(80))
            ],
        ),
        scene(
            "Pit Doggy 06",
            SceneId::Exact(pb70_name("Pit Doggy 06")),
            vec![
                ActionRef::new("masturbation", 
                Strength::Constant(80))
            ],
        ),
        scene(
            "Prone Bone 01",
            SceneId::Exact(pb70_name("Prone Bone 01")),
            vec![
                ActionRef::new("masturbation", 
                Strength::Constant(50))
            ],
        ),
        scene(
            "Prone Bone 02",
            SceneId::Exact(pb70_name("Prone Bone 02")),
            vec![
                ActionRef::new("penetration.vaginal", 
                Strength::Constant(70))
            ],
        ),
        scene(
            "Prone Bone 03",
            SceneId::Exact(pb70_name("Prone Bone 03")),
            vec![
                ActionRef::new("penetration.vaginal.deep", 
                Strength::Constant(90))
            ],
        ),
        scene(
            "Prone Bone Anal 02",
            SceneId::Exact(pb70_name("Prone Bone Anal 02")),
            vec![
                ActionRef::new("penetration.anal", 
                Strength::Constant(70))
            ],
        ),
        scene(
            "Prone Bone Anal 03",
            SceneId::Exact(pb70_name("Prone Bone Anal 03")),
            vec![
                ActionRef::new("penetration.anal.deep", 
                Strength::Constant(90))
            ],
        ),
        scene(
            "Romantic Cowgirl",
            SceneId::Exact(pb70_name("Romantic Cowgirl")),
            vec![
                ActionRef::new("penetration.vaginal", 
                Strength::Constant(70))
            ],
        ),
    
        scene(
            "Romantic Missionary 01",
            SceneId::Exact(pb70_name("Romantic Missionary 01")),
            vec![],
        ),
    
        scene(
            "Romantic Missionary 02",
            SceneId::Exact(pb70_name("Romantic Missionary 02")),
            vec![
                ActionRef::new("penetration.vaginal", 
                Strength::Constant(50))
            ],
        ),
    
        scene(
            "Romantic Missionary 03",
            SceneId::Exact(pb70_name("Romantic Missionary 03")),
            vec![
                ActionRef::new("penetration.vaginal", 
                Strength::Constant(70))
            ],
        ),
        scene(
            "Romantic Missionary 04",
            SceneId::Exact(pb70_name("Romantic Missionary 04")),
            vec![
                ActionRef::new("penetration.vaginal.deep", 
                Strength::Constant(80))
            ],
        ),
        scene(
            "Spooning 01",
            SceneId::Exact(pb70_name("Spooning 01")),
            vec![],
        ),
        scene(
            "Spooning 02",
            SceneId::Exact(pb70_name("Spooning 02")),
            vec![
                ActionRef::new("penetration.vaginal", 
                Strength::Constant(40))
            ],
        ),
        scene(
            "Spooning 03",
            SceneId::Exact(pb70_name("Spooning 03")),
            vec![
                ActionRef::new("penetration.vaginal.deep", 
                Strength::Constant(60))
            ],
        ),
        scene(
            "Spooning 04",
            SceneId::Exact(pb70_name("Spooning 04")),
            vec![
                ActionRef::new("penetration.vaginal.deep", 
                Strength::Constant(80))
            ],
        ),
        scene(
            "Standing 69",
            SceneId::Exact(pb70_name("Standing 69")),
            vec![
                ActionRef::new("penetration.oral", Strength::Constant(40)),
                ActionRef::new("cunnilungus", Strength::Constant(40))
            ],
        ),
        
        scene(
            "Standing Sequence",
            SceneId::Exact(pb70_name("Standing Sequence Stage 1")),
            vec![],
        ),
    
        scene(
            "Standing Sequence Stage 2",
            SceneId::Exact(pb70_name("Standing Sequence Stage 2")),
            vec![
                ActionRef::new("penetration.vaginal", Strength::Constant(50))
            ],
        ),
        scene(
            "Standing Sequence Stage 3",
            SceneId::Exact(pb70_name("Standing Sequence Stage 3")),
            vec![
                ActionRef::new("penetration.vaginal", Strength::Constant(75))
            ],
        )
    ];
    vec
}