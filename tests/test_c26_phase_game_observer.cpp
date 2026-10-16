#define DOCTEST_CONFIG_IMPLEMENT_WITH_MAIN
#include "doctest/doctest.h"
#include "Client/Bridge/PhaseGameObserverBridge.h"

TEST_CASE("PhaseGameObserverBridge: Spectator mode") {
    Client::Bridge::PhaseGameObserverBridge obs;
    CHECK_FALSE(obs.IsObserverMode());
    obs.SetObserverMode(true);
    obs.SetTargetVid(777);
    CHECK(obs.IsObserverMode());
    CHECK(obs.GetTargetVid() == 777);
}
