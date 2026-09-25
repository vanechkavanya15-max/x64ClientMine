use m2_client::instance_base::{CreateData, InstanceBase};

#[test]
fn test_integration_instance_base() {
    let instance = InstanceBase::new();
    assert!(instance.is_ready());
}

#[test]
fn test_integration_create_and_getters_setters() {
    let mut instance = InstanceBase::new();
    let data = CreateData {
        type_: 1,
        state_flags: 2,
        empire_id: 3,
        guild_id: 4,
        level: 50,
        vid: 12345,
        race: 6,
        mov_spd: 100,
        atk_spd: 120,
        pos_x: 1000,
        pos_y: 2000,
        rot: 90.0,
        armor: 11200,
        weapon: 279,
        hair: 1001,
        mount_vnum: 20101,
        alignment: 12000,
        pk_mode: 0,
        name: "Wojownik".to_string(),
        is_main: true,
    };

    instance.create(data);

    assert!(instance.is_ready());
    assert_eq!(instance.get_vid(), 12345);
    assert_eq!(instance.get_name(), "Wojownik");
    assert_eq!(instance.get_race(), 6);
    assert_eq!(instance.get_empire_id(), 3);
    assert_eq!(instance.get_guild_id(), 4);
    assert_eq!(instance.get_level(), 50);
    assert_eq!(instance.get_alignment(), 12000);
    assert_eq!(instance.get_pk_mode(), 0);
    assert_eq!(instance.get_armor(), 11200);
    assert_eq!(instance.get_weapon(), 279);
    assert_eq!(instance.get_hair(), 1001);
    assert_eq!(instance.get_mount_vnum(), 20101);

    // Test setters
    instance.set_level(51);
    assert_eq!(instance.get_level(), 51);

    instance.set_alignment(13000);
    assert_eq!(instance.get_alignment(), 13000);

    instance.set_name("Sura".to_string());
    assert_eq!(instance.get_name(), "Sura");
}

#[test]
fn test_integration_state_process() {
    use m2_client::instance_base::{Command, PixelPosition};

    let mut instance = InstanceBase::new();
    let data = CreateData {
        vid: 1,
        ..Default::default()
    };
    instance.create(data);
    assert_eq!(instance.rot, 0.0);

    let cmd = Command {
        chk_time: 100,
        cmd_time: 100,
        dst_rot: 180.0,
        func: 1, // Func::Move
        arg: 0,
        target_vid: 0,
        dst_pos: PixelPosition::new(10.0, 10.0, 0.0),
    };

    instance.state_process(cmd);
    assert_eq!(instance.rot, 180.0);
}
