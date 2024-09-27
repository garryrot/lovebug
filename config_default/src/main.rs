use std::fs::{self};

use bp_scheduler::config::actions::*;
use config::*;
use serde::Serialize;

fn main() {
    let config_dir = "deploy/Data/F4SE/Plugins/Lovebug";

    let triggers = vec![("Default.json", get_default_trigger())];
    let actions = vec![
        ("Default.json", get_default_actions()),
        ("BodyParts.json", get_body_actions()),
    ];

    for trigger in triggers {
        let path = format!("../{}/Triggers/{}", config_dir, trigger.0);
        write_file(path, trigger.1);
    }
    for action in actions {
        let path = format!("../{}/Actions/{}", config_dir, action.0);
        write_file(path, action.1);
    }
}

fn get_default_trigger() -> Vec<Trigger> {
    let default_config: Vec<Trigger> = vec![Trigger::Scene(Scene {
        enabled: true,
        description: "Scene Default".into(),
        framework: Framework::All,
        scene_id: SceneId::Any,
        tags: SceneTags::Any,
        actions: vec![
            ActionRef::new(
                "vibrate",
                Strength::RandomFunscript(50, vec!["Blowjob".into(), "Deepthroat".into()]),
            ),
            ActionRef::new("linear.stroke", Strength::Funscript(50, "Blowjob".into())),
            ActionRef::new("constrict", Strength::Constant(50)),
            ActionRef::new("oscillate", Strength::Constant(50)),
        ],
    })];

    /*
        Body Parts:
            Nipple,
            Penis,
            Vaginal,
            Anal
            Clitoral?

        

        BP70 - Blowjob
                            Male: Vibration Penis,  80
                                  Stroke Penis      80

        BP70 - Cowgirl
                            Female: Penetration Vaginal
                                    Vibratino Vaginal 
                            
                              Male: Penetration Penis             80
                                    Vibratino Penis

        BP70 - Cowgirl Sequence
                Cowgirl Sequence 1:
                    Stroking Dick of Male       20
                        Stroking 20%    PENIS

                Cowgirl Sequence 2:
                    Riding Dick of Male         25
                        Stroking 30%    PENIS
                        Vibrate  30%    PENIS
                        Constrict 20%   PENIS
                
                Cowgirl Sequence 3:
                    Riding Dick of Male Faster  50
                        Stroking 50%    PENIS
                        Vibrate 50%     PENIS
                        Constrict 30%   PENIS
    
                Cowgirl Sequence 4:
                    Riding Dick of Male Faster  75
                        Stroking 75%    PENIS
                        Vibrate 50%     PENIS
                        Constrict 50%   PENIS

     [UAP] BP70 - Cowgirl Sequence 5:
                    Riding Dick of Male Faster  100
                        Stroking 90%    PENIS
                        Vibrate 100%    PENIS
                        Constrict 75%   PENIS
                
        BP70 - Cunnilungus
                    Male: Masturbating
                    Female: Pussy Licking Passive, Nipple Stroking Passive

                        Vibrate Vaginal 60%
                        Vibrate Nipple  20%
                        Stroke Penis 30%
                        Vibrate Penis 20%


        BP70 - Doggy
                    Male: Fucking Female from behind

                        Vibrate Penis, Vagina   70%
                        Stroke Penis, Vagina    70%
                        Oscillate Penis, Vagina 70%
                        Constrict Penis, Vagina 70%

        BP70 - Footjob
                    Female footjobbing male nothing else
                        
                        Vibrate Penis, Vagina   25%
                        Stroke Penis, Vagina    25%
                        Oscillate Penis, Vagina 25%

        BP70 - Footjob Fast
                    Female footjobbing male nothing else
                        
                        Vibrate Penis, Vagina   60%
                        Stroke Penis, Vagina    75%
                        Oscillate Penis, Vagina 60%

        BP70 - Impregnate Cowgirl Kiss
                    Female % Male Kissing

                        Constrict 20%
        
        BP70 - Impregnate Cowgirl Start Fucking
                    
                        Stroke, Penis Vagina    30%
                        Vibrate, Penis Vagina
                        Constrict, Penis Vagina
                        Oscillate, Penis Vagina

        BP70 - Impregnate Cowgirl Fuck Faster and Deeper

                        Stroke, Penis Vagina    60%
                        Vibrate, Penis Vagina
                        Constrict, Penis Vagina
                        Oscillate, Penis Vagina

        BP70 - Impregnate Cowgirl Thrust Into Her

                        Stroke, Penis Vagina    80%
                        Vibrate, Penis Vagina
                        Constrict, Penis Vagina
                        Oscillate, Penis Vagina

        BP70 - Impregnate Cowgirl Orgasm

                        Stroke, Penis Vagina    100%
                        Vibrate, Penis Vagina
                        Constrict, Penis Vagina
                        Oscillate, Penis Vagina

        BP70 - Impregnate Missionary Stage Kiss

                        Constrict Penis/Vagina 20%

        BP70 - Impregnate Missionary Stage Grab His Back

                        Stroke, Penis Vagina    60%
                        Vibrate, Penis Vagina
                        Constrict, Penis Vagina
                        Oscillate, Penis Vagina


        BP70 - Impregnate Missionary Stage Grab His Back 2

                        Stroke, Penis Vagina    70%
                        Vibrate, Penis Vagina
                        Constrict, Penis Vagina
                        Oscillate, Penis Vagina

        BP70 - Impregnate Missionary Stage Grab His Back 3

                        Stroke, Penis Vagina    90%
                        Vibrate, Penis Vagina
                        Constrict, Penis Vagina
                        Oscillate, Penis Vagina
        

        BP70 - Impregnate Missionary Stage Grab His Back 4

                        Stroke, Penis Vagina    100%
                        Vibrate, Penis Vagina
                        Constrict, Penis Vagina
                        Oscillate, Penis Vagina
        
        BP70 - Kissing
                - Nix

        BP70 - Laying Blowjob
                - Stroke Penis      70%
                - Vibrate Penis
                - Constrict Penis
                - Oscillate Penis

        BP70 - Leg Wrap Cunnilungus
                - Vibrate, Constrict Vaginal

        BP70 - Lotus
                - Stroke, Vibrate, Constrict, Oscillate Penis, Vaginl 60%

        BP70 - Missionary
                - Stroke, Vibrate, Constrict, Oscillate Penis, Vaginl 60%

        BP70 - Missionary Grind (tease)
                - Stroke, Vibrate, Constrict, Oscillate Penis, Vaginl 10%

        BP70 - Missionary Grind 2
                - Stroke, Vibrate, Constrict, Oscillate Penis, Vaginl 40%

        BP70 - Missionary Grind 3
                - Stroke, Vibrate, Constrict, Oscillate Penis, Vaginl 60%

        BP70 - Missionary Grind 4
                - Stroke, Vibrate, Constrict, Oscillate Penis, Vaginl 80%

        BP70 - Missionary Sequence Stage 1
                - Stroke Penis
                - Vibrate Vaginal 33

        BP70 - Missionary Sequence Stage 2
                - Stroke, Vibrate, Constrict, Oscillate, Penis, Vaginal 66

            BP70 - Missionary Sequence Stage 2
            - Stroke, Vibrate, Constrict, Oscillate, Penis, Vaginal 80

        BP70 - Mistress
                - Stroke, Oscillate, Vibrate, Constrict Penis 40
    
        BP70 - Mistress 2
                - Stroke, Oscillate, Vibrate, Constrict Penis 80

        BP70 - Mistress 3
                - Vibrate, Constrict, Vaginal 40

        BP70 - Mistress 4
            - Stroke, Vibrate, Constrict, Oscillate, Penis, Vaginal 50

        BP70 - Mistress 5
            - Stroke, Vibrate, Constrict, Oscillate, Penis, Vaginal 80

        BP70 - Pit Doggy
        BP70 - Prone Bone
        BP70 - Romantic Cowgirl
        BP70 - Romantic Missionary
        BP70 - Spooning
        BP70 - Standing 69
        BP70 - Standing Sequence



    
     */

    default_config
}

