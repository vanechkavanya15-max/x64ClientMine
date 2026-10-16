#define DOCTEST_CONFIG_IMPLEMENT_WITH_MAIN
#include "doctest.h"
#include "Client/UI/UIWindowEventListener.h"

using namespace Client::UI;

TEST_CASE("UIWindowEventListener - Register and Unregister MouseListener") {
    UIWindowEventListener listener;
    Window win("TestWin");

    SUBCASE("Registering valid listener") {
        auto res = listener.RegisterMouseListener(&win, [](Window*, const MouseEvent&) { return true; });
        CHECK(res.has_value());
    }

    SUBCASE("Registering nullptr window") {
        auto res = listener.RegisterMouseListener(nullptr, [](Window*, const MouseEvent&) { return true; });
        CHECK(!res.has_value());
        CHECK(res.error() == UIListenerError::WindowNullptr);
    }

    SUBCASE("Registering same window twice") {
        listener.RegisterMouseListener(&win, [](Window*, const MouseEvent&) { return true; });
        auto res = listener.RegisterMouseListener(&win, [](Window*, const MouseEvent&) { return true; });
        CHECK(!res.has_value());
        CHECK(res.error() == UIListenerError::AlreadyRegistered);
    }

    SUBCASE("Unregistering valid listener") {
        listener.RegisterMouseListener(&win, [](Window*, const MouseEvent&) { return true; });
        auto res = listener.UnregisterMouseListener(&win);
        CHECK(res.has_value());
    }

    SUBCASE("Unregistering not found") {
        auto res = listener.UnregisterMouseListener(&win);
        CHECK(!res.has_value());
        CHECK(res.error() == UIListenerError::NotFound);
    }
}

TEST_CASE("UIWindowEventListener - Register and Unregister DragDropListener") {
    UIWindowEventListener listener;
    Window win("TestWin");

    SUBCASE("Registering valid listener") {
        auto res = listener.RegisterDragDropListener(&win, [](Window*, Window*, const DragDropPayload&) { return true; });
        CHECK(res.has_value());
    }

    SUBCASE("Registering nullptr window") {
        auto res = listener.RegisterDragDropListener(nullptr, [](Window*, Window*, const DragDropPayload&) { return true; });
        CHECK(!res.has_value());
        CHECK(res.error() == UIListenerError::WindowNullptr);
    }

    SUBCASE("Registering same window twice") {
        listener.RegisterDragDropListener(&win, [](Window*, Window*, const DragDropPayload&) { return true; });
        auto res = listener.RegisterDragDropListener(&win, [](Window*, Window*, const DragDropPayload&) { return true; });
        CHECK(!res.has_value());
        CHECK(res.error() == UIListenerError::AlreadyRegistered);
    }

    SUBCASE("Unregistering valid listener") {
        listener.RegisterDragDropListener(&win, [](Window*, Window*, const DragDropPayload&) { return true; });
        auto res = listener.UnregisterDragDropListener(&win);
        CHECK(res.has_value());
    }
}

TEST_CASE("UIWindowEventListener - Dispatching Events") {
    UIWindowEventListener listener;
    Window win("TestWin");
    Window sourceWin("SourceWin");

    SUBCASE("Dispatch Mouse Event") {
        bool called = false;
        listener.RegisterMouseListener(&win, [&called](Window* w, const MouseEvent& e) {
            called = true;
            CHECK(e.type == MouseEventType::OnClick);
            CHECK(e.position.x == 10);
            return true;
        });

        bool handled = listener.DispatchMouseEvent(&win, MouseEvent{MouseEventType::OnClick, Point{10, 20}});
        CHECK(called);
        CHECK(handled);
    }

    SUBCASE("Dispatch DragDrop Event") {
        bool called = false;
        listener.RegisterDragDropListener(&win, [&called, &sourceWin](Window* target, Window* src, const DragDropPayload& payload) {
            called = true;
            CHECK(target->GetName() == "TestWin");
            CHECK(src == &sourceWin);
            CHECK(payload.dataType == "Item");
            CHECK(payload.dataId == 42);
            return true;
        });

        bool handled = listener.DispatchDragDropEvent(&win, &sourceWin, DragDropPayload{"Item", 42});
        CHECK(called);
        CHECK(handled);
    }

    SUBCASE("Dispatch on unhandled window") {
        bool handled = listener.DispatchMouseEvent(&win, MouseEvent{MouseEventType::OnClick, Point{0, 0}});
        CHECK(!handled);
    }
}

TEST_CASE("UIWindowEventListener - ClearListeners and ClearAll") {
    UIWindowEventListener listener;
    Window win1("Win1");
    Window win2("Win2");

    listener.RegisterMouseListener(&win1, [](Window*, const MouseEvent&) { return true; });
    listener.RegisterDragDropListener(&win1, [](Window*, Window*, const DragDropPayload&) { return true; });
    listener.RegisterMouseListener(&win2, [](Window*, const MouseEvent&) { return true; });

    SUBCASE("ClearListeners for specific window") {
        auto res = listener.ClearListeners(&win1);
        CHECK(res.has_value());
        
        auto res2 = listener.ClearListeners(&win1);
        CHECK(!res2.has_value());
        CHECK(res2.error() == UIListenerError::NotFound);

        // Make sure win2 is still there
        bool handled = listener.DispatchMouseEvent(&win2, MouseEvent{MouseEventType::OnClick, Point{0,0}});
        CHECK(handled);
    }

    SUBCASE("ClearAll") {
        listener.ClearAll();
        bool handled1 = listener.DispatchMouseEvent(&win1, MouseEvent{MouseEventType::OnClick, Point{0,0}});
        bool handled2 = listener.DispatchMouseEvent(&win2, MouseEvent{MouseEventType::OnClick, Point{0,0}});
        CHECK(!handled1);
        CHECK(!handled2);
    }
}
