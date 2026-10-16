#define DOCTEST_CONFIG_IMPLEMENT_WITH_MAIN
#include "doctest/doctest.h"
#include "Client/Terrain/TerrainEnvironmentConfigManager.h"

TEST_CASE("TerrainEnvironmentConfigManager: Configuration store and read") {
    using namespace Client::Terrain;
    TerrainEnvironmentConfigManager mgr;
    EnvironmentParams p{1000.0f, 8000.0f, 0.005f, true};
    mgr.SetParams(p);
    auto res = mgr.GetParams();
    CHECK(res.fogStart == 1000.0f);
    CHECK(res.nightMode);
}
