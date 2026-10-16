#define DOCTEST_CONFIG_IMPLEMENT_WITH_MAIN
#include "doctest/doctest.h"
#include "Client/UI/UIWindowEventListenerDispatch.h"
#include "Client/UI/UIWindowHierarchyTree.h"

using namespace Client::UI;

TEST_CASE("UIWindowEventListenerDispatch: Basic dispatch and priorities") {
    UIWindowEventListenerDispatch dispatcher;
    WindowId winId(105);
    
    std::vector<int> callOrder;

    auto handler1 = [&](WindowId id, const EventPayload& p) -> bool {
        callOrder.push_back(1);
        return false; // don't consume
    };
    auto handler2 = [&](WindowId id, const EventPayload& p) -> bool {
        callOrder.push_back(2);
        return true; // consume
    };
    auto handler3 = [&](WindowId id, const EventPayload& p) -> bool {
        callOrder.push_back(3);
        return false;
    };

    // Register with priorities (higher number runs first)
    CHECK(dispatcher.RegisterHandler(winId, UIEventType::OnClick, handler1, 0).has_value());
    CHECK(dispatcher.RegisterHandler(winId, UIEventType::OnClick, handler2, 10).has_value());
    CHECK(dispatcher.RegisterHandler(winId, UIEventType::OnClick, handler3, 20).has_value());

    // Should call 3, then 2, and 2 consumes so 1 is never called.
    auto result = dispatcher.DispatchEvent(winId, UIEventType::OnClick, MouseEventData{10, 10, 1});
    CHECK(result.has_value());
    CHECK(result.value() == true);
    
    CHECK(callOrder.size() == 2);
    CHECK(callOrder[0] == 3);
    CHECK(callOrder[1] == 2);
}

TEST_CASE("UIWindowEventListenerDispatch: Payload validation") {
    UIWindowEventListenerDispatch dispatcher;
    WindowId winId(200);
    
    auto dummyHandler = [](WindowId, const EventPayload&) { return true; };
    dispatcher.RegisterHandler(winId, UIEventType::OnKeyDown, dummyHandler);

    // Valid payload
    auto res1 = dispatcher.DispatchEvent(winId, UIEventType::OnKeyDown, KeyEventData{65});
    CHECK(res1.has_value());
    CHECK(res1.value() == true);

    // Invalid payload for KeyDown (passing MouseEventData)
    auto res2 = dispatcher.DispatchEvent(winId, UIEventType::OnKeyDown, MouseEventData{0,0,0});
    CHECK(!res2.has_value());
    CHECK(res2.error() == DispatchError::PayloadMismatch);
}

TEST_CASE("UIWindowEventListenerDispatch: Event Bubbling") {
    UIWindowHierarchyTree hierarchy;
    WindowId root(1);
    WindowId child(2);
    WindowId grandchild(3);

    hierarchy.AddWindow(root);
    hierarchy.AddWindow(child);
    hierarchy.AddWindow(grandchild);
    
    hierarchy.SetParent(child, root);
    hierarchy.SetParent(grandchild, child);

    UIWindowEventListenerDispatch dispatcher;

    bool rootCalled = false;
    bool childCalled = false;
    bool grandchildCalled = false;

    dispatcher.RegisterHandler(root, UIEventType::OnChar, [&](WindowId, const EventPayload&) {
        rootCalled = true;
        return true; // Consumes event
    });
    
    dispatcher.RegisterHandler(child, UIEventType::OnChar, [&](WindowId, const EventPayload&) {
        childCalled = true;
        return false; // Doesn't consume
    });

    // Grandchild has no handler, so it should bubble to child, then to root.
    auto res = dispatcher.BubbleEvent(grandchild, UIEventType::OnChar, hierarchy, CharEventData{'a'});
    
    CHECK(res.has_value());
    CHECK(res.value() == true); // Event was consumed
    CHECK(grandchildCalled == false);
    CHECK(childCalled == true);
    CHECK(rootCalled == true);
}

TEST_CASE("UIWindowEventListenerDispatch: Unregister and Clear") {
    UIWindowEventListenerDispatch dispatcher;
    WindowId winId(300);
    
    int callCount = 0;
    auto handler = [&](WindowId, const EventPayload&) { callCount++; return true; };
    
    auto regRes = dispatcher.RegisterHandler(winId, UIEventType::OnMouseOver, handler);
    REQUIRE(regRes.has_value());
    uint64_t handlerId = regRes.value();
    
    dispatcher.DispatchEvent(winId, UIEventType::OnMouseOver);
    CHECK(callCount == 1);
    
    dispatcher.UnregisterHandler(winId, UIEventType::OnMouseOver, handlerId);
    auto res = dispatcher.DispatchEvent(winId, UIEventType::OnMouseOver);
    CHECK(res.has_value());
    CHECK(res.value() == false); // no handler to run
    CHECK(callCount == 1);

    dispatcher.RegisterHandler(winId, UIEventType::OnMouseLeave, handler);
    dispatcher.ClearAllHandlers(winId);
    auto res2 = dispatcher.DispatchEvent(winId, UIEventType::OnMouseLeave);
    CHECK(res2.has_value());
    CHECK(res2.value() == false);
}
