#define DOCTEST_CONFIG_IMPLEMENT_WITH_MAIN
#include "doctest/doctest.h"
#include "Client/UI/WindowManagerButtonApiBridge.h"

TEST_CASE("WindowManagerButtonApiBridge: Button operations and radio group") {
    Client::UI::WindowManagerButtonApiBridge bridge;
    SUBCASE("Push button toggling") {
        bridge.RegisterButton(10, Client::UI::ButtonType::Push);
        CHECK(bridge.IsButtonEnabled(10));
        bridge.SetButtonDown(10, true);
        CHECK(bridge.IsButtonDown(10));
        bridge.SetButtonFlash(10, true);
        CHECK(bridge.IsButtonFlashing(10));
    }
    SUBCASE("Radio group exclusivity") {
        bridge.RegisterButton(1, Client::UI::ButtonType::Radio, 100);
        bridge.RegisterButton(2, Client::UI::ButtonType::Radio, 100);
        bridge.RegisterButton(3, Client::UI::ButtonType::Radio, 100);

        bridge.PressRadioButton(1);
        CHECK(bridge.IsButtonDown(1));
        CHECK_FALSE(bridge.IsButtonDown(2));
        CHECK_FALSE(bridge.IsButtonDown(3));

        bridge.PressRadioButton(2);
        CHECK_FALSE(bridge.IsButtonDown(1));
        CHECK(bridge.IsButtonDown(2));
        CHECK_FALSE(bridge.IsButtonDown(3));
    }
}
