#define DOCTEST_CONFIG_IMPLEMENT_WITH_MAIN
#include "doctest/doctest.h"
#include "Client/UI/UIWindowStateFlags.h"

TEST_CASE("UIWindowStateFlags: Flag operations") {
    Client::UI::UIWindowStateFlags flags;
    CHECK(flags.IsVisible());
    CHECK(flags.IsEnabled());

    flags.RemoveFlag(Client::UI::FLAG_SHOW);
    CHECK_FALSE(flags.IsVisible());

    flags.SetFlag(Client::UI::FLAG_MODAL);
    CHECK(flags.HasFlag(Client::UI::FLAG_MODAL));
}