fn get_default_actions() -> Vec<Action> {
    vec![
        Action::build(
            "vibrate",
            vec![Control::Scalar(
                Selector::All,
                vec![ScalarActuators::Vibrate],
            )],
        ),
        Action::build(
            "constrict",
            vec![Control::Scalar(
                Selector::All,
                vec![ScalarActuators::Constrict],
            )],
        ),
        Action::build(
            "inflate",
            vec![Control::Scalar(
                Selector::All,
                vec![ScalarActuators::Inflate],
            )],
        ),
        Action::build(
            "scalar",
            vec![Control::Scalar(
                Selector::All,
                vec![
                    ScalarActuators::Vibrate,
                    ScalarActuators::Constrict,
                    ScalarActuators::Oscillate,
                    ScalarActuators::Inflate,
                ],
            )],
        ),
        Action::build(
            "linear",
            vec![Control::Stroke(
                Selector::All,
                StrokeRange {
                    min_ms: 100,
                    max_ms: 1500,
                    min_pos: 0.0,
                    max_pos: 1.0,
                },
            )],
        ),
        Action::build(
            "oscillate",
            vec![Control::Scalar(
                Selector::All,
                vec![ScalarActuators::Oscillate],
            )],
        ),
    ]
}

fn get_body_actions() -> Vec<Action> {
    vec![
        Action::build(
            "vibrate.nipple",
            vec![Control::Scalar(
                Selector::BodyParts(vec!["nipple".into()]),
                vec![ScalarActuators::Vibrate],
            )],
        ),
        Action::build(
            "vibrate.vaginal",
            vec![Control::Scalar(
                Selector::BodyParts(vec!["vaginal".into()]),
                vec![ScalarActuators::Vibrate],
            )],
        ),
        Action::build(
            "vibrate.clitoral",
            vec![Control::Scalar(
                Selector::BodyParts(vec!["clitoral".into()]),
                vec![ScalarActuators::Vibrate],
            )],
        ),
        Action::build(
            "vibrate.anal",
            vec![Control::Scalar(
                Selector::BodyParts(vec!["anal".into()]),
                vec![ScalarActuators::Vibrate],
            )],
        ),
        Action::build(
            "constrict.vaginal",
            vec![Control::Scalar(
                Selector::BodyParts(vec!["constrict".into()]),
                vec![ScalarActuators::Inflate],
            )],
        ),
        Action::build(
            "constrict.anal",
            vec![Control::Scalar(
                Selector::BodyParts(vec!["constrict".into()]),
                vec![ScalarActuators::Inflate],
            )],
        ),
        Action::build(
            "inflate.vaginal",
            vec![Control::Scalar(
                Selector::BodyParts(vec!["vaginal".into()]),
                vec![ScalarActuators::Inflate],
            )],
        ),
        Action::build(
            "inflate.anal",
            vec![Control::Scalar(
                Selector::BodyParts(vec!["anal".into()]),
                vec![ScalarActuators::Inflate],
            )],
        ),
    ]
}

fn write_file<T>(file: String, content: T)
where
    T: Serialize,
    T: Clone,
{
    let _ = fs::remove_file(file.clone());
    fs::write(file, serde_json::to_string_pretty(&content).unwrap()).unwrap();
}
