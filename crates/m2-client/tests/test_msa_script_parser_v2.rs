use m2_client::msa_script_parser_v2::*;
use std::ffi::CString;

#[test]
fn test_parse_basic_fields() {
    let script = r#"
        MotionFileName "test_motion.gr2"
        MotionDuration 1.5
        Accumulation 10.5 20.0 30.2
        Group LoopData
        {
            MotionLoopCount 3
        }
    "#;
    
    let data = parse_msa_data(script).unwrap();
    assert_eq!(data.motion_file_name, "test_motion.gr2");
    assert_eq!(data.motion_duration, 1.5);
    assert_eq!(data.loop_count, 3);
    assert_eq!(data.accumulation_data, Some([10.5, 20.0, 30.2]));
}

#[test]
fn test_parse_screen_waving() {
    let script = r#"
        Group MotionEventData
        {
            MotionEventDataCount 1
            Group Event00
            {
                MotionEventType 1
                StartingTime 0.5
                DuringTime 1.0
                Power 50
                AffectingRange 100
            }
        }
    "#;
    
    let data = parse_msa_data(script).unwrap();
    assert_eq!(data.events.len(), 1);
    match &data.events[0] {
        MotionEventData::ScreenWaving { starting_time, duration_time, power, affecting_range } => {
            assert_eq!(*starting_time, 0.5);
            assert_eq!(*duration_time, 1.0);
            assert_eq!(*power, 50);
            assert_eq!(*affecting_range, 100);
        }
        _ => panic!("Expected ScreenWaving event"),
    }
}

#[test]
fn test_parse_screen_flashing() {
    let script = r#"
        Group MotionEventData
        {
            MotionEventDataCount 1
            Group Event00
            {
                MotionEventType 2
                StartingTime 1.2
            }
        }
    "#;
    
    let data = parse_msa_data(script).unwrap();
    assert_eq!(data.events.len(), 1);
    match &data.events[0] {
        MotionEventData::ScreenFlashing { starting_time } => {
            assert_eq!(*starting_time, 1.2);
        }
        _ => panic!("Expected ScreenFlashing event"),
    }
}

#[test]
fn test_parse_effect_and_fly() {
    let script = r#"
        Group MotionEventData
        {
            MotionEventDataCount 2
            Group Event00
            {
                MotionEventType 3
                StartingTime 0.1
                IndependentFlag 1
                AttachingEnable 1
                AttachingBoneName "Bip01 R Hand"
                FollowingEnable 0
                EffectFileName "d:/ymir work/effect/test.mse"
                EffectPosition 0.0 0.0 0.0
            }
            Group Event01
            {
                MotionEventType 4
                StartingTime 0.2
                AttachingEnable 1
                AttachingBoneName "Bip01 Head"
                FlyFileName "d:/ymir work/fly/test.msf"
                FlyPosition 1.0 2.0 3.0
            }
        }
    "#;
    
    let data = parse_msa_data(script).unwrap();
    assert_eq!(data.events.len(), 2);
    match &data.events[0] {
        MotionEventData::Effect { starting_time, independent_flag, attaching_enable, attaching_bone_name, following_enable, effect_file_name, effect_position } => {
            assert_eq!(*starting_time, 0.1);
            assert_eq!(*independent_flag, true);
            assert_eq!(*attaching_enable, true);
            assert_eq!(attaching_bone_name, "Bip01 R Hand");
            assert_eq!(*following_enable, false);
            assert_eq!(effect_file_name, "d:/ymir work/effect/test.mse");
            assert_eq!(*effect_position, [0.0, 0.0, 0.0]);
        }
        _ => panic!("Expected Effect event"),
    }
    match &data.events[1] {
        MotionEventData::Fly { starting_time, attaching_enable, attaching_bone_name, fly_file_name, fly_position } => {
            assert_eq!(*starting_time, 0.2);
            assert_eq!(*attaching_enable, true);
            assert_eq!(attaching_bone_name, "Bip01 Head");
            assert_eq!(fly_file_name, "d:/ymir work/fly/test.msf");
            assert_eq!(*fly_position, [1.0, 2.0, 3.0]);
        }
        _ => panic!("Expected Fly event"),
    }
}

#[test]
fn test_parse_attack() {
    let script = r#"
        Group MotionEventData
        {
            MotionEventDataCount 1
            Group Event00
            {
                MotionEventType 5
                StartingTime 0.5
                DuringTime 0.2
                EnableHitProcess 1
                Group AttackData
                {
                    AttackType 1
                    HitLimit 5
                }
                Group CollisionData
                {
                    SphereRadius 150.0
                }
            }
        }
    "#;
    
    let data = parse_msa_data(script).unwrap();
    assert_eq!(data.events.len(), 1);
    match &data.events[0] {
        MotionEventData::Attack { starting_time, duration_time, enable_hit_process, attack_type, hit_limit, sphere_radius } => {
            assert_eq!(*starting_time, 0.5);
            assert_eq!(*duration_time, 0.2);
            assert_eq!(*enable_hit_process, true);
            assert_eq!(*attack_type, 1);
            assert_eq!(*hit_limit, 5);
            assert_eq!(*sphere_radius, 150.0);
        }
        _ => panic!("Expected Attack event"),
    }
}

#[test]
fn test_parse_sound() {
    let script = r#"
        Group MotionEventData
        {
            MotionEventDataCount 1
            Group Event00
            {
                MotionEventType 6
                StartingTime 0.8
                SoundFileName "d:/ymir work/sound/test.wav"
            }
        }
    "#;
    
    let data = parse_msa_data(script).unwrap();
    assert_eq!(data.events.len(), 1);
    match &data.events[0] {
        MotionEventData::Sound { starting_time, sound_file_name } => {
            assert_eq!(*starting_time, 0.8);
            assert_eq!(sound_file_name, "d:/ymir work/sound/test.wav");
        }
        _ => panic!("Expected Sound event"),
    }
}

#[test]
fn test_m2_msa_parse_ffi() {
    let script = CString::new(r#"
        Group MotionEventData
        {
            MotionEventDataCount 2
            Group Event00
            {
                MotionEventType 1
                StartingTime 0.1
                DuringTime 0.5
                Power 10
                AffectingRange 50
            }
            Group Event01
            {
                MotionEventType 2
                StartingTime 0.5
            }
        }
    "#).unwrap();
    
    unsafe {
        let count = m2_msa_parse(script.as_ptr(), script.as_bytes().len());
        assert_eq!(count, 2);
    }
    
    unsafe {
        let count = m2_msa_parse(std::ptr::null(), 0);
        assert_eq!(count, 0);
    }
}
