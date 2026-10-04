#[path = "../src/monster_script_msm_parser.rs"]
mod parser;

use parser::*;

#[test]
fn test_parse_top_level_keys() {
    let content = r#"
        BaseModelFileName "D:\ymir work\pc\warrior\warrior_novice.GR2"
        TreeFileName "D:\ymir work\pc\warrior\warrior_tree.txt"
        AttributeFileName "D:\ymir work\pc\warrior\warrior_attribute.txt"
        SmokeBoneName "Bip01 Spine"
        MotionListFileName "D:\ymir work\pc\warrior\warrior_motion.txt"
        
        SmokeFileName 
        {
            0 "d:\ymir work\effect\smoke0.mse"
            1 "d:\ymir work\effect\smoke1.mse"
        }
    "#;

    let parsed = ScriptParser::parse(content).expect("Failed to parse");
    
    assert_eq!(parsed.base_model_file_name, "D:\\ymir work\\pc\\warrior\\warrior_novice.GR2");
    assert_eq!(parsed.tree_file_name, "D:\\ymir work\\pc\\warrior\\warrior_tree.txt");
    assert_eq!(parsed.attribute_file_name, "D:\\ymir work\\pc\\warrior\\warrior_attribute.txt");
    assert_eq!(parsed.smoke_bone_name, "Bip01 Spine");
    assert_eq!(parsed.motion_list_file_name, "D:\\ymir work\\pc\\warrior\\warrior_motion.txt");

    assert_eq!(parsed.smoke_effects.len(), 2);
    assert_eq!(parsed.smoke_effects[0], (0, "d:\\ymir work\\effect\\smoke0.mse".to_string()));
    assert_eq!(parsed.smoke_effects[1], (1, "d:\\ymir work\\effect\\smoke1.mse".to_string()));
}

#[test]
fn test_parse_shape_data() {
    let content = r#"
    Group ShapeData
    {
        PathName "d:\ymir work\pc\warrior\"
        ShapeDataCount 2
        
        Group ShapeData00
        {
            ShapeIndex 0
            Model "warrior_novice.gr2"
            SourceSkin "warrior_novice_red.dds"
            TargetSkin "warrior_novice_blue.dds"
        }
        
        Group ShapeData01
        {
            ShapeIndex 1
            local_model "warrior_1.gr2"
            local_sourceskin "warrior_1_red.dds"
            local_targetskin "warrior_1_blue.dds"
            
            SourceSkin2 "warrior_1_face_red.dds"
            TargetSkin2 "warrior_1_face_blue.dds"
        }
    }
    "#;

    let parsed = ScriptParser::parse(content).expect("Failed to parse");
    
    assert_eq!(parsed.shape_data.len(), 2);
    
    let shape0 = &parsed.shape_data[0];
    assert_eq!(shape0.shape_index, 0);
    assert_eq!(shape0.model_file_name, "d:\\ymir work\\pc\\warrior\\warrior_novice.gr2");
    assert_eq!(shape0.skins.len(), 1);
    assert_eq!(shape0.skins[0].src_file_name, "d:\\ymir work\\pc\\warrior\\warrior_novice_red.dds");
    assert_eq!(shape0.skins[0].dst_file_name, "d:\\ymir work\\pc\\warrior\\warrior_novice_blue.dds");

    let shape1 = &parsed.shape_data[1];
    assert_eq!(shape1.shape_index, 1);
    assert_eq!(shape1.model_file_name, "warrior_1.gr2");
    assert_eq!(shape1.skins.len(), 2);
    assert_eq!(shape1.skins[0].src_file_name, "warrior_1_red.dds");
    assert_eq!(shape1.skins[0].dst_file_name, "warrior_1_blue.dds");
    assert_eq!(shape1.skins[1].src_file_name, "d:\\ymir work\\pc\\warrior\\warrior_1_face_red.dds");
    assert_eq!(shape1.skins[1].dst_file_name, "d:\\ymir work\\pc\\warrior\\warrior_1_face_blue.dds");
}

#[test]
fn test_parse_hair_data() {
    let content = r#"
    Group HairData
    {
        PathName "d:\ymir work\pc\warrior\hair\"
        HairDataCount 1
        
        Group HairData00
        {
            HairIndex 1
            Model "hair_1.gr2"
            SourceSkin "hair_1.dds"
            TargetSkin "hair_1_blue.dds"
        }
    }
    "#;

    let parsed = ScriptParser::parse(content).expect("Failed to parse");
    
    assert_eq!(parsed.hair_data.len(), 1);
    
    let hair0 = &parsed.hair_data[0];
    assert_eq!(hair0.hair_index, 1);
    assert_eq!(hair0.model_file_name, "d:\\ymir work\\pc\\warrior\\hair\\hair_1.gr2");
    assert_eq!(hair0.skins.len(), 1);
    assert_eq!(hair0.skins[0].src_file_name, "d:\\ymir work\\pc\\warrior\\hair\\hair_1.dds");
    assert_eq!(hair0.skins[0].dst_file_name, "d:\\ymir work\\pc\\warrior\\hair\\hair_1_blue.dds");
}

