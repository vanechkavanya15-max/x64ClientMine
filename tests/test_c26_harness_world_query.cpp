#define DOCTEST_CONFIG_IMPLEMENT_WITH_MAIN
#include "doctest/doctest.h"
#include "Client/IPC/HarnessWorldQueryRouter.h"

TEST_CASE("HarnessWorldQueryRouter: World snapshot JSON serialization") {
    Client::IPC::HarnessWorldQueryRouter router;
    router.AddEntity({1001, 101, 50.0f, 60.0f, 500, 500});
    router.AddEntity({1002, 8001, 70.0f, 80.0f, 10000, 10000});

    std::string json = router.SerializeWorldSnapshot(100);
    CHECK(json.find("\"player_vid\":100") != std::string::npos);
    CHECK(json.find("\"entity_count\":2") != std::string::npos);
    CHECK(json.find("\"vid\":1001") != std::string::npos);
}
