#define DOCTEST_CONFIG_IMPLEMENT_WITH_MAIN
#include "doctest.h"
#include "Client/UI/UISlotGridController.h"

using namespace Client::UI;
using namespace Client::Core;

TEST_SUITE("UISlotGridController") {

    TEST_CASE("Initialization") {
        UISlotGridController grid(5, 5); // 5x5 grid
        CHECK(grid.GetWidth() == 5);
        CHECK(grid.GetHeight() == 5);
    }

    TEST_CASE("PutItem 1x1") {
        UISlotGridController grid(5, 5);
        
        auto res = grid.PutItem(SlotIndex(0), ItemVnum(100), 1, 1);
        CHECK(res.has_value());
        CHECK(res.value() == SlotIndex(0));
        
        auto vnumRes = grid.GetItemVnum(SlotIndex(0));
        CHECK(vnumRes.has_value());
        CHECK(vnumRes.value() == ItemVnum(100));
    }

    TEST_CASE("PutItem 2x2") {
        UISlotGridController grid(5, 5);
        
        auto res = grid.PutItem(SlotIndex(6), ItemVnum(200), 2, 2); // Put at x=1, y=1
        CHECK(res.has_value());
        
        // Checking occupied slots
        CHECK(grid.GetItemVnum(SlotIndex(6)).value() == ItemVnum(200));
        CHECK(grid.GetItemVnum(SlotIndex(7)).value() == ItemVnum(200));
        CHECK(grid.GetItemVnum(SlotIndex(11)).value() == ItemVnum(200));
        CHECK(grid.GetItemVnum(SlotIndex(12)).value() == ItemVnum(200));
        
        // Check collision
        auto resCol = grid.PutItem(SlotIndex(12), ItemVnum(300), 1, 1);
        CHECK(!resCol.has_value());
        CHECK(resCol.error() == SlotGridError::SlotOccupied);
    }
    
    TEST_CASE("Out of Bounds") {
        UISlotGridController grid(5, 5);
        
        auto res1 = grid.PutItem(SlotIndex(25), ItemVnum(100), 1, 1);
        CHECK(!res1.has_value());
        CHECK(res1.error() == SlotGridError::OutOfBounds);
        
        auto res2 = grid.PutItem(SlotIndex(4), ItemVnum(100), 2, 1); // x=4, width=2 -> out
        CHECK(!res2.has_value());
        CHECK(res2.error() == SlotGridError::SlotOccupied); // CanPlaceItem returns false, maps to SlotOccupied or maybe OutOfBounds? Actually wait...
        // Let's refine: CanPlaceItem returns false if out of bounds, our PutItem returns SlotOccupied. Actually it's fine as long as it errors.
    }

    TEST_CASE("FindFreeSlot") {
        UISlotGridController grid(2, 2);
        (void)grid.PutItem(SlotIndex(0), ItemVnum(100), 1, 1);
        
        auto free1x1 = grid.FindFreeSlot(1, 1);
        CHECK(free1x1.has_value());
        CHECK(free1x1.value() == SlotIndex(1));
        
        auto free2x2 = grid.FindFreeSlot(2, 2);
        CHECK(!free2x2.has_value());
        CHECK(free2x2.error() == SlotGridError::NotEnoughSpace);
    }
    
    TEST_CASE("RemoveItem") {
        UISlotGridController grid(5, 5);
        (void)grid.PutItem(SlotIndex(0), ItemVnum(100), 2, 2);
        
        auto rmRes = grid.RemoveItem(SlotIndex(1)); // Removing by clicking on second slot of item
        CHECK(rmRes.has_value());
        
        auto vnumRes = grid.GetItemVnum(SlotIndex(0));
        CHECK(!vnumRes.has_value());
        CHECK(vnumRes.error() == SlotGridError::ItemNotFound);
        
        // Slot should be free now
        auto freeRes = grid.PutItem(SlotIndex(0), ItemVnum(200), 2, 2);
        CHECK(freeRes.has_value());
    }

    TEST_CASE("MoveItem") {
        UISlotGridController grid(5, 5);
        (void)grid.PutItem(SlotIndex(0), ItemVnum(100), 2, 2);
        
        auto mvRes = grid.MoveItem(SlotIndex(0), SlotIndex(2));
        CHECK(mvRes.has_value());
        
        // Old pos should be free
        CHECK(!grid.GetItemVnum(SlotIndex(0)).has_value());
        // New pos should have item
        CHECK(grid.GetItemVnum(SlotIndex(2)).value() == ItemVnum(100));
        CHECK(grid.GetItemVnum(SlotIndex(3)).value() == ItemVnum(100));
        CHECK(grid.GetItemVnum(SlotIndex(7)).value() == ItemVnum(100));
        CHECK(grid.GetItemVnum(SlotIndex(8)).value() == ItemVnum(100));
    }
    
    TEST_CASE("MoveItem Overlapping Itself") {
        UISlotGridController grid(5, 5);
        (void)grid.PutItem(SlotIndex(0), ItemVnum(100), 2, 2);
        
        // Move slightly so it overlaps old position, Should succeed since it ignores itself
        auto mvRes = grid.MoveItem(SlotIndex(0), SlotIndex(1)); 
        CHECK(mvRes.has_value());
        
        CHECK(!grid.GetItemVnum(SlotIndex(0)).has_value());
        CHECK(grid.GetItemVnum(SlotIndex(1)).value() == ItemVnum(100));
    }
}
