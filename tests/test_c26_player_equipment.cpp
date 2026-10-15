#define DOCTEST_CONFIG_IMPLEMENT_WITH_MAIN
#include "doctest/doctest.h"
#include "Client/Player/PlayerEquipmentController.h"

using namespace Client::Player;
using namespace Client::Core;

TEST_CASE("PlayerEquipmentController - EquipItem and GetItemInSlot") {
    PlayerEquipmentController controller;
    
    SUBCASE("Equip item in empty slot succeeds") {
        ItemVnum vnum(12345);
        auto result = controller.EquipItem(EquipmentSlotType::Weapon, vnum);
        CHECK(result.has_value());
        
        auto getResult = controller.GetItemInSlot(EquipmentSlotType::Weapon);
        REQUIRE(getResult.has_value());
        CHECK(getResult.value() == vnum);
        CHECK_FALSE(controller.IsSlotEmpty(EquipmentSlotType::Weapon));
    }
    
    SUBCASE("Equip item in occupied slot fails with SlotOccupied") {
        ItemVnum vnum1(111);
        ItemVnum vnum2(222);
        
        controller.EquipItem(EquipmentSlotType::Armor, vnum1);
        auto result = controller.EquipItem(EquipmentSlotType::Armor, vnum2);
        
        REQUIRE_FALSE(result.has_value());
        CHECK(result.error() == EquipmentError::SlotOccupied);
        
        // Ensure original item is still there
        auto getResult = controller.GetItemInSlot(EquipmentSlotType::Armor);
        REQUIRE(getResult.has_value());
        CHECK(getResult.value() == vnum1);
    }
}

TEST_CASE("PlayerEquipmentController - UnequipItem") {
    PlayerEquipmentController controller;
    
    SUBCASE("Unequip existing item succeeds") {
        ItemVnum vnum(777);
        controller.EquipItem(EquipmentSlotType::Helmet, vnum);
        
        auto result = controller.UnequipItem(EquipmentSlotType::Helmet);
        CHECK(result.has_value());
        CHECK(controller.IsSlotEmpty(EquipmentSlotType::Helmet));
    }
    
    SUBCASE("Unequip empty slot fails with SlotEmpty") {
        auto result = controller.UnequipItem(EquipmentSlotType::Shield);
        
        REQUIRE_FALSE(result.has_value());
        CHECK(result.error() == EquipmentError::SlotEmpty);
    }
}

TEST_CASE("PlayerEquipmentController - ClearAll") {
    PlayerEquipmentController controller;
    
    controller.EquipItem(EquipmentSlotType::Weapon, ItemVnum(1));
    controller.EquipItem(EquipmentSlotType::Armor, ItemVnum(2));
    controller.EquipItem(EquipmentSlotType::Helmet, ItemVnum(3));
    
    CHECK_FALSE(controller.IsSlotEmpty(EquipmentSlotType::Weapon));
    CHECK_FALSE(controller.IsSlotEmpty(EquipmentSlotType::Armor));
    CHECK_FALSE(controller.IsSlotEmpty(EquipmentSlotType::Helmet));
    
    controller.ClearAll();
    
    CHECK(controller.IsSlotEmpty(EquipmentSlotType::Weapon));
    CHECK(controller.IsSlotEmpty(EquipmentSlotType::Armor));
    CHECK(controller.IsSlotEmpty(EquipmentSlotType::Helmet));
    
    auto getResult = controller.GetItemInSlot(EquipmentSlotType::Weapon);
    REQUIRE_FALSE(getResult.has_value());
    CHECK(getResult.error() == EquipmentError::SlotEmpty);
}
