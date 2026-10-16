#define DOCTEST_CONFIG_IMPLEMENT_WITH_MAIN
#include "doctest/doctest.h"
#include "Client/IPC/HarnessPlayerRpcRouter.h"

TEST_CASE("HarnessPlayerRpcRouter: Routing player RPC actions") {
    Client::IPC::HarnessPlayerRpcRouter router;
    router.RouteMove(150.0f, 300.0f);
    router.RouteAttack(5001);
    router.RouteUseItem(3);
    router.RouteSendChat("Hello World");

    const auto& hist = router.GetCommandHistory();
    REQUIRE(hist.size() == 4);
    CHECK(hist[0].method == "move");
    CHECK(hist[0].posX == 150.0f);
    CHECK(hist[1].method == "attack");
    CHECK(hist[1].targetVid == 5001);
    CHECK(hist[2].itemSlot == 3);
    CHECK(hist[3].chatMsg == "Hello World");
}
