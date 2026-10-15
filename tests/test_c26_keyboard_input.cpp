#define DOCTEST_CONFIG_IMPLEMENT_WITH_MAIN
#include "doctest.h"
#include "../src/Client/Input/KeyboardInputDispatcher.h"

using namespace Client::Input;

TEST_CASE("KeyboardInputDispatcher - Register and Dispatch") {
    KeyboardInputDispatcher dispatcher;
    
    bool called = false;
    KeyState receivedState = KeyState::Released;
    
    KeyCode key{42};
    auto handler = [&](KeyState state) {
        called = true;
        receivedState = state;
    };
    
    auto regResult = dispatcher.RegisterHandler(key, handler);
    CHECK(regResult.has_value());
    
    auto dispatchResult = dispatcher.Dispatch(key, KeyState::Pressed);
    CHECK(dispatchResult.has_value());
    CHECK(called == true);
    CHECK(receivedState == KeyState::Pressed);
}

TEST_CASE("KeyboardInputDispatcher - Unregister") {
    KeyboardInputDispatcher dispatcher;
    
    KeyCode key{42};
    auto handler = [&](KeyState) {};
    
    dispatcher.RegisterHandler(key, handler);
    
    auto unregResult = dispatcher.UnregisterHandler(key);
    CHECK(unregResult.has_value());
    
    auto dispatchResult = dispatcher.Dispatch(key, KeyState::Pressed);
    CHECK(!dispatchResult.has_value());
    CHECK(dispatchResult.error() == KeyboardError::HandlerNotFound);
}

TEST_CASE("KeyboardInputDispatcher - Errors") {
    KeyboardInputDispatcher dispatcher;
    KeyCode key{42};
    
    SUBCASE("Already Registered") {
        dispatcher.RegisterHandler(key, [](KeyState){});
        auto res = dispatcher.RegisterHandler(key, [](KeyState){});
        CHECK(!res.has_value());
        CHECK(res.error() == KeyboardError::AlreadyRegistered);
    }
    
    SUBCASE("Invalid Key / Null Handler") {
        auto res = dispatcher.RegisterHandler(key, nullptr);
        CHECK(!res.has_value());
        CHECK(res.error() == KeyboardError::InvalidKey);
    }
    
    SUBCASE("Unregister NotFound") {
        auto res = dispatcher.UnregisterHandler(key);
        CHECK(!res.has_value());
        CHECK(res.error() == KeyboardError::HandlerNotFound);
    }
}

TEST_CASE("KeyboardInputDispatcher - Clear") {
    KeyboardInputDispatcher dispatcher;
    KeyCode key{42};
    dispatcher.RegisterHandler(key, [](KeyState){});
    
    dispatcher.Clear();
    auto res = dispatcher.Dispatch(key, KeyState::Pressed);
    CHECK(!res.has_value());
    CHECK(res.error() == KeyboardError::HandlerNotFound);
}
