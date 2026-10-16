#define DOCTEST_CONFIG_IMPLEMENT_WITH_MAIN
#include "doctest/doctest.h"
#include "Client/Input/IMEClipboardBridge.h"

TEST_CASE("IMEClipboardBridge: Text sanitize and clipboard storage") {
    Client::Input::IMEClipboardBridge clip;
    std::string dirty = "Text\x01\x02\x03WithControl";
    clip.SetClipboardText(dirty);
    CHECK(clip.GetClipboardText() == "TextWithControl");
}
