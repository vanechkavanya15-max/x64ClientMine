#include <doctest.h>
#include "Client/Events/UIEvents.h"
#include "EterBase/EventBus.h"

using namespace Client::Events;

TEST_CASE("CPythonPlayerInput and CPythonEventManager UI events dispatch") {
    bool attackEventReceived = false;
    bool sizeEventReceived = false;
    bool questEventReceived = false;
    
    auto sub1 = ::EterBase::EventBus::Instance().Subscribe<UICannotAttackEvent>([&](const UICannotAttackEvent& ev) {
        attackEventReceived = true;
        CHECK(ev.mainCharIndex == 1);
        CHECK(ev.reason == "IN_SAFE");
    });

    auto sub2 = ::EterBase::EventBus::Instance().Subscribe<UISizeEvent>([&](const UISizeEvent& ev) {
        sizeEventReceived = true;
        CHECK(ev.width == 800);
        CHECK(ev.height == 600);
        CHECK(ev.uiContext == nullptr);
    });

    auto sub3 = ::EterBase::EventBus::Instance().Subscribe<UIRecvQuestEvent>([&](const UIRecvQuestEvent& ev) {
        questEventReceived = true;
        CHECK(ev.index == 42);
        CHECK(ev.name == "Main Quest");
    });

    SUBCASE("PlayerInput events are emitted correctly") {
        ::EterBase::EventBus::Instance().Publish(UICannotAttackEvent(1, "IN_SAFE"));
        CHECK(attackEventReceived);
    }

    SUBCASE("EventManager UI size event is emitted correctly") {
        ::EterBase::EventBus::Instance().Publish(UISizeEvent(nullptr, 800, 600));
        CHECK(sizeEventReceived);
    }

    SUBCASE("EventManager UI quest event is emitted correctly") {
        ::EterBase::EventBus::Instance().Publish(UIRecvQuestEvent(nullptr, 42, "Main Quest"));
        CHECK(questEventReceived);
    }

    ::EterBase::EventBus::Instance().Unsubscribe<UICannotAttackEvent>(sub1);
    ::EterBase::EventBus::Instance().Unsubscribe<UISizeEvent>(sub2);
    ::EterBase::EventBus::Instance().Unsubscribe<UIRecvQuestEvent>(sub3);
}

TEST_CASE("All specific UI events construction and validation") {
    SUBCASE("UIPCTargetBoardSetEvent") {
        UIPCTargetBoardSetEvent ev(nullptr, 123, "TestTarget");
        CHECK(ev.virtualID == 123);
        CHECK(ev.name == "TestTarget");
    }

    SUBCASE("UIInsertItemIconEvent") {
        UIInsertItemIconEvent ev(nullptr, "icon", 55, "Sword", "A sharp sword", 1, 10);
        CHECK(ev.itemIndex == 55);
        CHECK(ev.title == "Sword");
    }

    SUBCASE("UIShowDungeonResultEvent") {
        UIShowDungeonResultEvent ev(nullptr, 1, 2, 3, 4, 5, 6, 7, 8, 9);
        CHECK(ev.args[0] == 1);
        CHECK(ev.args[8] == 9);
    }
}
