#define DOCTEST_CONFIG_IMPLEMENT_WITH_MAIN
#include "doctest.h"
#include "Client/Actors/ActorEquipmentComponent.h"

using namespace Client::Actors;
using namespace Client::Core;

TEST_CASE("ActorEquipmentComponent Core Mechanics") {
    ActorEquipmentComponent eq;

    SlotIndex slotWeapon(1);
    SlotIndex slotArmor(2);
    ItemVnum sword(1001);
    ItemVnum armor(2001);

    SUBCASE("Initially slots are empty") {
        CHECK_FALSE(eq.HasItem(slotWeapon));
        CHECK_FALSE(eq.HasItem(slotArmor));
        CHECK(eq.GetItem(slotWeapon) == std::nullopt);
    }

    SUBCASE("Equip item successfully") {
        auto res = eq.EquipItem(slotWeapon, sword);
        CHECK(res.has_value());
        CHECK(eq.HasItem(slotWeapon));
        CHECK(eq.GetItem(slotWeapon).value() == sword);
    }

    SUBCASE("Equip item on already occupied slot fails") {
        eq.EquipItem(slotWeapon, sword);
        auto res = eq.EquipItem(slotWeapon, armor);
        CHECK_FALSE(res.has_value());
        CHECK(res.error() == EquipmentError::SlotAlreadyOccupied);
        CHECK(eq.GetItem(slotWeapon).value() == sword); // Remains unchanged
    }

    SUBCASE("Unequip item successfully") {
        eq.EquipItem(slotWeapon, sword);
        auto res = eq.UnequipItem(slotWeapon);
        CHECK(res.has_value());
        CHECK_FALSE(eq.HasItem(slotWeapon));
        CHECK(eq.GetItem(slotWeapon) == std::nullopt);
    }

    SUBCASE("Unequip item from empty slot fails") {
        auto res = eq.UnequipItem(slotArmor);
        CHECK_FALSE(res.has_value());
        CHECK(res.error() == EquipmentError::SlotEmpty);
    }
}
