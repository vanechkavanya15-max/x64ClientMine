#define DOCTEST_CONFIG_IMPLEMENT_WITH_MAIN
#include "doctest/doctest.h"
#include "Client/Terrain/TerrainCoordinateMapper.h"

TEST_CASE("TerrainCoordinateMapper: Sector indexing") {
    using namespace Client::Terrain;
    CHECK(TerrainCoordinateMapper::GetTerrainNum(100.0f, 100.0f, 3) == 0);
    CHECK(TerrainCoordinateMapper::GetTerrainNum(25700.0f, 100.0f, 3) == 1);
    CHECK(TerrainCoordinateMapper::GetTerrainNum(100.0f, 25700.0f, 3) == 3);
}
