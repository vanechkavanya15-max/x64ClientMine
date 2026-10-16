#define DOCTEST_CONFIG_IMPLEMENT_WITH_MAIN
#include "doctest/doctest.h"
#include "Client/UI/WindowManagerSlotApiBridge.h"
#include "Client/Core/StrongTypes.h"

TEST_CASE("WindowManagerSlotApiBridge: Slot operations, locking and cooltime") {
    Client::UI::WindowManagerSlotApiBridge bridge;
    
    using namespace Client::Core;
    using namespace Client::UI;

    SUBCASE("Set and get slots") {
        auto res = bridge.SetSlot(SlotIndex(0), ItemVnum(101), 5);
        CHECK(res.has_value());
        
        auto slot = bridge.GetSlot(SlotIndex(0));
        REQUIRE(slot.has_value());
        CHECK(slot->itemVnum == ItemVnum(101));
        CHECK(slot->count == 5);
        CHECK_FALSE(slot->locked);
    }
    
    SUBCASE("Slot locking and error handling") {
        bridge.SetSlot(SlotIndex(1), ItemVnum(202), 1);
        
        auto res = bridge.LockSlot(SlotIndex(1));
        CHECK(res.has_value());
        CHECK(bridge.IsSlotLocked(SlotIndex(1)));
        
        res = bridge.UnlockSlot(SlotIndex(1));
        CHECK(res.has_value());
        CHECK_FALSE(bridge.IsSlotLocked(SlotIndex(1)));
        
        auto errRes = bridge.LockSlot(SlotIndex(99));
        CHECK_FALSE(errRes.has_value());
        CHECK(errRes.error() == SlotApiError::SlotNotFound);
    }
    
    SUBCASE("Layered cooltime management and updates") {
        bridge.SetSlot(SlotIndex(2), ItemVnum(303), 1);
        
        auto res = bridge.SetSlotCoolTime(SlotIndex(2), 0, 5.0f, 0.0f); // Layer 0
        CHECK(res.has_value());
        res = bridge.SetSlotCoolTime(SlotIndex(2), 1, 10.0f, 0.0f); // Layer 1
        CHECK(res.has_value());
        
        auto slot = bridge.GetSlot(SlotIndex(2));
        REQUIRE(slot.has_value());
        CHECK(slot->coolTimes[0].active);
        CHECK(slot->coolTimes[1].active);

        // Pause layer 1
        res = bridge.PauseSlotCoolTime(SlotIndex(2), 1);
        CHECK(res.has_value());

        bridge.Update(2.5f);
        
        slot = bridge.GetSlot(SlotIndex(2));
        CHECK(slot->coolTimes[0].active);
        CHECK(slot->coolTimes[0].elapsed == doctest::Approx(2.5f));
        
        // Layer 1 is paused
        CHECK(slot->coolTimes[1].active);
        CHECK(slot->coolTimes[1].elapsed == doctest::Approx(0.0f));

        // Resume and update
        res = bridge.ResumeSlotCoolTime(SlotIndex(2), 1);
        CHECK(res.has_value());
        
        bridge.Update(3.0f);
        slot = bridge.GetSlot(SlotIndex(2));
        
        // Layer 0 finished (2.5 + 3.0 > 5.0)
        CHECK_FALSE(slot->coolTimes[0].active);
        CHECK(slot->coolTimes[0].elapsed == doctest::Approx(5.0f));
        
        // Layer 1 active (0.0 + 3.0 < 10.0)
        CHECK(slot->coolTimes[1].active);
        CHECK(slot->coolTimes[1].elapsed == doctest::Approx(3.0f));
    }
    
    SUBCASE("Slot swapping") {
        bridge.SetSlot(SlotIndex(3), ItemVnum(404), 10);
        bridge.SetSlot(SlotIndex(4), ItemVnum(505), 20);
        
        auto res = bridge.SwapSlots(SlotIndex(3), SlotIndex(4));
        CHECK(res.has_value());
        
        auto slotA = bridge.GetSlot(SlotIndex(3));
        auto slotB = bridge.GetSlot(SlotIndex(4));
        
        REQUIRE(slotA.has_value());
        REQUIRE(slotB.has_value());
        
        CHECK(slotA->itemVnum == ItemVnum(505));
        CHECK(slotA->count == 20);
        CHECK(slotB->itemVnum == ItemVnum(404));
        CHECK(slotB->count == 10);
    }
    
    SUBCASE("Slot swapping with locked slots") {
        bridge.SetSlot(SlotIndex(5), ItemVnum(606), 1);
        bridge.SetSlot(SlotIndex(6), ItemVnum(707), 1);
        
        bridge.LockSlot(SlotIndex(5));
        
        auto res = bridge.SwapSlots(SlotIndex(5), SlotIndex(6));
        CHECK_FALSE(res.has_value());
        CHECK(res.error() == SlotApiError::SlotLocked);
    }
    
    SUBCASE("Slot cover management") {
        bridge.SetSlot(SlotIndex(7), ItemVnum(808), 1);
        
        auto res = bridge.SetSlotCover(SlotIndex(7), 1);
        CHECK(res.has_value());
        
        auto slot = bridge.GetSlot(SlotIndex(7));
        REQUIRE(slot.has_value());
        CHECK(slot->cover.active);
        CHECK(slot->cover.coverType == 1);
        
        res = bridge.ClearSlotCover(SlotIndex(7));
        CHECK(res.has_value());
        
        slot = bridge.GetSlot(SlotIndex(7));
        CHECK_FALSE(slot->cover.active);
        CHECK(slot->cover.coverType == 0);
    }

    SUBCASE("SetSlotCount operations") {
        bridge.SetSlot(SlotIndex(10), ItemVnum(909), 1);
        auto res = bridge.SetSlotCount(SlotIndex(10), 50);
        CHECK(res.has_value());
        
        auto slot = bridge.GetSlot(SlotIndex(10));
        REQUIRE(slot.has_value());
        CHECK(slot->count == 50);

        // Non-existent slot created with count
        auto res2 = bridge.SetSlotCount(SlotIndex(11), 25);
        CHECK(res2.has_value());
        auto slot2 = bridge.GetSlot(SlotIndex(11));
        REQUIRE(slot2.has_value());
        CHECK(slot2->count == 25);
    }

    SUBCASE("Drag and drop / Attaching icon operations") {
        CHECK_FALSE(bridge.IsAttaching());
        CHECK_FALSE(bridge.IsDragging());

        bridge.AttachIcon(1, 5001, 3, 32, 64);
        CHECK(bridge.IsAttaching());
        CHECK(bridge.IsDragging());

        const auto& icon = bridge.GetAttachedIcon();
        CHECK(icon.type == 1);
        CHECK(icon.index == 5001);
        CHECK(icon.slotNumber == 3);
        CHECK(icon.width == 32);
        CHECK(icon.height == 64);

        bridge.SetAttachingFlag(false);
        CHECK_FALSE(bridge.IsAttaching());

        bridge.AttachIcon(2, 6002, 5, 32, 32);
        CHECK(bridge.IsAttaching());
        bridge.DeattachIcon();
        CHECK_FALSE(bridge.IsAttaching());
        CHECK_FALSE(bridge.IsDragging());
    }
}

