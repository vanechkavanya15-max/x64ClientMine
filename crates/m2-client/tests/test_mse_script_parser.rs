use m2_client::mse_script_parser::*;

#[test]
fn test_mse_script_parser_emitter() {
    let script = r#"
        Group EmitterProperty
        {
            MaxEmissionCount    100
            CycleLength         0.1
            CycleLoopEnable     1
            LoopCount           5
            EmitterShape        1
            EmitterAdvancedType 0
            EmittingSize        10.0 20.0 30.0
            EmittingRadius      5.5
            EmittingDirection   0.0 1.0 0.0
            EmitterEmitFromEdgeFlag 1
            
            TimeEventEmittingSize 2 0.0 1.0 1.0 2.0
        }
    "#;

    let parsed = MseEffectData::parse(script).unwrap();
    let e = &parsed.emitter_property;

    assert_eq!(e.max_emission_count, 100);
    assert_eq!(e.cycle_length, 0.1);
    assert_eq!(e.cycle_loop_enable, true);
    assert_eq!(e.loop_count, 5);
    assert_eq!(e.emitter_shape, 1);
    assert_eq!(e.emitting_size, [10.0, 20.0, 30.0]);
    assert_eq!(e.emitting_radius, 5.5);
    assert_eq!(e.emitting_direction, [0.0, 1.0, 0.0]);
    assert_eq!(e.emit_from_edge_flag, true);
    
    assert_eq!(e.time_event_emitting_size.len(), 2);
    assert_eq!(e.time_event_emitting_size[0].time, 0.0);
    assert_eq!(e.time_event_emitting_size[0].value, 1.0);
    assert_eq!(e.time_event_emitting_size[1].time, 1.0);
    assert_eq!(e.time_event_emitting_size[1].value, 2.0);
}

#[test]
fn test_mse_script_parser_particle() {
    let script = r#"
        Group ParticleProperty
        {
            SrcBlendType 5
            DestBlendType 2
            BillboardType 1
            RotationType 2
            RotationSpeed 5.0
            RotationRandomStartingBegin 0
            RotationRandomStartingEnd 360
            AttachEnable 1
            StretchEnable 0
            Gravity 9.8
            AirResistance 0.1
            
            TimeEventScaleX 2 0.0 1.0 1.0 2.0
            TimeEventColorRed 2 0.0 1.0 1.0 0.5
            TimeEventColorGreen 2 0.0 0.0 1.0 0.5
            TimeEventColorBlue 2 0.0 0.0 1.0 0.5
            TimeEventAlpha 2 0.0 1.0 1.0 0.0
            
            TextureFiles 2 "tex1.tga" "tex2.tga"
        }
    "#;

    let parsed = MseEffectData::parse(script).unwrap();
    let p = &parsed.particle_property;

    assert_eq!(p.src_blend_type, 5);
    assert_eq!(p.dest_blend_type, 2);
    assert_eq!(p.billboard_type, 1);
    assert_eq!(p.rotation_speed, 5.0);
    assert_eq!(p.rotation_random_starting_end, 360);
    assert_eq!(p.attach_flag, true);
    assert_eq!(p.stretch_flag, false);
    
    assert_eq!(p.time_event_gravity.len(), 1);
    assert_eq!(p.time_event_gravity[0].value, 9.8);
    
    assert_eq!(p.time_event_air_resistance.len(), 1);
    assert_eq!(p.time_event_air_resistance[0].value, 0.1);
    
    assert_eq!(p.time_event_scale_x.len(), 2);
    assert_eq!(p.time_event_scale_x[1].value, 2.0);

    assert_eq!(p.time_event_color.len(), 2);
    assert_eq!(p.time_event_color[0].r, 1.0);
    assert_eq!(p.time_event_color[0].g, 0.0);
    assert_eq!(p.time_event_color[0].b, 0.0);
    assert_eq!(p.time_event_color[0].a, 1.0);

    assert_eq!(p.time_event_color[1].r, 0.5);
    assert_eq!(p.time_event_color[1].g, 0.5);
    assert_eq!(p.time_event_color[1].b, 0.5);
    assert_eq!(p.time_event_color[1].a, 0.0);

    assert_eq!(p.texture_files.len(), 2);
    assert_eq!(p.texture_files[0], "tex1.tga");
    assert_eq!(p.texture_files[1], "tex2.tga");
}