#[test]
fn test_parse_attaching_data() {
    let content = r#"
    Group AttachingData
    {
        AttachingDataCount 3
        
        Group AttachingData00
        {
            AttachingDataType 1
            isAttaching 1
            AttachingModelIndex 0
            AttachingBoneName "Bip01"
            
            CollisionType 3
            SphereDataCount 1
            Group SphereData00
            {
                Radius 10.5
                Position 0.0 0.0 5.0
            }
        }
        
        Group AttachingData01
        {
            AttachingDataType 2
            isAttaching 0
            AttachingModelIndex 1
            AttachingBoneName "Bip01 Head"
            
            EffectScriptName "d:\ymir work\effect\head.mse"
            EffectPosition 0.0 0.0 0.0
            EffectRotation 0.0 90.0 0.0
        }
        
        Group AttachingData02
        {
            AttachingDataType 3
            isAttaching 1
            AttachingModelIndex 2
            AttachingBoneName "Bip01 L Hand"
            
            ObjectScriptName "d:\ymir work\item\sword.msm"
        }
    }
    "#;

    let parsed = ScriptParser::parse(content).expect("Failed to parse");
    
    assert_eq!(parsed.attaching_data.len(), 3);
    
    // 1. Collision
    let a0 = &parsed.attaching_data[0];
    assert_eq!(a0.data_type, AttachingDataType::CollisionData);
    assert!(a0.is_attaching);
    assert_eq!(a0.attaching_model_index, 0);
    assert_eq!(a0.attaching_bone_name, "Bip01");
    let col = a0.collision_data.as_ref().unwrap();
    assert_eq!(col.collision_type, 3);
    assert_eq!(col.spheres.len(), 1);
    assert_eq!(col.spheres[0].radius, 10.5);
    assert_eq!(col.spheres[0].position, [0.0, 0.0, 5.0]);

    // 2. Effect
    let a1 = &parsed.attaching_data[1];
    assert_eq!(a1.data_type, AttachingDataType::Effect);
    assert!(!a1.is_attaching);
    assert_eq!(a1.attaching_model_index, 1);
    assert_eq!(a1.attaching_bone_name, "Bip01 Head");
    let eff = a1.effect_data.as_ref().unwrap();
    assert_eq!(eff.file_name, "d:\\ymir work\\effect\\head.mse");
    assert_eq!(eff.position, [0.0, 0.0, 0.0]);
    assert_eq!(eff.rotation, [0.0, 90.0, 0.0]);
    
    // 3. Object
    let a2 = &parsed.attaching_data[2];
    assert_eq!(a2.data_type, AttachingDataType::Object);
    assert!(a2.is_attaching);
    assert_eq!(a2.attaching_model_index, 2);
    assert_eq!(a2.attaching_bone_name, "Bip01 L Hand");
    let obj = a2.object_data.as_ref().unwrap();
    assert_eq!(obj.file_name, "d:\\ymir work\\item\\sword.msm");
}

#[test]
fn test_warrior_m_mock() {
    let content = r#"
        BaseModelFileName "D:\ymir work\pc\warrior\warrior_novice.GR2"
        SmokeBoneName "Bip01 Spine"

        Group ShapeData
        {
            PathName "d:\ymir work\pc\warrior\"
            ShapeDataCount 1
            Group ShapeData00
            {
                ShapeIndex 0
                Model "warrior_novice.gr2"
                SourceSkin "warrior_novice_red.dds"
                TargetSkin "warrior_novice_blue.dds"
            }
        }
        
        Group HairData
        {
            PathName "d:\ymir work\pc\warrior\hair\"
            HairDataCount 1
            Group HairData00
            {
                HairIndex 1
                Model "hair_1.gr2"
                SourceSkin "hair_1.dds"
                TargetSkin "hair_1.dds"
            }
        }
        
        Group AttachingData
        {
            AttachingDataCount 1
            Group AttachingData00
            {
                AttachingDataType 1
                isAttaching 1
                AttachingModelIndex 0
                AttachingBoneName "Bip01"
                CollisionType 3
                Group SphereData00
                {
                    Radius 10.5
                    Position 0.0 0.0 5.0
                }
            }
        }
    "#;
    
    let parsed = ScriptParser::parse(content).expect("Failed to parse");
    assert_eq!(parsed.base_model_file_name, "D:\\ymir work\\pc\\warrior\\warrior_novice.GR2");
    assert_eq!(parsed.smoke_bone_name, "Bip01 Spine");
    assert_eq!(parsed.shape_data.len(), 1);
    assert_eq!(parsed.hair_data.len(), 1);
    assert_eq!(parsed.attaching_data.len(), 1);
}
