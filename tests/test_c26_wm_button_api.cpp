#define DOCTEST_CONFIG_IMPLEMENT_WITH_MAIN
#include "doctest/doctest.h"
#include "Client/UI/WindowManagerButtonApiBridge.h"

using namespace Client::UI;

TEST_CASE("WindowManagerButtonApiBridge: Registration and Basic Operations") {
    WindowManagerButtonApiBridge bridge;
    ButtonId id1{10};

    SUBCASE("Register and Unregister") {
        CHECK(bridge.RegisterButton(id1).has_value());
        CHECK(bridge.RegisterButton(id1).error() == ButtonError::AlreadyExists); // Duplicate
        
        CHECK(bridge.UnregisterButton(id1).has_value());
        CHECK(bridge.UnregisterButton(id1).error() == ButtonError::NotFound);
    }

    SUBCASE("Type and State Setters") {
        bridge.RegisterButton(id1, ButtonType::Push);
        
        CHECK(bridge.SetButtonType(id1, ButtonType::Toggle).has_value());
        CHECK(bridge.SetButtonState(id1, ButtonVisualState::Over).has_value());
        CHECK(bridge.GetButtonVisualState(id1).value() == ButtonVisualState::Over);
        
        ButtonId invalidId{999};
        CHECK(bridge.SetButtonType(invalidId, ButtonType::Toggle).error() == ButtonError::NotFound);
    }

    SUBCASE("Enable and Disable") {
        bridge.RegisterButton(id1);
        
        CHECK(bridge.IsButtonEnabled(id1).value() == true);
        CHECK(bridge.SetButtonEnable(id1, false).has_value());
        CHECK(bridge.IsButtonEnabled(id1).value() == false);
        CHECK(bridge.GetButtonVisualState(id1).value() == ButtonVisualState::Disable);

        // Cannot interact when disabled
        CHECK(bridge.SimulateMouseEnter(id1).error() == ButtonError::IsDisabled);
        CHECK(bridge.SimulateMousePress(id1).error() == ButtonError::IsDisabled);
    }

    SUBCASE("Flash and Tooltip") {
        bridge.RegisterButton(id1);
        
        CHECK(bridge.SetButtonFlash(id1, true).has_value());
        CHECK(bridge.IsButtonFlashing(id1).value() == true);
        
        CHECK(bridge.SetButtonTooltip(id1, "Test Tooltip").has_value());
        CHECK(bridge.GetButtonTooltip(id1).value() == "Test Tooltip");
    }
}

TEST_CASE("WindowManagerButtonApiBridge: Mouse Simulation") {
    WindowManagerButtonApiBridge bridge;
    ButtonId pId{1};
    ButtonId tId{2};
    ButtonId rId1{3};
    ButtonId rId2{4};

    bridge.RegisterButton(pId, ButtonType::Push);
    bridge.RegisterButton(tId, ButtonType::Toggle);
    bridge.RegisterButton(rId1, ButtonType::Radio, 100);
    bridge.RegisterButton(rId2, ButtonType::Radio, 100);

    SUBCASE("Push Button") {
        CHECK(bridge.GetButtonVisualState(pId).value() == ButtonVisualState::Normal);
        
        bridge.SimulateMouseEnter(pId);
        CHECK(bridge.GetButtonVisualState(pId).value() == ButtonVisualState::Over);
        
        bridge.SimulateMousePress(pId);
        CHECK(bridge.GetButtonVisualState(pId).value() == ButtonVisualState::Down);
        
        bridge.SimulateMouseRelease(pId);
        // Po zwolnieniu (skoro nadal jesteśmy na hover) powinien wrocic do Over
        CHECK(bridge.GetButtonVisualState(pId).value() == ButtonVisualState::Over);
        CHECK(bridge.IsButtonDown(pId).value() == false); // Push button nie zostaje wcisniety
        
        bridge.SimulateMouseLeave(pId);
        CHECK(bridge.GetButtonVisualState(pId).value() == ButtonVisualState::Normal);
    }

    SUBCASE("Push Button - Mouse Leave while Pressed") {
        bridge.SimulateMouseEnter(pId);
        bridge.SimulateMousePress(pId);
        CHECK(bridge.GetButtonVisualState(pId).value() == ButtonVisualState::Down);
        
        bridge.SimulateMouseLeave(pId);
        CHECK(bridge.GetButtonVisualState(pId).value() == ButtonVisualState::Normal);
        // Release outside shouldn't trigger action
        bridge.SimulateMouseRelease(pId);
        CHECK(bridge.IsButtonDown(pId).value() == false);
    }

    SUBCASE("Toggle Button") {
        bridge.SimulateMouseEnter(tId);
        bridge.SimulateMousePress(tId);
        bridge.SimulateMouseRelease(tId);
        
        CHECK(bridge.IsButtonDown(tId).value() == true);
        CHECK(bridge.GetButtonVisualState(tId).value() == ButtonVisualState::Down);
        
        // Drugie klikniecie wylacza
        bridge.SimulateMousePress(tId);
        bridge.SimulateMouseRelease(tId);
        
        CHECK(bridge.IsButtonDown(tId).value() == false);
        // Nadal jest hover
        CHECK(bridge.GetButtonVisualState(tId).value() == ButtonVisualState::Over);
    }

    SUBCASE("Radio Buttons Exclusivity") {
        bridge.SimulateMouseEnter(rId1);
        bridge.SimulateMousePress(rId1);
        bridge.SimulateMouseRelease(rId1);
        
        CHECK(bridge.IsButtonDown(rId1).value() == true);
        CHECK(bridge.GetButtonVisualState(rId1).value() == ButtonVisualState::Down);
        
        bridge.SimulateMouseEnter(rId2);
        bridge.SimulateMousePress(rId2);
        bridge.SimulateMouseRelease(rId2);
        
        CHECK(bridge.IsButtonDown(rId2).value() == true);
        CHECK(bridge.IsButtonDown(rId1).value() == false);
        CHECK(bridge.GetButtonVisualState(rId1).value() == ButtonVisualState::Normal);
        CHECK(bridge.GetButtonVisualState(rId2).value() == ButtonVisualState::Down);
    }

    SUBCASE("Manual SetButtonDown on Radio") {
        CHECK(bridge.SetButtonDown(rId1, true).has_value());
        CHECK(bridge.IsButtonDown(rId1).value() == true);
        
        CHECK(bridge.SetButtonDown(rId2, true).has_value());
        CHECK(bridge.IsButtonDown(rId2).value() == true);
        CHECK(bridge.IsButtonDown(rId1).value() == false);
    }
}
