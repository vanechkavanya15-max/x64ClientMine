#define DOCTEST_CONFIG_IMPLEMENT_WITH_MAIN
#include "doctest/doctest.h"
#include "Client/Terrain/TerrainHeightCollisionQuerier.h"

TEST_CASE("TerrainHeightCollisionQuerier: Height and water level checks") {
    using namespace Client::Terrain;
    float h = TerrainHeightCollisionQuerier::GetHeight(0.0f, 0.0f);
    CHECK(h == doctest::Approx(60.0f));
    CHECK(TerrainHeightCollisionQuerier::GetWaterHeight() == 30.0f);
    CHECK_FALSE(TerrainHeightCollisionQuerier::IsUnderWater(0.0f, 0.0f));
}
