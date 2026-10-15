#include "doctest.h"
#include "Client/Player/PlayerEquipmentController.h"

using namespace Client::Player;
using namespace Client::Core;

TEST_CASE("PlayerEquipmentController - EquipItem") {
    PlayerEquipmentController controller;
    
    SUBCASE("Successfully equip item") {
        auto result = controller.EquipItem(SlotIndex(1), ItemVnum(100));
        CHECK(result.has_value() == true);
        CHECK(controller.HasItem(SlotIndex(1)) == true);
        CHECK(controller.GetItem(SlotIndex(1)).value() == ItemVnum(100));
    }
    
    SUBCASE("Fail on invalid slot") {
        auto result = controller.EquipItem(SlotIndex(0xFFFF), ItemVnum(100));
        CHECK(result.has_value() == false);
        CHECK(result.error() == EquipmentError::InvalidSlot);
    }
    
    SUBCASE("Fail on invalid item") {
        auto result = controller.EquipItem(SlotIndex(1), ItemVnum(0));
        CHECK(result.has_value() == false);
        CHECK(result.error() == EquipmentError::InvalidItem);
    }
    
    SUBCASE("Fail on occupied slot") {
        controller.EquipItem(SlotIndex(1), ItemVnum(100));
        auto result = controller.EquipItem(SlotIndex(1), ItemVnum(200));
        CHECK(result.has_value() == false);
        CHECK(result.error() == EquipmentError::SlotOccupied);
    }
}

TEST_CASE("PlayerEquipmentController - UnequipItem") {
    PlayerEquipmentController controller;
    controller.EquipItem(SlotIndex(1), ItemVnum(100));
    
    SUBCASE("Successfully unequip item") {
        auto result = controller.UnequipItem(SlotIndex(1));
        CHECK(result.has_value() == true);
        CHECK(result.value() == ItemVnum(100));
        CHECK(controller.HasItem(SlotIndex(1)) == false);
    }
    
    SUBCASE("Fail on invalid slot") {
        auto result = controller.UnequipItem(SlotIndex(0xFFFF));
        CHECK(result.has_value() == false);
        CHECK(result.error() == EquipmentError::InvalidSlot);
    }
    
    SUBCASE("Fail on empty slot") {
        auto result = controller.UnequipItem(SlotIndex(2));
        CHECK(result.has_value() == false);
        CHECK(result.error() == EquipmentError::SlotEmpty);
    }
}

TEST_CASE("PlayerEquipmentController - Clear") {
    PlayerEquipmentController controller;
    controller.EquipItem(SlotIndex(1), ItemVnum(100));
    controller.EquipItem(SlotIndex(2), ItemVnum(200));
    
    controller.Clear();
    CHECK(controller.HasItem(SlotIndex(1)) == false);
    CHECK(controller.HasItem(SlotIndex(2)) == false);
}

TEST_CASE("PlayerEquipmentController - GetItem and HasItem") {
    PlayerEquipmentController controller;
    
    SUBCASE("HasItem with invalid slot") {
        CHECK(controller.HasItem(SlotIndex(0xFFFF)) == false);
    }
    
    SUBCASE("GetItem with invalid slot") {
        CHECK(controller.GetItem(SlotIndex(0xFFFF)).has_value() == false);
    }
    
    SUBCASE("GetItem with empty slot") {
        CHECK(controller.GetItem(SlotIndex(1)).has_value() == false);
    }
}