#[test]
fn test_mse_script_parser_defaults() {
    let script = r#"
        Group EmitterProperty
        {
        }
        Group ParticleProperty
        {
        }
    "#;

    let parsed = MseEffectData::parse(script).unwrap();
    let e = &parsed.emitter_property;
    let p = &parsed.particle_property;

    assert_eq!(e.cycle_length, 0.05);
    assert_eq!(e.cycle_loop_enable, false);
    assert_eq!(p.src_blend_type, 5);
    assert_eq!(p.dest_blend_type, 2);
    assert_eq!(p.color_operation_type, 4);
    assert_eq!(p.texture_files.len(), 0);
}

#[test]
fn test_mse_script_parser_empty() {
    let script = "";
    let res = MseEffectData::parse(script);
    assert!(res.is_err());
    assert_eq!(res.err().unwrap(), "Empty script");
}

#[test]
fn test_mse_script_parser_color_blend_missing_channels() {
    let script = r#"
        Group ParticleProperty
        {
            TimeEventColorRed 2 0.0 1.0 1.0 0.5
        }
    "#;

    let parsed = MseEffectData::parse(script).unwrap();
    let p = &parsed.particle_property;

    assert_eq!(p.time_event_color.len(), 2);
    assert_eq!(p.time_event_color[0].r, 1.0);
    assert_eq!(p.time_event_color[0].g, 0.0); // missing channels default to 0
}

#[test]
fn test_mse_script_parser_time_event_interpolation() {
    // Tests get_time_event_blend_value implicitly via parsed color
    let script = r#"
        Group ParticleProperty
        {
            TimeEventColorRed 3 0.0 0.0 0.5 1.0 1.0 0.0
            TimeEventAlpha 2 0.25 1.0 0.75 0.0
        }
    "#;

    let parsed = MseEffectData::parse(script).unwrap();
    let p = &parsed.particle_property;
    
    // Times involved: 0.0, 0.25, 0.5, 0.75, 1.0
    assert_eq!(p.time_event_color.len(), 5);
    
    // time = 0.0
    assert_eq!(p.time_event_color[0].time, 0.0);
    assert_eq!(p.time_event_color[0].r, 0.0);
    assert_eq!(p.time_event_color[0].a, 1.0); // 0.0 <= 0.25, so alpha = 1.0
    
    // time = 0.25
    assert_eq!(p.time_event_color[1].time, 0.25);
    assert_eq!(p.time_event_color[1].r, 0.5); // mid point between 0.0 (val 0.0) and 0.5 (val 1.0)
    assert_eq!(p.time_event_color[1].a, 1.0); 
    
    // time = 0.5
    assert_eq!(p.time_event_color[2].time, 0.5);
    assert_eq!(p.time_event_color[2].r, 1.0);
    assert_eq!(p.time_event_color[2].a, 0.5); // mid point between 0.25 (val 1.0) and 0.75 (val 0.0)

    // time = 0.75
    assert_eq!(p.time_event_color[3].time, 0.75);
    assert_eq!(p.time_event_color[3].r, 0.5); // mid point between 0.5 (val 1.0) and 1.0 (val 0.0)
    assert_eq!(p.time_event_color[3].a, 0.0);

    // time = 1.0
    assert_eq!(p.time_event_color[4].time, 1.0);
    assert_eq!(p.time_event_color[4].r, 0.0);
    assert_eq!(p.time_event_color[4].a, 0.0);
}

#[test]
fn test_mse_script_parser_native_metin2_syntax() {
    let script = r#"
        Group EmitterProperty
        {
            MaxEmissionCount    50
            TimeEventEmittingSize 0.0 2.5 0.5 5.0 1.0 10.0
        }
        Group ParticleProperty
        {
            TimeEventGravity 0.0 100.0 1.0 200.0
            TextureFiles { "d:/ymir work/effect/etc/fire01.dds" "d:/ymir work/effect/etc/fire02.dds" }
        }
    "#;

    let parsed = MseEffectData::parse(script).unwrap();
    assert_eq!(parsed.emitter_property.max_emission_count, 50);
    assert_eq!(parsed.emitter_property.time_event_emitting_size.len(), 3);
    assert_eq!(parsed.emitter_property.time_event_emitting_size[1].time, 0.5);
    assert_eq!(parsed.emitter_property.time_event_emitting_size[1].value, 5.0);
    assert_eq!(parsed.particle_property.time_event_gravity.len(), 2);
    assert_eq!(parsed.particle_property.texture_files.len(), 2);
    assert_eq!(parsed.particle_property.texture_files[0], "d:/ymir work/effect/etc/fire01.dds");
}
