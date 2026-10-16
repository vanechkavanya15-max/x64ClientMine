#define DOCTEST_CONFIG_IMPLEMENT_WITH_MAIN
#include "doctest.h"
#include "../src/Client/Input/KeyboardInputDispatcher.h"
#include <new>

using namespace Client::Input;

static bool g_simulateAllocFailure = false;

void* operator new(std::size_t size) {
    if (g_simulateAllocFailure) {
        throw std::bad_alloc();
    }
    return std::malloc(size);
}

void* operator new[](std::size_t size) {
    if (g_simulateAllocFailure) {
        throw std::bad_alloc();
    }
    return std::malloc(size);
}

void operator delete(void* ptr) noexcept {
    std::free(ptr);
}

void operator delete[](void* ptr) noexcept {
    std::free(ptr);
}

void operator delete(void* ptr, std::size_t) noexcept {
    std::free(ptr);
}

void operator delete[](void* ptr, std::size_t) noexcept {
    std::free(ptr);
}

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
    
    (void)dispatcher.RegisterHandler(key, handler);
    
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
        (void)dispatcher.RegisterHandler(key, [](KeyState){});
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

    SUBCASE("Allocation Failed") {
        g_simulateAllocFailure = true;
        auto res = dispatcher.RegisterHandler(key, [](KeyState){});
        g_simulateAllocFailure = false;
        CHECK(!res.has_value());
        CHECK(res.error() == KeyboardError::AllocationFailed);
    }
}

TEST_CASE("KeyboardInputDispatcher - Clear") {
    KeyboardInputDispatcher dispatcher;
    KeyCode key{42};
    (void)dispatcher.RegisterHandler(key, [](KeyState){});
    
    dispatcher.Clear();
    auto res = dispatcher.Dispatch(key, KeyState::Pressed);
    CHECK(!res.has_value());
    CHECK(res.error() == KeyboardError::HandlerNotFound);
}
