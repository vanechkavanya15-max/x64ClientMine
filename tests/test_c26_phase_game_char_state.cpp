#define DOCTEST_CONFIG_IMPLEMENT_WITH_MAIN
#include "doctest/doctest.h"
#include "Client/Bridge/PhaseGameCharacterStateBridge.h"

TEST_CASE("PhaseGameCharacterStateBridge: Point change recording") {
    Client::Bridge::PhaseGameCharacterStateBridge bridge;
    bridge.OnPointChange(1, 1500, 100);
    const auto& hist = bridge.GetHistory();
    REQUIRE(hist.size() == 1);
    CHECK(hist[0].type == 1);
    CHECK(hist[0].value == 1500);
    CHECK(hist[0].amount == 100);
}
