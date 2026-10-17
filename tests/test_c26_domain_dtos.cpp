#define DOCTEST_CONFIG_IMPLEMENT_WITH_MAIN
#include "doctest.h"
#include "Client/Contracts/DomainDTOs.h"
#include <cstring>
#include <vector>

using namespace Client::Contracts;

TEST_SUITE("DomainDTOs_C_ABI_Parity")
{
    TEST_CASE("ActorSnapshot_SizeAndAlignment")
    {
        CHECK(sizeof(ActorSnapshot) == 32);
        CHECK(std::is_standard_layout_v<ActorSnapshot>);
        CHECK(std::is_trivially_copyable_v<ActorSnapshot>);

        ActorSnapshot actor{};
        actor.vid = 1001;
        actor.vnum = 20016; // Kowal / Blacksmith
        actor.x = 12345.6f;
        actor.y = 65432.1f;
        actor.z = 150.0f;
        actor.hp = 10000;
        actor.max_hp = 10000;
        actor.type = 1; // NPC
        actor.is_dead = 0;

        // Symulacja transferu pamieci FFI zero-copy
        std::vector<uint8_t> buffer(sizeof(ActorSnapshot));
        std::memcpy(buffer.data(), &actor, sizeof(ActorSnapshot));

        ActorSnapshot reconstructed{};
        std::memcpy(&reconstructed, buffer.data(), sizeof(ActorSnapshot));

        CHECK(reconstructed.vid == 1001);
        CHECK(reconstructed.vnum == 20016);
        CHECK(reconstructed.x == doctest::Approx(12345.6f));
        CHECK(reconstructed.hp == 10000);
        CHECK(reconstructed.type == 1);
        CHECK(reconstructed.is_dead == 0);
    }

    TEST_CASE("ActorAppearanceDTO_SizeAndAlignment")
    {
        CHECK(sizeof(ActorAppearanceDTO) == 32);
        CHECK(std::is_standard_layout_v<ActorAppearanceDTO>);
        CHECK(std::is_trivially_copyable_v<ActorAppearanceDTO>);

        ActorAppearanceDTO app{};
        app.vid = 505;
        app.armor_vnum = 11299; // Stal Wojownik +9
        app.weapon_vnum = 299;  // FMS +9
        app.hair_vnum = 5001;
        app.acce_vnum = 85001;
        app.mount_vnum = 20110;
        app.alignment = 20000;  // Rycerski
        app.pk_mode = 0;        // Peace
        app.guild_rank = 1;     // Lider
        app.guild_id = 42;

        CHECK(app.armor_vnum == 11299);
        CHECK(app.weapon_vnum == 299);
        CHECK(app.alignment == 20000);
        CHECK(app.guild_id == 42);
    }

    TEST_CASE("ActorMotionStateDTO_SizeAndAlignment")
    {
        CHECK(sizeof(ActorMotionStateDTO) == 24);
        CHECK(std::is_standard_layout_v<ActorMotionStateDTO>);
        CHECK(std::is_trivially_copyable_v<ActorMotionStateDTO>);

        ActorMotionStateDTO motion{};
        motion.vid = 100;
        motion.motion_mode = 2; // Run
        motion.motion_index = 1;
        motion.rotation_deg = 180.0f;
        motion.move_speed = 150.0f;
        motion.attack_speed = 120.0f;
        motion.is_combo = 0;
        motion.is_loop = 1;

        CHECK(motion.motion_mode == 2);
        CHECK(motion.rotation_deg == doctest::Approx(180.0f));
        CHECK(motion.is_loop == 1);
    }

    TEST_CASE("PlayerStatsDTO_SizeAndAlignment")
    {
        CHECK(sizeof(PlayerStatsDTO) == 72);
        CHECK(std::is_standard_layout_v<PlayerStatsDTO>);
        CHECK(std::is_trivially_copyable_v<PlayerStatsDTO>);

        PlayerStatsDTO stats{};
        stats.vid = 1;
        stats.level = 105;
        stats.hp = 35000;
        stats.max_hp = 35000;
        stats.sp = 12000;
        stats.max_sp = 12000;
        stats.yang = 1999999999LL;
        stats.exp = 12345678LL;
        stats.next_exp = 50000000LL;
        stats.cheque = 500;
        stats.gem = 1250;

        CHECK(stats.level == 105);
        CHECK(stats.hp == 35000);
        CHECK(stats.yang == 1999999999LL);
        CHECK(stats.cheque == 500);
    }

    TEST_CASE("PlayerAttributesDTO_SizeAndAlignment")
    {
        CHECK(sizeof(PlayerAttributesDTO) == 32);
        CHECK(std::is_standard_layout_v<PlayerAttributesDTO>);
        CHECK(std::is_trivially_copyable_v<PlayerAttributesDTO>);

        PlayerAttributesDTO attr{};
        attr.str = 90;
        attr.dex = 90;
        attr.con = 90;
        attr.int_pt = 16;
        attr.min_atk = 850;
        attr.max_atk = 980;
        attr.def = 650;
        attr.magic_def = 300;
        attr.atk_speed = 145;
        attr.move_speed = 160;
        attr.cast_speed = 120;
        attr.status_flags = 0;
        attr.play_time_sec = 360000;

        CHECK(attr.str == 90);
        CHECK(attr.min_atk == 850);
        CHECK(attr.move_speed == 160);
    }

    TEST_CASE("PlayerQuickslotDTO_SizeAndAlignment")
    {
        CHECK(sizeof(PlayerQuickslotDTO) == 8);
        CHECK(std::is_standard_layout_v<PlayerQuickslotDTO>);
        CHECK(std::is_trivially_copyable_v<PlayerQuickslotDTO>);

        PlayerQuickslotDTO slot{};
        slot.slot_type = 2; // Skill
        slot.window_type = 0;
        slot.slot_index = 0;
        slot.vnum = 1;      // Trzystronne Ciecie

        CHECK(slot.slot_type == 2);
        CHECK(slot.vnum == 1);
    }

    TEST_CASE("ItemSlotDTO_SizeAndAlignment")
    {
        CHECK(sizeof(ItemSlotDTO) == 104);
        CHECK(sizeof(ItemAttributeDTO) == 8);
        CHECK(std::is_standard_layout_v<ItemSlotDTO>);
        CHECK(std::is_trivially_copyable_v<ItemSlotDTO>);

        ItemSlotDTO item{};
        item.vnum = 299; // FMS +9
        item.count = 1;
        item.cell = 0;
        item.window_type = 1; // Inventory
        item.is_equipped = 0;
        item.is_locked = 0;
        item.sockets[0] = 28430; // KD Smierci +4
        item.sockets[1] = 28434; // KD Potwora +4
        item.attributes[0].type = 72; // Sr Obrazenia
        item.attributes[0].value = 45; // 45%

        CHECK(item.vnum == 299);
        CHECK(item.sockets[0] == 28430);
        CHECK(item.attributes[0].type == 72);
        CHECK(item.attributes[0].value == 45);
    }

    TEST_CASE("InventoryGridSummaryDTO_SizeAndAlignment")
    {
        CHECK(sizeof(InventoryGridSummaryDTO) == 32);
        CHECK(std::is_standard_layout_v<InventoryGridSummaryDTO>);
        CHECK(std::is_trivially_copyable_v<InventoryGridSummaryDTO>);

        InventoryGridSummaryDTO grid{};
        grid.total_slots = 90;
        grid.occupied_slots = 15;
        grid.total_items_count = 1500;
        grid.page_masks[0] = 0x000000000000FFFFULL; // 16 zajetych slotow na stronie 0

        CHECK(grid.total_slots == 90);
        CHECK(grid.occupied_slots == 15);
        CHECK(grid.page_masks[0] == 0x000000000000FFFFULL);
    }

    TEST_CASE("SkillSlotDTO_SizeAndAlignment")
    {
        CHECK(sizeof(SkillSlotDTO) == 24);
        CHECK(std::is_standard_layout_v<SkillSlotDTO>);
        CHECK(std::is_trivially_copyable_v<SkillSlotDTO>);

        SkillSlotDTO skill{};
        skill.vnum = 1;
        skill.level = 40;
        skill.master_type = 3; // Perfect Master
        skill.can_use = 1;
        skill.is_toggled = 0;
        skill.sp_cost = 250;
        skill.cooldown_remaining_ms = 0;
        skill.cooldown_total_ms = 12000;
        skill.last_cast_time_ms = 100000;

        CHECK(skill.vnum == 1);
        CHECK(skill.master_type == 3);
        CHECK(skill.cooldown_total_ms == 12000);
    }

    TEST_CASE("NetworkSessionDTO_SizeAndAlignment")
    {
        CHECK(sizeof(PacketHeaderDTO) == 8);
        CHECK(sizeof(NetworkSessionDTO) == 32);
        CHECK(std::is_standard_layout_v<NetworkSessionDTO>);
        CHECK(std::is_trivially_copyable_v<NetworkSessionDTO>);

        NetworkSessionDTO session{};
        session.phase = 5; // Game
        session.ping_ms = 24;
        session.bytes_received = 1048576;
        session.bytes_sent = 524288;

        CHECK(session.phase == 5);
        CHECK(session.ping_ms == 24);
        CHECK(session.bytes_received == 1048576);
    }

    TEST_CASE("CombatAndNavigationDTO_SizeAndAlignment")
    {
        CHECK(sizeof(CombatCommandDTO) == 28);
        CHECK(sizeof(NavigationWaypointDTO) == 28);
        CHECK(std::is_standard_layout_v<CombatCommandDTO>);
        CHECK(std::is_standard_layout_v<NavigationWaypointDTO>);

        CombatCommandDTO cmd{};
        cmd.target_vid = 1005;
        cmd.attack_type = 1; // Skill
        cmd.skill_vnum = 1;
        cmd.range_limit = 250.0f;

        CHECK(cmd.target_vid == 1005);
        CHECK(cmd.skill_vnum == 1);
        CHECK(cmd.range_limit == doctest::Approx(250.0f));

        NavigationWaypointDTO wp{};
        wp.x = 50000.0f;
        wp.y = 80000.0f;
        wp.z = 200.0f;
        wp.acceptance_radius = 25.0f;
        wp.movement_mode = 2; // Run

        CHECK(wp.x == doctest::Approx(50000.0f));
        CHECK(wp.movement_mode == 2);
    }

    TEST_CASE("WorldAndTerrainDTO_SizeAndAlignment")
    {
        CHECK(sizeof(TerrainQueryDTO) == 32);
        CHECK(sizeof(WorldEnvironmentDTO) == 64);
        CHECK(std::is_standard_layout_v<TerrainQueryDTO>);
        CHECK(std::is_standard_layout_v<WorldEnvironmentDTO>);

        TerrainQueryDTO terrain{};
        terrain.world_x = 1000.0f;
        terrain.world_y = 2000.0f;
        terrain.height_z = 350.5f;
        terrain.attr_flags = 0; // Wolna przestrzen
        terrain.is_walkable = 1;

        CHECK(terrain.height_z == doctest::Approx(350.5f));
        CHECK(terrain.is_walkable == 1);

        WorldEnvironmentDTO env{};
        env.map_id = 1;
        env.time_of_day = 14.5f;
        env.fog_near = 3000.0f;
        env.fog_far = 15000.0f;

        CHECK(env.time_of_day == doctest::Approx(14.5f));
        CHECK(env.fog_far == doctest::Approx(15000.0f));
    }

    TEST_CASE("WindowGeometryAndEventDTO_SizeAndAlignment")
    {
        CHECK(sizeof(WindowGeometryDTO) == 32);
        CHECK(sizeof(GameEventDTO) == 32);
        CHECK(std::is_standard_layout_v<WindowGeometryDTO>);
        CHECK(std::is_standard_layout_v<GameEventDTO>);

        WindowGeometryDTO win{};
        win.window_id = 10;
        win.parent_id = 0;
        win.left = 100;
        win.top = 200;
        win.right = 300;
        win.bottom = 450;
        win.is_show = 1;
        win.is_rendering = 1;
        win.window_type = 3; // SlotWindow

        CHECK(win.window_id == 10);
        CHECK(win.right - win.left == 200);
        CHECK(win.bottom - win.top == 250);

        GameEventDTO evt{};
        evt.event_type = 1; // Damage
        evt.timestamp_ms = 54321;
        evt.source_vid = 100;
        evt.target_vid = 200;
        evt.value1 = 4520; // 4520 dmg
        evt.value2 = 1;    // Krytyk
        evt.flag = 0;

        CHECK(evt.event_type == 1);
        CHECK(evt.value1 == 4520);
        CHECK(evt.value2 == 1);
    }
}
