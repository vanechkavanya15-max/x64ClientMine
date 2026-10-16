#define DOCTEST_CONFIG_IMPLEMENT_WITH_MAIN
#include "doctest/doctest.h"
#include "Client/IPC/HarnessAuthProtocolRouter.h"

TEST_CASE("HarnessAuthProtocolRouter: Credentials and slot selection") {
    Client::IPC::HarnessAuthProtocolRouter auth;
    CHECK(auth.SetCredentials("player1", "pass123"));
    CHECK(auth.HasCredentials());
    CHECK(auth.SelectSlot(2));
    CHECK(auth.GetSelectedSlot() == 2);
    CHECK_FALSE(auth.SelectSlot(9));
    auth.SetAutoReconnect(true);
    CHECK(auth.IsAutoReconnect());
}
