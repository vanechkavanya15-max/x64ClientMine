#[path = "../src/effect_mse_script_parser.rs"]
mod effect_mse_script_parser;
use effect_mse_script_parser::{EffectData, ScriptParser};

#[test]
fn test_mse_parser_particle_block() {
    let script = r#"
Group Particle
{
    StartTime 0.123
    List TimeEventPosition
    {
        0.0 MOVING_TYPE_DIRECT 1.0 2.0 3.0
    }

    Group EmitterProperty
    {
        MaxEmissionCount 10
        CycleLength 2.5
        CycleLoopEnable 1
        LoopCount 5
        EmitterShape 1
        EmitterEmitFromEdgeFlag 0
        EmittingSize 10.0 20.0 30.0
        EmittingDirection 0.0 0.0 1.0
        TimeEventEmittingSize 0.0 1.0 1.0 2.0
        TimeEventLifeTime 0.0 5.0
    }

    Group ParticleProperty
    {
        SrcBlendType 5
        DestBlendType 2
        ColorOperationType 4
        BillboardType 1
        RotationType 2
        RotationSpeed 3.14
        AttachEnable 1
        StretchEnable 0
        TexAniType 1
        TexAniDelay 0.1
        TexAniRandomStartFrameEnable 0
        
        Gravity 9.81
        AirResistance 0.5
        
        TimeEventScaleX 0.0 1.0 1.0 2.0
        TimeEventScaleY 0.0 1.0 1.0 2.0
        
        TimeEventColorRed 0.0 1.0 1.0 0.5
        TimeEventColorGreen 0.0 0.5 1.0 0.2
        TimeEventColorBlue 0.0 0.0 1.0 1.0
        TimeEventAlpha 0.0 1.0 1.0 0.0
        
        TimeEventRotation 0.0 0.0 1.0 180.0
        
        List TextureFiles
        {
            "d:/ymir work/effect/bg/spark.tga"
            "d:/ymir work/effect/bg/spark2.tga"
        }
    }
}
    "#;

    let parsed = ScriptParser::parse(script).expect("Failed to parse particle script");
    assert_eq!(parsed.particles.len(), 1);
    
    let p = &parsed.particles[0];
    assert_eq!(p.start_time, 0.123);
    assert_eq!(p.time_event_table_position.len(), 1);
    assert_eq!(p.time_event_table_position[0].time, 0.0);
    assert_eq!(p.time_event_table_position[0].value, [1.0, 2.0, 3.0]);
    assert_eq!(p.time_event_table_position[0].moving_type, 0);

    let ep = &p.emitter_property;
    assert_eq!(ep.max_emission_count, 10);
    assert_eq!(ep.cycle_length, 2.5);
    assert_eq!(ep.cycle_loop_flag, true);
    assert_eq!(ep.loop_count, 5);
    assert_eq!(ep.emitter_shape, 1);
    assert_eq!(ep.emit_from_edge_flag, false);
    assert_eq!(ep.emitting_size, [10.0, 20.0, 30.0]);
    assert_eq!(ep.emitting_direction, [0.0, 0.0, 1.0]);
    assert_eq!(ep.time_event_emitting_size.len(), 2);
    assert_eq!(ep.time_event_emitting_size[1].time, 1.0);
    assert_eq!(ep.time_event_emitting_size[1].value, 2.0);
    assert_eq!(ep.time_event_life_time.len(), 1);
    assert_eq!(ep.time_event_life_time[0].value, 5.0);

    let pp = &p.particle_property;
    assert_eq!(pp.src_blend_type, 5);
    assert_eq!(pp.dest_blend_type, 2);
    assert_eq!(pp.color_operation_type, 4);
    assert_eq!(pp.billboard_type, 1);
    assert_eq!(pp.rotation_type, 2);
    assert_eq!(pp.rotation_speed, 3.14);
    assert_eq!(pp.attach_flag, true);
    assert_eq!(pp.stretch_flag, false);
    assert_eq!(pp.tex_ani_type, 1);
    assert_eq!(pp.tex_ani_delay, 0.1);
    assert_eq!(pp.tex_ani_random_start_frame_flag, false);
    assert_eq!(pp.time_event_gravity[0].value, 9.81);
    assert_eq!(pp.time_event_air_resistance[0].value, 0.5);
    
    assert_eq!(pp.time_event_color.len(), 2);
    assert_eq!(pp.time_event_color[0].time, 0.0);
    assert_eq!(pp.time_event_color[0].value, [1.0, 0.5, 0.0, 1.0]);
    assert_eq!(pp.time_event_color[1].time, 1.0);
    assert_eq!(pp.time_event_color[1].value, [0.5, 0.2, 1.0, 0.0]);
    
    assert_eq!(pp.time_event_scale_x.len(), 2);
    assert_eq!(pp.time_event_rotation.len(), 2);
    
    assert_eq!(pp.texture_files.len(), 2);
    assert_eq!(pp.texture_files[0], "d:/ymir work/effect/bg/spark.tga");
}

