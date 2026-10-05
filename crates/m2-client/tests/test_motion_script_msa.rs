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
            MotionEventDataCount 3
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
                AttachingEnable 1
                AttachingBoneName "Bip01 Head"
                EffectFileName "d:/ymir work/effect/skill.mse"
                EffectPosition 1.0 2.0 3.0
                FollowingEnable 0
            }
            Group Event02
            {
                MotionEventType 6
                StartingTime 0.8
                FlyFileName "d:/ymir work/effect/fly.mse"
                AttachingEnable 0
                AttachingBoneName "Bip01 R Hand"
            }
            Group Event03
            {
                MotionEventType 2
                StartingTime 1.0
                DuringTime 0.5
                Power 10
                AffectingRange 500
            }
        }
    "#;
    let res = parse_msa_script(script).expect("Parsing failed");
    assert_eq!(res.events.len(), 4);
    
    match &res.events[0] {
        MotionEventData::Sound { frame, starting_time, file_name } => {
            assert_eq!(*frame, 6); // (0.1 / (1/60.0)).round() = 6
            assert_eq!(*starting_time, 0.1);
            assert_eq!(file_name, "sound/effect.wav");
        }
        _ => panic!("Expected Sound event"),
    }
    
    match &res.events[1] {
        MotionEventData::Effect { frame, starting_time, is_attaching, is_following, attaching_bone_name, effect_file_name, effect_position, .. } => {
            assert_eq!(*frame, 30); // 0.5 * 60 = 30
            assert_eq!(*starting_time, 0.5);
            assert_eq!(*is_attaching, true);
            assert_eq!(*is_following, false);
            assert_eq!(attaching_bone_name, "Bip01 Head");
            assert_eq!(effect_file_name, "d:/ymir work/effect/skill.mse");
            assert_eq!(*effect_position, [1.0, 2.0, 3.0]);
        }
        _ => panic!("Expected Effect event"),
    }
    
    match &res.events[2] {
        MotionEventData::Fly { frame, starting_time, fly_file_name, attaching_bone_name, is_attaching, .. } => {
            assert_eq!(*frame, 48); // 0.8 * 60 = 48
            assert_eq!(*starting_time, 0.8);
            assert_eq!(*is_attaching, false);
            assert_eq!(attaching_bone_name, "Bip01 R Hand");
            assert_eq!(fly_file_name, "d:/ymir work/effect/fly.mse");
        }
        _ => panic!("Expected Fly event"),
    }

    match &res.events[3] {
        MotionEventData::ScreenWaving { frame, starting_time, duration_time, power, affecting_range } => {
            assert_eq!(*frame, 60);
            assert_eq!(*starting_time, 1.0);
            assert_eq!(*duration_time, 0.5);
            assert_eq!(*power, 10);
            assert_eq!(*affecting_range, 500);
        }
        _ => panic!("Expected ScreenWaving event"),
    }
}

#[test]
fn test_msa_attacking_data() {
    let script = r#"
        Group AttackingData
        {
            AttackType 0
            HittingType 1
            StiffenTime 0.1
            InvisibleTime 0.2
            ExternalForce 15.0
            HitLimitCount 3
            MotionType 2
            
            HitDataCount 1
            Group HitData00
            {
                AttackingStartTime 0.1
                AttackingEndTime 0.5
                AttackingBone "Bip01 R Hand"
                WeaponLength 150.0
                HitPosition 0.1 0 0 0 10 10 10 0.2 10 10 10 20 20 20
            }
        }
    "#;
    let res = parse_msa_script(script).expect("Parsing failed");
    
    let attack_data = res.attacking_data.expect("Expected attacking data");
    assert_eq!(attack_data.attack_type, 0);
    assert_eq!(attack_data.hitting_type, 1);
    assert_eq!(attack_data.stiffen_time, 0.1);
    assert_eq!(attack_data.invisible_time, 0.2);
    assert_eq!(attack_data.external_force, 15.0);
    assert_eq!(attack_data.hit_limit_count, 3);
    assert_eq!(attack_data.motion_type, 2);
    
    assert_eq!(attack_data.hit_data.len(), 1);
    let hit = &attack_data.hit_data[0];
    assert_eq!(hit.attack_start_time, 0.1);
    assert_eq!(hit.attack_end_time, 0.5);
    assert_eq!(hit.bone_name, "Bip01 R Hand");
    assert_eq!(hit.weapon_length, 150.0);
    
    assert_eq!(hit.hit_positions.len(), 2);
    assert_eq!(hit.hit_positions[0].time, 0.1);
    assert_eq!(hit.hit_positions[0].last_position, [0.0, 0.0, 0.0]);
    assert_eq!(hit.hit_positions[0].position, [10.0, 10.0, 10.0]);

    assert_eq!(hit.hit_positions[1].time, 0.2);
    assert_eq!(hit.hit_positions[1].last_position, [10.0, 10.0, 10.0]);
    assert_eq!(hit.hit_positions[1].position, [20.0, 20.0, 20.0]);
}

