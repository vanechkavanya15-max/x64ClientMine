#define DOCTEST_CONFIG_IMPLEMENT_WITH_MAIN
#include "doctest.h"

#include "Client/Events/EventDialogBoxController.h"

using namespace Client::Events;

// Helper dla mockowania EventBusa jesli to potrzebne, ale my mozemy przetestowac stany wbudowane
// a zdarzenia z EventBusa mozemy ewentualnie zasubskrybowac
struct TestEventTracker {
    bool openedFired = false;
    bool closedFired = false;
    DialogEventId lastOpenedId{0};
    DialogEventId lastClosedId{0};
    std::string lastOpenedName;

    uint32_t sub1, sub2;

    TestEventTracker() {
        sub1 = EterBase::EventBus::Instance().Subscribe<EventDialogBoxOpenedEvent>(
            [this](const EventDialogBoxOpenedEvent& ev) {
                openedFired = true;
                lastOpenedId = ev.eventId;
                lastOpenedName = ev.eventName;
            }
        );
        
        sub2 = EterBase::EventBus::Instance().Subscribe<EventDialogBoxClosedEvent>(
            [this](const EventDialogBoxClosedEvent& ev) {
                closedFired = true;
                lastClosedId = ev.eventId;
            }
        );
    }

    ~TestEventTracker() {
        EterBase::EventBus::Instance().Unsubscribe<EventDialogBoxOpenedEvent>(sub1);
        EterBase::EventBus::Instance().Unsubscribe<EventDialogBoxClosedEvent>(sub2);
    }

    void Reset() {
        openedFired = false;
        closedFired = false;
        lastOpenedId = DialogEventId{0};
        lastClosedId = DialogEventId{0};
        lastOpenedName = "";
    }
};

TEST_CASE("EventDialogBoxController - Initial State") {
    EventDialogBoxController controller;

    CHECK(controller.IsDialogOpen() == false);
    CHECK(controller.GetCurrentDialogId().has_value() == false);
    
    // Update when not open should fail
    auto res = controller.Update();
    CHECK(res.has_value() == false);
    CHECK(res.error() == EventDialogError::NotOpen);

    // Close when not open should fail
    auto resClose = controller.CloseDialog();
    CHECK(resClose.has_value() == false);
    CHECK(resClose.error() == EventDialogError::NotOpen);
}

TEST_CASE("EventDialogBoxController - Open Dialog Successfully") {
    EventDialogBoxController controller;
    TestEventTracker tracker;

    DialogEventId id{42};
    std::string_view name = "Halloween Event";

    auto res = controller.OpenDialog(id, name);
    CHECK(res.has_value() == true);

    CHECK(controller.IsDialogOpen() == true);
    CHECK(controller.GetCurrentDialogId().has_value() == true);
    CHECK(controller.GetCurrentDialogId().value() == id);

    CHECK(tracker.openedFired == true);
    CHECK(tracker.lastOpenedId == id);
    CHECK(tracker.lastOpenedName == name);
    
    // Update should succeed now
    auto updateRes = controller.Update();
    CHECK(updateRes.has_value() == true);
}

TEST_CASE("EventDialogBoxController - Open Dialog Validation Failures") {
    EventDialogBoxController controller;

    SUBCASE("Invalid ID 0") {
        auto res = controller.OpenDialog(DialogEventId{0}, "Test");
        CHECK(res.has_value() == false);
        CHECK(res.error() == EventDialogError::InvalidDialogId);
    }

    SUBCASE("Empty Name") {
        auto res = controller.OpenDialog(DialogEventId{1}, "");
        CHECK(res.has_value() == false);
        CHECK(res.error() == EventDialogError::ValidationFailed);
    }
}

TEST_CASE("EventDialogBoxController - Open Already Open Dialog") {
    EventDialogBoxController controller;
    
    auto res1 = controller.OpenDialog(DialogEventId{10}, "First");
    CHECK(res1.has_value() == true);

    auto res2 = controller.OpenDialog(DialogEventId{20}, "Second");
    CHECK(res2.has_value() == false);
    CHECK(res2.error() == EventDialogError::AlreadyOpen);

    CHECK(controller.GetCurrentDialogId().value() == DialogEventId{10});
}

TEST_CASE("EventDialogBoxController - Close Dialog Successfully") {
    EventDialogBoxController controller;
    TestEventTracker tracker;

    DialogEventId id{99};
    controller.OpenDialog(id, "Christmas Event");
    tracker.Reset(); // Clear the opened event flag

    auto res = controller.CloseDialog();
    CHECK(res.has_value() == true);

    CHECK(controller.IsDialogOpen() == false);
    CHECK(controller.GetCurrentDialogId().has_value() == false);

    CHECK(tracker.closedFired == true);
    CHECK(tracker.lastClosedId == id);
}

TEST_CASE("EventDialogBoxController - Open and Close sequence") {
    EventDialogBoxController controller;
    
    CHECK(controller.OpenDialog(DialogEventId{5}, "Event5").has_value() == true);
    CHECK(controller.CloseDialog().has_value() == true);
    
    CHECK(controller.IsDialogOpen() == false);
    
    // Reopen another event
    CHECK(controller.OpenDialog(DialogEventId{6}, "Event6").has_value() == true);
    CHECK(controller.IsDialogOpen() == true);
    CHECK(controller.GetCurrentDialogId().value() == DialogEventId{6});
}
