#define DOCTEST_CONFIG_IMPLEMENT_WITH_MAIN
#include "doctest.h"
#include "Client/UI/SlotItemDragDropManager.h"

using namespace Client::UI;
using namespace Client::Core;

TEST_SUITE("SlotItemDragDropManager") {

    TEST_CASE("Initial State") {
        SlotItemDragDropManager manager;
        CHECK_FALSE(manager.IsDragging());
    }

    TEST_CASE("StartDrag Failures") {
        SlotItemDragDropManager manager;
        UISlotGridController grid(5, 5);

        SUBCASE("Source Grid Null") {
            auto res = manager.StartDrag(nullptr, SlotIndex(0), 1, 1);
            CHECK_FALSE(res.has_value());
            CHECK(res.error() == SlotDragDropError::InvalidTarget);
        }

        SUBCASE("Source Item Not Found") {
            auto res = manager.StartDrag(&grid, SlotIndex(0), 1, 1);
            CHECK_FALSE(res.has_value());
            CHECK(res.error() == SlotDragDropError::SourceItemNotFound);
        }
    }

    TEST_CASE("Successful StartDrag, Drop, and Cancel") {
        SlotItemDragDropManager manager;
        UISlotGridController grid(5, 5);
        (void)grid.PutItem(SlotIndex(0), ItemVnum(123), 2, 2);

        SUBCASE("Successful Drop in Same Grid") {
            auto startRes = manager.StartDrag(&grid, SlotIndex(0), 2, 2);
            CHECK(startRes.has_value());
            CHECK(manager.IsDragging());

            // Source slot should be empty during drag
            CHECK_FALSE(grid.GetItemVnum(SlotIndex(0)).has_value());

            auto dropRes = manager.Drop(&grid, SlotIndex(2));
            CHECK(dropRes.has_value());
            CHECK_FALSE(manager.IsDragging());

            // Target slot should be occupied by item 123
            auto targetItem = grid.GetItemVnum(SlotIndex(2));
            CHECK(targetItem.has_value());
            CHECK(targetItem.value() == ItemVnum(123));
        }

        SUBCASE("Cancel Drag restores item") {
            auto startRes = manager.StartDrag(&grid, SlotIndex(0), 2, 2);
            CHECK(startRes.has_value());

            auto cancelRes = manager.CancelDrag();
            CHECK(cancelRes.has_value());
            CHECK_FALSE(manager.IsDragging());

            // Original slot should have item back
            auto originalItem = grid.GetItemVnum(SlotIndex(0));
            CHECK(originalItem.has_value());
            CHECK(originalItem.value() == ItemVnum(123));
        }
    }

    TEST_CASE("Transaction Rollback on Failed Drop") {
        SlotItemDragDropManager manager;
        UISlotGridController grid(5, 5);
        (void)grid.PutItem(SlotIndex(0), ItemVnum(123), 2, 2);
        (void)grid.PutItem(SlotIndex(2), ItemVnum(456), 1, 1);

        SUBCASE("Target Occupied") {
            auto startRes = manager.StartDrag(&grid, SlotIndex(0), 2, 2);
            CHECK(startRes.has_value());

            // Try to drop on occupied slot
            auto dropRes = manager.Drop(&grid, SlotIndex(2));
            CHECK_FALSE(dropRes.has_value());
            CHECK(dropRes.error() == SlotDragDropError::TargetOccupied);
            CHECK_FALSE(manager.IsDragging()); // Should cancel drag automatically

            // Should be restored to original position
            auto originalItem = grid.GetItemVnum(SlotIndex(0));
            CHECK(originalItem.has_value());
            CHECK(originalItem.value() == ItemVnum(123));
        }

        SUBCASE("Target Out Of Bounds") {
            auto startRes = manager.StartDrag(&grid, SlotIndex(0), 2, 2);
            CHECK(startRes.has_value());

            auto dropRes = manager.Drop(&grid, SlotIndex(24)); // Near end of 5x5, 2x2 won't fit
            CHECK_FALSE(dropRes.has_value());
            CHECK(dropRes.error() == SlotDragDropError::TargetOccupied); // Our mapping says OutOfBounds maps to TargetOccupied or GridError, tested code maps OutOfBounds -> TargetOccupied rollback

            // Restored
            auto originalItem = grid.GetItemVnum(SlotIndex(0));
            CHECK(originalItem.has_value());
            CHECK(originalItem.value() == ItemVnum(123));
        }
        
        SUBCASE("Null Target Grid triggers Cancel") {
            auto startRes = manager.StartDrag(&grid, SlotIndex(0), 2, 2);
            CHECK(startRes.has_value());

            auto dropRes = manager.Drop(nullptr, SlotIndex(2));
            CHECK_FALSE(dropRes.has_value());
            CHECK(dropRes.error() == SlotDragDropError::InvalidTarget);
            
            // Restored
            auto originalItem = grid.GetItemVnum(SlotIndex(0));
            CHECK(originalItem.has_value());
            CHECK(originalItem.value() == ItemVnum(123));
        }
    }

    TEST_CASE("Cross-Grid Drop") {
        SlotItemDragDropManager manager;
        UISlotGridController grid1(5, 5);
        UISlotGridController grid2(5, 5);

        (void)grid1.PutItem(SlotIndex(0), ItemVnum(999), 1, 1);

        auto startRes = manager.StartDrag(&grid1, SlotIndex(0), 1, 1);
        CHECK(startRes.has_value());

        auto dropRes = manager.Drop(&grid2, SlotIndex(10));
        CHECK(dropRes.has_value());

        CHECK_FALSE(grid1.GetItemVnum(SlotIndex(0)).has_value());
        
        auto targetItem = grid2.GetItemVnum(SlotIndex(10));
        CHECK(targetItem.has_value());
        CHECK(targetItem.value() == ItemVnum(999));
    }
}
