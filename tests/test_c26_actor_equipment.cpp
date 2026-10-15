#define DOCTEST_CONFIG_IMPLEMENT_WITH_MAIN
#include "doctest/doctest.h"
#include "Client/Actor/ActorEquipmentComponent.h"
#include "Client/Core/StrongTypes.h"

using namespace Client::Actor;
using namespace Client::Core;

TEST_CASE("ActorEquipmentComponent core functionality") {
    ActorEquipmentComponent equipment;
    
    SUBCASE("Equipping valid items") {
        auto result1 = equipment.Equip(SlotIndex{0}, ItemVnum{1001});
        CHECK(result1.has_value());
        
        auto result2 = equipment.Equip(SlotIndex{1}, ItemVnum{1002});
        CHECK(result2.has_value());
        
        auto item1 = equipment.GetItem(SlotIndex{0});
        REQUIRE(item1.has_value());
        CHECK(item1->value() == 1001);
        
        auto item2 = equipment.GetItem(SlotIndex{1});
        REQUIRE(item2.has_value());
        CHECK(item2->value() == 1002);
    }
    
    SUBCASE("Equipping invalid items") {
        auto result = equipment.Equip(SlotIndex{0}, ItemVnum{0}); // invalid
        REQUIRE(!result.has_value());
        CHECK(result.error() == EquipmentError::InvalidItem);
        
        auto item = equipment.GetItem(SlotIndex{0});
        CHECK(!item.has_value());
    }
    
    SUBCASE("Equipping on occupied slot") {
        auto result1 = equipment.Equip(SlotIndex{0}, ItemVnum{1001});
        CHECK(result1.has_value());
        
        auto result2 = equipment.Equip(SlotIndex{0}, ItemVnum{1002});
        REQUIRE(!result2.has_value());
        CHECK(result2.error() == EquipmentError::SlotOccupied);
        
        auto item = equipment.GetItem(SlotIndex{0});
        REQUIRE(item.has_value());
        CHECK(item->value() == 1001); // Item should not be changed
    }
    
    SUBCASE("Unequipping items") {
        equipment.Equip(SlotIndex{0}, ItemVnum{1001});
        
        auto result = equipment.Unequip(SlotIndex{0});
        REQUIRE(result.has_value());
        CHECK(result.value().value() == 1001);
        
        auto item = equipment.GetItem(SlotIndex{0});
        CHECK(!item.has_value());
    }
    
    SUBCASE("Unequipping from empty slot") {
        auto result = equipment.Unequip(SlotIndex{0});
        REQUIRE(!result.has_value());
        CHECK(result.error() == EquipmentError::SlotEmpty);
    }
    
    SUBCASE("Clear equipment") {
        equipment.Equip(SlotIndex{0}, ItemVnum{1001});
        equipment.Equip(SlotIndex{1}, ItemVnum{1002});
        equipment.Equip(SlotIndex{2}, ItemVnum{1003});
        
        equipment.Clear();
        
        CHECK(!equipment.GetItem(SlotIndex{0}).has_value());
        CHECK(!equipment.GetItem(SlotIndex{1}).has_value());
        CHECK(!equipment.GetItem(SlotIndex{2}).has_value());
    }
}
