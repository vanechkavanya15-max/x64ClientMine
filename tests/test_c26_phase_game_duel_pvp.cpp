#define DOCTEST_CONFIG_IMPLEMENT_WITH_MAIN
#include "doctest/doctest.h"
#include "Client/Bridge/PhaseGameDuelPvpBridge.h"

TEST_CASE("PhaseGameDuelPvpBridge: Duel lifecycle") {
    Client::Bridge::PhaseGameDuelPvpBridge bridge;
    CHECK_FALSE(bridge.IsInDuel());
    bridge.StartDuel(4002);
    CHECK(bridge.IsInDuel());
    CHECK(bridge.GetOpponentVid() == 4002);
    bridge.EndDuel();
    CHECK_FALSE(bridge.IsInDuel());
}
