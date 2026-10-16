#define DOCTEST_CONFIG_IMPLEMENT_WITH_MAIN
#include "doctest/doctest.h"
#include "Client/UI/WindowManagerSlotApiBridge.h"

TEST_CASE("WindowManagerSlotApiBridge: Slot operations, locking and cooltime") {
    Client::UI::WindowManagerSlotApiBridge bridge;
    SUBCASE("Set and get slots") {
        CHECK(bridge.SetSlot(0, 101, 5));
        auto slot = bridge.GetSlot(0);
        REQUIRE(slot.has_value());
        CHECK(slot->itemVnum == 101);
        CHECK(slot->count == 5);
        CHECK_FALSE(slot->locked);
    }
    SUBCASE("Slot locking") {
        bridge.SetSlot(1, 202, 1);
        CHECK(bridge.LockSlot(1));
        CHECK(bridge.IsSlotLocked(1));
        CHECK(bridge.UnlockSlot(1));
        CHECK_FALSE(bridge.IsSlotLocked(1));
    }
    SUBCASE("Cooltime management and updates") {
        bridge.SetSlot(2, 303, 1);
        CHECK(bridge.SetSlotCoolTime(2, 5.0f, 0.0f));
        auto slot = bridge.GetSlot(2);
        REQUIRE(slot.has_value());
        CHECK(slot->coolTime.active);

        bridge.Update(2.5f);
        slot = bridge.GetSlot(2);
        CHECK(slot->coolTime.active);
        CHECK(slot->coolTime.elapsed == doctest::Approx(2.5f));

        bridge.Update(3.0f);
        slot = bridge.GetSlot(2);
        CHECK_FALSE(slot->coolTime.active);
    }
}
