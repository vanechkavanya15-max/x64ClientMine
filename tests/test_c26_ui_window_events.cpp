#define DOCTEST_CONFIG_IMPLEMENT_WITH_MAIN
#include "doctest/doctest.h"
#include "Client/UI/UIWindowEventListenerDispatch.h"

TEST_CASE("UIWindowEventListenerDispatch: Event dispatching") {
    Client::UI::UIWindowEventListenerDispatch dispatcher;
    uint32_t clickedWindow = 0;
    dispatcher.RegisterHandler(Client::UI::UIEventType::Click, [&](uint32_t id) {
        clickedWindow = id;
    });

    CHECK(dispatcher.DispatchEvent(Client::UI::UIEventType::Click, 105));
    CHECK(clickedWindow == 105);
    CHECK_FALSE(dispatcher.DispatchEvent(Client::UI::UIEventType::DoubleClick, 105));
}
