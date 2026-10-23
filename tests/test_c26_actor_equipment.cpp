#define DOCTEST_CONFIG_IMPLEMENT_WITH_MAIN
#include "doctest/doctest.h"
#include "Client/Actors/ActorEquipmentComponent.h"
#include "Client/Core/StrongTypes.h"

using namespace Client::Actors;
using namespace Client::Core;

TEST_CASE("ActorEquipmentComponent core functionality") {
    ActorEquipmentComponent equipment;
    
    SUBCASE("Equipping valid items") {
        auto result1 = equipment.EquipItem(SlotIndex{0}, ItemVnum{1001});
        CHECK(result1.has_value());
        
        auto result2 = equipment.EquipItem(SlotIndex{1}, ItemVnum{1002});
        CHECK(result2.has_value());
        
        auto item1 = equipment.GetItem(SlotIndex{0});
        REQUIRE(item1.has_value());
        CHECK(item1->value() == 1001);
        
        auto item2 = equipment.GetItem(SlotIndex{1});
        REQUIRE(item2.has_value());
        CHECK(item2->value() == 1002);
    }
    
    SUBCASE("Equipping invalid items") {
        auto result = equipment.EquipItem(SlotIndex{0}, ItemVnum{0}); // invalid
        REQUIRE(!result.has_value());
        CHECK(result.error() == EquipmentError::InvalidItem);
        
        auto item = equipment.GetItem(SlotIndex{0});
        CHECK(!item.has_value());
    }
    
    SUBCASE("Equipping on occupied slot") {
        auto result1 = equipment.EquipItem(SlotIndex{0}, ItemVnum{1001});
        CHECK(result1.has_value());
        
        auto result2 = equipment.EquipItem(SlotIndex{0}, ItemVnum{1002});
        REQUIRE(!result2.has_value());
        CHECK(result2.error() == EquipmentError::SlotAlreadyOccupied);
        
        auto item = equipment.GetItem(SlotIndex{0});
        REQUIRE(item.has_value());
        CHECK(item->value() == 1001); // Item should not be changed
    }
    
    SUBCASE("Unequipping items") {
        equipment.EquipItem(SlotIndex{0}, ItemVnum{1001});
        
        auto result = equipment.UnequipItem(SlotIndex{0});
        REQUIRE(result.has_value());
        
        auto item = equipment.GetItem(SlotIndex{0});
        CHECK(!item.has_value());
    }
    
    SUBCASE("Unequipping from empty slot") {
        auto result = equipment.UnequipItem(SlotIndex{0});
        REQUIRE(!result.has_value());
        CHECK(result.error() == EquipmentError::SlotEmpty);
    }
    
    SUBCASE("Setting valid model parts") {
        auto result1 = equipment.SetModelPart(ModelPart::Weapon, ItemVnum{2001});
        CHECK(result1.has_value());
        
        auto result2 = equipment.SetModelPart(ModelPart::Hair, ItemVnum{2002});
        CHECK(result2.has_value());
        
        auto part1 = equipment.GetModelPart(ModelPart::Weapon);
        REQUIRE(part1.has_value());
        CHECK(part1->value() == 2001);
        
        auto part2 = equipment.GetModelPart(ModelPart::Hair);
        REQUIRE(part2.has_value());
        CHECK(part2->value() == 2002);
    }
    
    SUBCASE("Setting invalid model parts") {
        auto result = equipment.SetModelPart(ModelPart::Weapon, ItemVnum{0});
        REQUIRE(!result.has_value());
        CHECK(result.error() == EquipmentError::InvalidItem);
        
        auto part = equipment.GetModelPart(ModelPart::Weapon);
        CHECK(!part.has_value());
    }
    
    SUBCASE("Setting on occupied model part") {
        auto result1 = equipment.SetModelPart(ModelPart::Weapon, ItemVnum{2001});
        CHECK(result1.has_value());
        
        auto result2 = equipment.SetModelPart(ModelPart::Weapon, ItemVnum{2002});
        REQUIRE(!result2.has_value());
        CHECK(result2.error() == EquipmentError::SlotAlreadyOccupied);
        
        auto part = equipment.GetModelPart(ModelPart::Weapon);
        REQUIRE(part.has_value());
        CHECK(part->value() == 2001);
    }
    
    SUBCASE("Removing model parts") {
        equipment.SetModelPart(ModelPart::Weapon, ItemVnum{2001});
        
        auto result = equipment.RemoveModelPart(ModelPart::Weapon);
        REQUIRE(result.has_value());
        
        auto part = equipment.GetModelPart(ModelPart::Weapon);
        CHECK(!part.has_value());
    }
    
    SUBCASE("Removing from empty model part") {
        auto result = equipment.RemoveModelPart(ModelPart::Weapon);
        REQUIRE(!result.has_value());
        CHECK(result.error() == EquipmentError::SlotEmpty);
    }
    
    SUBCASE("Clear all") {
        equipment.EquipItem(SlotIndex{0}, ItemVnum{1001});
        equipment.EquipItem(SlotIndex{1}, ItemVnum{1002});
        equipment.SetModelPart(ModelPart::Weapon, ItemVnum{2001});
        equipment.SetModelPart(ModelPart::Hair, ItemVnum{2002});
        
        equipment.ClearAll();
        
        CHECK(!equipment.GetItem(SlotIndex{0}).has_value());
        CHECK(!equipment.GetItem(SlotIndex{1}).has_value());
        CHECK(!equipment.GetModelPart(ModelPart::Weapon).has_value());
        CHECK(!equipment.GetModelPart(ModelPart::Hair).has_value());
    }
}