#[test]
fn test_mse_parser_mesh_block() {
    let script = r#"
Group Mesh
{
    StartTime 0.5
    MeshFileName "d:/ymir work/effect/bg/sword_effect.mde"
    MeshAnimationLoopEnable 1
    MeshAnimationLoopCount 0
    MeshAnimationFrameDelay 0.03
    MeshElementCount 2
    
    Group 0
    {
        BillboardType 1
        BlendingEnable 1
        BlendingSrcType 5
        BlendingDestType 2
        TextureAnimationLoopEnable 1
        TextureAnimationFrameDelay 0.1
        ColorOperationType 4
        ColorFactor 1.0 1.0 1.0 1.0
        TimeEventAlpha 0.0 1.0 1.0 0.5
    }
    
    Group 1
    {
        BillboardType 0
        BlendingEnable 0
        ColorOperationType 4
        ColorFactor 0.5 0.5 0.5 1.0
    }
}
    "#;

    let parsed = ScriptParser::parse(script).expect("Failed to parse mesh script");
    assert_eq!(parsed.meshes.len(), 1);
    
    let m = &parsed.meshes[0];
    assert_eq!(m.start_time, 0.5);
    assert_eq!(m.mesh_file_name, "d:/ymir work/effect/bg/sword_effect.mde");
    assert_eq!(m.mesh_animation_loop_enable, true);
    assert_eq!(m.mesh_animation_loop_count, 0);
    assert_eq!(m.mesh_animation_frame_delay, 0.03);
    assert_eq!(m.mesh_data_vector.len(), 2);
    
    let md0 = &m.mesh_data_vector[0];
    assert_eq!(md0.billboard_type, 1);
    assert_eq!(md0.blending_enable, true);
    assert_eq!(md0.blending_src_type, 5);
    assert_eq!(md0.blending_dest_type, 2);
    assert_eq!(md0.texture_animation_loop_enable, true);
    assert_eq!(md0.texture_animation_frame_delay, 0.1);
    assert_eq!(md0.color_operation_type, 4);
    assert_eq!(md0.color_factor, [1.0, 1.0, 1.0, 1.0]);
    assert_eq!(md0.time_event_alpha.len(), 2);
    assert_eq!(md0.time_event_alpha[1].value, 0.5);
    
    let md1 = &m.mesh_data_vector[1];
    assert_eq!(md1.billboard_type, 0);
    assert_eq!(md1.blending_enable, false);
    assert_eq!(md1.color_factor, [0.5, 0.5, 0.5, 1.0]);
}

#[test]
fn test_mse_parser_light_block() {
    let script = r#"
Group Light
{
    StartTime 0.0
    Duration 2.0
    LoopFlag 1
    LoopCount 3
    AmbientColor 0.1 0.1 0.1 1.0
    DiffuseColor 1.0 0.0 0.0 1.0
    MaxRange 500.0
    Attenuation0 1.0
    Attenuation1 0.5
    Attenuation2 0.0
    TimeEventRange 0.0 0.0 1.0 1.0 2.0 0.0
}
    "#;

    let parsed = ScriptParser::parse(script).expect("Failed to parse light script");
    assert_eq!(parsed.lights.len(), 1);
    
    let l = &parsed.lights[0];
    assert_eq!(l.start_time, 0.0);
    assert_eq!(l.duration, 2.0);
    assert_eq!(l.loop_flag, true);
    assert_eq!(l.loop_count, 3);
    assert_eq!(l.ambient_color, [0.1, 0.1, 0.1, 1.0]);
    assert_eq!(l.diffuse_color, [1.0, 0.0, 0.0, 1.0]);
    assert_eq!(l.max_range, 500.0);
    assert_eq!(l.attenuation0, 1.0);
    assert_eq!(l.attenuation1, 0.5);
    assert_eq!(l.attenuation2, 0.0);
    assert_eq!(l.time_event_range.len(), 3);
    assert_eq!(l.time_event_range[0].value, 0.0);
    assert_eq!(l.time_event_range[1].value, 1.0);
    assert_eq!(l.time_event_range[2].value, 0.0);
}

#[test]
fn test_mse_parser_full_effect() {
    let script = r#"
BoundingSphereRadius 150.0
BoundingSpherePosition 0.0 0.0 50.0

Group Particle
{
    StartTime 0.0
}

Group Mesh
{
    StartTime 0.0
}

Group Light
{
    StartTime 0.0
}
    "#;
    
    let parsed = ScriptParser::parse(script).expect("Failed to parse full script");
    assert_eq!(parsed.bounding_sphere_radius, 150.0);
    assert_eq!(parsed.bounding_sphere_position, [0.0, 0.0, 50.0]);
    assert_eq!(parsed.particles.len(), 1);
    assert_eq!(parsed.meshes.len(), 1);
    assert_eq!(parsed.lights.len(), 1);
}
