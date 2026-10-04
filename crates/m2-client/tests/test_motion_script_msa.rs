use m2_client::motion_script_msa_parser::{parse_msa_script, MotionEventData};

#[test]
fn test_msa_basic_duration_accumulation() {
    let script = r#"
        MotionDuration 2.5
        Accumulation 10.0 20.0 30.0
    "#;
    let res = parse_msa_script(script).expect("Parsing failed");
    assert_eq!(res.motion_duration, 2.5);
    assert_eq!(res.accumulation, Some([10.0, 20.0, 30.0]));
}

#[test]
fn test_msa_events_parsing() {
    let script = r#"
        Group MotionEventData
        {
            Group Event00
            {
                MotionEventType 5
                StartingTime 0.1
                SoundFileName "sound/effect.wav"
            }
            Group Event01
            {
                MotionEventType 1
                StartingTime 0.5
                AttachingBoneName "Bip01 Head"
                EffectFileName "d:/ymir work/effect/skill.mse"
            }
            Group Event02
            {
                MotionEventType 6
                StartingTime 0.8
                FlyFileName "d:/ymir work/effect/fly.mse"
            }
        }
    "#;
    let res = parse_msa_script(script).expect("Parsing failed");
    assert_eq!(res.events.len(), 3);
    
    match &res.events[0] {
        MotionEventData::Sound { frame, file } => {
            assert_eq!(*frame, 3); // 0.1 / (1/30) = 3
            assert_eq!(file, "sound/effect.wav");
        }
        _ => panic!("Expected Sound event"),
    }
    
    match &res.events[1] {
        MotionEventData::Effect { target_bone, file } => {
            assert_eq!(target_bone, "Bip01 Head");
            assert_eq!(file, "d:/ymir work/effect/skill.mse");
        }
        _ => panic!("Expected Effect event"),
    }
    
    match &res.events[2] {
        MotionEventData::Fly { file } => {
            assert_eq!(file, "d:/ymir work/effect/fly.mse");
        }
        _ => panic!("Expected Fly event"),
    }
}

#[test]
fn test_msa_warrior_combat_script() {
    let script = r#"
        ScriptType MotionData

        MotionFileName "d:/ymir work/pc/warrior/action/combo_01.gr2"
        MotionDuration 1.2
        Accumulation 50.0 0.0 0.0

        Group MotionEventData
        {
            MotionEventDataCount 2
            Group Event00
            {
                MotionEventType 5
                StartingTime 0.0
                SoundFileName "sound/pc/warrior/action/combo_01.wav"
            }
            Group Event01
            {
                MotionEventType 1
                StartingTime 0.2
                AttachingEnable 1
                AttachingBoneName "Bip01 R Hand"
                EffectFileName "d:/ymir work/effect/pc/warrior/combo_01.mse"
            }
        }
    "#;
    let res = parse_msa_script(script).expect("Parsing failed");
    assert_eq!(res.motion_duration, 1.2);
    assert_eq!(res.accumulation, Some([50.0, 0.0, 0.0]));
    assert_eq!(res.events.len(), 2);
}
