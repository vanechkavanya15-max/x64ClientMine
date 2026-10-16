#define DOCTEST_CONFIG_IMPLEMENT_WITH_MAIN
#include "doctest/doctest.h"
#include "Client/UI/WindowManagerLayoutApiBridge.h"

TEST_CASE("WindowManagerLayoutApiBridge: Coordinate calculations and alignment") {
    Client::UI::WindowManagerLayoutApiBridge bridge;
    bridge.SetWindowPosition(1, 10, 20);
    bridge.SetWindowSize(1, 100, 50);

    SUBCASE("Left-Top alignment") {
        auto r = bridge.CalculateGlobalRect(1, 800, 600);
        CHECK(r.left == 10);
        CHECK(r.top == 20);
        CHECK(r.Width() == 100);
        CHECK(r.Height() == 50);
    }
    SUBCASE("Center-Center alignment") {
        bridge.SetHorizontalAlign(1, Client::UI::HorizontalAlign::Center);
        bridge.SetVerticalAlign(1, Client::UI::VerticalAlign::Center);
        auto r = bridge.CalculateGlobalRect(1, 800, 600);
        CHECK(r.left == 360);
        CHECK(r.top == 295);
    }
}
