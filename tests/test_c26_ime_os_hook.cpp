#define DOCTEST_CONFIG_IMPLEMENT_WITH_MAIN
#include "doctest/doctest.h"
#include "Client/Input/IMEOsHookController.h"

TEST_CASE("IMEOsHookController: Input capture state") {
    Client::Input::IMEOsHookController hook;
    CHECK(hook.IsCapturing());
    hook.EnableInputCapture(false);
    CHECK_FALSE(hook.IsCapturing());
    hook.SetReadingInformation(true);
    CHECK(hook.HasReadingInformation());
}