#[test]
fn test_msa_combo_loop_and_special_attacking_event() {
    let script = r#"
        Group ComboInputData
        {
            PreInputTime 0.1
            DirectInputTime 0.3
            InputLimitTime 0.5
        }

        Group LoopData
        {
            MotionLoopCount -1
            LoopCancelEnable 1
            LoopStartTime 0.1
            LoopEndTime 0.9
        }

        Group MotionEventData
        {
            Group Event00
            {
                MotionEventType 4
                StartingTime 0.2
                DuringTime 0.5
                EnableHitProcess 1
                
                AttackType 1
                HittingType 0
                StiffenTime 0.15
                InvisibleTime 0.0
                ExternalForce 0.0
                HitLimitCount 1
                
                HitDataCount 1
                Group HitData00
                {
                    AttackingStartTime 0.2
                    AttackingEndTime 0.4
                    AttackingBone "Bip01 R Foot"
                }
                
                CollisionType 3
                SphereDataCount 2
                Group SphereData00
                {
                    Radius 50.0
                    Position 10.0 20.0 30.0
                }
                Group SphereData01
                {
                    Radius 25.0
                    Position 0.0 0.0 5.0
                }
            }
        }
    "#;
    let res = parse_msa_script(script).expect("Parsing failed");
    
    let combo = res.combo_input_data.expect("Expected Combo");
    assert_eq!(combo.pre_input_time, 0.1);
    assert_eq!(combo.next_combo_time, 0.3);
    assert_eq!(combo.input_end_time, 0.5);

    let loop_data = res.loop_data.expect("Expected Loop");
    assert_eq!(loop_data.motion_loop_count, -1);
    assert_eq!(loop_data.loop_cancel_enable, true);
    assert_eq!(loop_data.loop_start_time, 0.1);
    assert_eq!(loop_data.loop_end_time, 0.9);

    assert_eq!(res.events.len(), 1);
    match &res.events[0] {
        MotionEventData::SpecialAttacking { frame, starting_time, duration_time, is_enable_hit_process, attack_data, collision_data } => {
            assert_eq!(*frame, 12); // 0.2 * 60 = 12
            assert_eq!(*starting_time, 0.2);
            assert_eq!(*duration_time, 0.5);
            assert_eq!(*is_enable_hit_process, true);
            
            assert_eq!(attack_data.attack_type, 1);
            assert_eq!(attack_data.stiffen_time, 0.15);
            assert_eq!(attack_data.hit_limit_count, 1);
            assert_eq!(attack_data.hit_data.len(), 1);
            assert_eq!(attack_data.hit_data[0].attack_start_time, 0.2);
            assert_eq!(attack_data.hit_data[0].attack_end_time, 0.4);
            assert_eq!(attack_data.hit_data[0].bone_name, "Bip01 R Foot");
            
            let col = collision_data.as_ref().expect("Expected collision data");
            assert_eq!(col.collision_type, 3);
            assert_eq!(col.sphere_data.len(), 2);
            assert_eq!(col.sphere_data[0].radius, 50.0);
            assert_eq!(col.sphere_data[0].position, [10.0, 20.0, 30.0]);
            assert_eq!(col.sphere_data[1].radius, 25.0);
            assert_eq!(col.sphere_data[1].position, [0.0, 0.0, 5.0]);
        }
        _ => panic!("Expected SpecialAttacking event"),
    }
}
