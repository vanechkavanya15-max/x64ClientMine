#define DOCTEST_CONFIG_IMPLEMENT_WITH_MAIN
#include "doctest.h"
#include "Client/UI/WindowManagerInputApiBridge.h"

using namespace Client::UI;

TEST_CASE("WindowManagerInputApiBridge: Capture tracking") {
    WindowManagerInputApiBridge bridge;

    SUBCASE("Set and release capture normally") {
        auto res = bridge.SetCapture(42);
        CHECK(res.has_value());
        CHECK(bridge.GetCaptureWindow() == 42);

        res = bridge.ReleaseCapture();
        CHECK(res.has_value());
        CHECK_FALSE(bridge.GetCaptureWindow().has_value());
    }

    SUBCASE("Already captured by another window") {
        bridge.SetCapture(42);
        auto res = bridge.SetCapture(99);
        CHECK_FALSE(res.has_value());
        CHECK(res.error() == InputError::AlreadyCaptured);
        CHECK(bridge.GetCaptureWindow() == 42);
    }

    SUBCASE("Release capture when not captured") {
        auto res = bridge.ReleaseCapture();
        CHECK_FALSE(res.has_value());
        CHECK(res.error() == InputError::NotCaptured);
    }
}

TEST_CASE("WindowManagerInputApiBridge: Focus stack tracking") {
    WindowManagerInputApiBridge bridge;

    SUBCASE("Basic focus and kill") {
        bridge.SetFocus(10);
        CHECK(bridge.GetFocusWindow() == 10);
        
        bridge.SetFocus(20);
        CHECK(bridge.GetFocusWindow() == 20);
        
        bridge.KillFocus();
        CHECK(bridge.GetFocusWindow() == 10);
    }

    SUBCASE("Specific kill focus") {
        bridge.SetFocus(10);
        bridge.SetFocus(20);
        bridge.SetFocus(30);

        bridge.KillFocus(20);
        
        auto stack = bridge.GetFocusStack();
        CHECK(stack.size() == 2);
        CHECK(stack[0] == 10);
        CHECK(stack[1] == 30);
    }

    SUBCASE("Deduplication") {
        bridge.SetFocus(10);
        bridge.SetFocus(20);
        bridge.SetFocus(10); // Should move 10 to top

        auto stack = bridge.GetFocusStack();
        CHECK(stack.size() == 2);
        CHECK(stack[0] == 20);
        CHECK(stack[1] == 10);
        CHECK(bridge.GetFocusWindow() == 10);
    }
}

TEST_CASE("WindowManagerInputApiBridge: Icon Attach/Detach") {
    WindowManagerInputApiBridge bridge;

    SUBCASE("Basic Attach") {
        CHECK_FALSE(bridge.IsAttaching());
        bridge.AttachIcon(1, 5, 101, 10, 42);
        CHECK(bridge.IsAttaching());
        auto data = bridge.GetAttachingData();
        CHECK(data.itemVnum == 101);
        CHECK(data.count == 10);
        CHECK(data.sourceWindowId == 42);

        bridge.DeattachIcon();
        CHECK_FALSE(bridge.IsAttaching());
    }
}

TEST_CASE("WindowManagerInputApiBridge: Drag and drop distances") {
    WindowManagerInputApiBridge bridge;

    SUBCASE("Distance calculates properly") {
        bridge.SetMousePosition(100, 100);
        bridge.BeginDrag(42, MouseButton::Left);
        
        bridge.SetMousePosition(100, 150); // Moved 50px down
        CHECK(bridge.CalculateDragDistance() == doctest::Approx(50.0));

        bridge.SetMousePosition(130, 140); // Moved 30px right, 40px down -> dist 50
        CHECK(bridge.CalculateDragDistance() == doctest::Approx(50.0));

        bridge.EndDrag();
        CHECK(bridge.CalculateDragDistance() == doctest::Approx(0.0));
    }
}

TEST_CASE("WindowManagerInputApiBridge: Click timing and restriction rect") {
    WindowManagerInputApiBridge bridge;

    SUBCASE("Double click detection left mouse") {
        auto res = bridge.OnButtonDown(10, MouseButton::Left, 100);
        CHECK(res.has_value());
        
        auto upRes = bridge.OnButtonUp(10, MouseButton::Left, 150);
        CHECK(upRes.has_value());
        CHECK_FALSE(upRes.value()); // Single click

        res = bridge.OnButtonDown(10, MouseButton::Left, 200);
        upRes = bridge.OnButtonUp(10, MouseButton::Left, 250);
        CHECK(upRes.has_value());
        CHECK(upRes.value()); // Double click detected
    }

    SUBCASE("Click timeout") {
        bridge.OnButtonDown(10, MouseButton::Left, 100);
        bridge.OnButtonUp(10, MouseButton::Left, 150);

        bridge.OnButtonDown(10, MouseButton::Left, 800);
        auto upRes = bridge.OnButtonUp(10, MouseButton::Left, 850);
        CHECK(upRes.has_value());
        CHECK_FALSE(upRes.value()); // Over threshold, counts as single
    }

    SUBCASE("Input lock") {
        bridge.LockInput(1000, 500); // Locked until 1500
        
        auto res = bridge.OnButtonDown(10, MouseButton::Left, 1000);
        CHECK_FALSE(res.has_value());
        CHECK(res.error() == InputError::InputLocked);

        res = bridge.OnButtonDown(10, MouseButton::Left, 1600);
        CHECK(res.has_value());
    }

    SUBCASE("Input restriction rect") {
        bridge.SetInputRestrictionRect({10, 10, 100, 100});
        
        bridge.SetMousePosition(50, 50);
        auto res = bridge.OnButtonDown(10, MouseButton::Left, 100);
        CHECK(res.has_value()); // Inside rect
        
        bridge.SetMousePosition(200, 50);
        res = bridge.OnButtonDown(10, MouseButton::Left, 150);
        CHECK_FALSE(res.has_value());
        CHECK(res.error() == InputError::OutsideRestriction);
        
        bridge.ClearInputRestrictionRect();
        res = bridge.OnButtonDown(10, MouseButton::Left, 200);
        CHECK(res.has_value()); // Rect cleared
    }
}

TEST_CASE("WindowManagerInputApiBridge: Window Destroyed Cleanup") {
    WindowManagerInputApiBridge bridge;

    bridge.SetFocus(42);
    bridge.SetCapture(42);
    bridge.AttachIcon(1, 1, 100, 1, 42);
    bridge.BeginDrag(42, MouseButton::Left);

    CHECK(bridge.GetFocusWindow() == 42);
    CHECK(bridge.GetCaptureWindow() == 42);
    CHECK(bridge.IsAttaching());
    CHECK(bridge.GetDragContext().isDragging);

    bridge.OnWindowDestroyed(42);

    CHECK_FALSE(bridge.GetFocusWindow().has_value());
    CHECK_FALSE(bridge.GetCaptureWindow().has_value());
    CHECK_FALSE(bridge.IsAttaching());
    CHECK_FALSE(bridge.GetDragContext().isDragging);
}
