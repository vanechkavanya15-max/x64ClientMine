#define DOCTEST_CONFIG_IMPLEMENT_WITH_MAIN
#include "doctest/doctest.h"
#include "Client/UI/WindowManagerInputApiBridge.h"

TEST_CASE("WindowManagerInputApiBridge: Focus, capture and drag-drop icon attach") {
    Client::UI::WindowManagerInputApiBridge bridge;
    SUBCASE("Focus and capture tracking") {
        bridge.SetCapture(42);
        CHECK(bridge.GetCaptureWindow() == 42);
        bridge.ReleaseCapture();
        CHECK_FALSE(bridge.GetCaptureWindow().has_value());

        bridge.SetFocus(99);
        CHECK(bridge.GetFocusWindow() == 99);
        bridge.KillFocus();
        CHECK_FALSE(bridge.GetFocusWindow().has_value());
    }
    SUBCASE("Drag and drop icon attachment") {
        CHECK_FALSE(bridge.IsAttaching());
        bridge.AttachIcon(1, 5, 101, 10);
        CHECK(bridge.IsAttaching());
        auto data = bridge.GetAttachingData();
        CHECK(data.itemVnum == 101);
        CHECK(data.count == 10);
        bridge.DeattachIcon();
        CHECK_FALSE(bridge.IsAttaching());
    }
}
