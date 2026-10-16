#define DOCTEST_CONFIG_IMPLEMENT_WITH_MAIN
#include "doctest.h"

#include "Client/UI/UIModalDialogManager.h"

using namespace Client::UI;
using EterBase::EventBus;

TEST_CASE("UIModalDialogManager - Basic Operations") {
    UIModalDialogManager manager;

    SUBCASE("Push and GetActiveDialog") {
        DialogId id1{100};
        
        auto pushResult = manager.PushDialog(id1);
        REQUIRE(pushResult.has_value());
        REQUIRE(manager.GetActiveDialogCount() == 1);
        REQUIRE(manager.IsDialogActive(id1));

        auto activeResult = manager.GetActiveDialog();
        REQUIRE(activeResult.has_value());
        CHECK(activeResult.value() == id1);
    }

    SUBCASE("Pushing same dialog again fails") {
        DialogId id1{100};
        manager.PushDialog(id1);
        
        auto pushResult = manager.PushDialog(id1);
        REQUIRE_FALSE(pushResult.has_value());
        CHECK(pushResult.error() == UIError::DialogAlreadyActive);
        REQUIRE(manager.GetActiveDialogCount() == 1);
    }

    SUBCASE("Pushing invalid dialog ID fails") {
        DialogId invalidId{0};
        auto pushResult = manager.PushDialog(invalidId);
        REQUIRE_FALSE(pushResult.has_value());
        CHECK(pushResult.error() == UIError::InvalidDialogId);
    }

    SUBCASE("PopDialog") {
        DialogId id1{100};
        DialogId id2{200};
        manager.PushDialog(id1);
        manager.PushDialog(id2);

        REQUIRE(manager.GetActiveDialogCount() == 2);

        auto popResult = manager.PopDialog();
        REQUIRE(popResult.has_value());
        REQUIRE(manager.GetActiveDialogCount() == 1);

        auto activeResult = manager.GetActiveDialog();
        REQUIRE(activeResult.has_value());
        CHECK(activeResult.value() == id1);
    }

    SUBCASE("PopDialog with specific ID") {
        DialogId id1{100};
        DialogId id2{200};
        manager.PushDialog(id1);
        manager.PushDialog(id2);

        auto popResult1 = manager.PopDialog(id1); // Should fail because id2 is on top
        REQUIRE_FALSE(popResult1.has_value());
        CHECK(popResult1.error() == UIError::DialogNotFound);

        auto popResult2 = manager.PopDialog(id2); // Should succeed
        REQUIRE(popResult2.has_value());
        REQUIRE(manager.GetActiveDialogCount() == 1);
        
        auto activeResult = manager.GetActiveDialog();
        REQUIRE(activeResult.has_value());
        CHECK(activeResult.value() == id1);
    }

    SUBCASE("PopDialog when no dialog is active fails") {
        auto popResult = manager.PopDialog();
        REQUIRE_FALSE(popResult.has_value());
        CHECK(popResult.error() == UIError::NoActiveDialog);
    }

    SUBCASE("ClearAll") {
        DialogId id1{100};
        DialogId id2{200};
        DialogId id3{300};
        manager.PushDialog(id1);
        manager.PushDialog(id2);
        manager.PushDialog(id3);

        REQUIRE(manager.GetActiveDialogCount() == 3);

        manager.ClearAll();

        REQUIRE(manager.GetActiveDialogCount() == 0);
        auto popResult = manager.PopDialog();
        REQUIRE_FALSE(popResult.has_value());
        CHECK(popResult.error() == UIError::NoActiveDialog);
    }
}

TEST_CASE("UIModalDialogManager - Event Broadcasting") {
    UIModalDialogManager manager;
    
    // Zmienne do śledzenia zdarzeń
    DialogId lastPushedDialog{0};
    DialogId lastPoppedDialog{0};
    int pushCount = 0;
    int popCount = 0;

    auto pushSubId = EventBus::Instance().Subscribe<ModalDialogPushedEvent>([&](const ModalDialogPushedEvent& e) {
        lastPushedDialog = e.dialogId;
        pushCount++;
    });

    auto popSubId = EventBus::Instance().Subscribe<ModalDialogPoppedEvent>([&](const ModalDialogPoppedEvent& e) {
        lastPoppedDialog = e.dialogId;
        popCount++;
    });

    SUBCASE("Push broadcasts event") {
        DialogId id1{500};
        manager.PushDialog(id1);

        CHECK(pushCount == 1);
        CHECK(lastPushedDialog == id1);
    }

    SUBCASE("Pop broadcasts event") {
        DialogId id1{500};
        manager.PushDialog(id1);
        manager.PopDialog();

        CHECK(popCount == 1);
        CHECK(lastPoppedDialog == id1);
    }

    SUBCASE("ClearAll broadcasts events for all active dialogs") {
        DialogId id1{500};
        DialogId id2{600};
        manager.PushDialog(id1);
        manager.PushDialog(id2);
        
        int initialPopCount = popCount;
        manager.ClearAll();

        CHECK(popCount == initialPopCount + 2);
        // lastPoppedDialog will be id1 because it's popped last (from bottom of stack to top, wait, clearall pops from back)
        // Actually ClearAll pops from back() so it pops id2 then id1.
        CHECK(lastPoppedDialog == id1); 
    }

    // Unsubscribe after tests
    EventBus::Instance().Unsubscribe<ModalDialogPushedEvent>(pushSubId);
    EventBus::Instance().Unsubscribe<ModalDialogPoppedEvent>(popSubId);
}
