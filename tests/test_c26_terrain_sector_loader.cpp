#define DOCTEST_CONFIG_IMPLEMENT_WITH_MAIN
#include "doctest/doctest.h"
#include "Client/Terrain/TerrainSectorLoader.h"

TEST_CASE("TerrainSectorLoader: Sector streaming lifecycle") {
    Client::Terrain::TerrainSectorLoader loader;
    CHECK(loader.LoadSector(1));
    CHECK(loader.IsSectorLoaded(1));
    CHECK_FALSE(loader.LoadSector(1)); // already loaded
    CHECK(loader.GetLoadedSectorCount() == 1);
    CHECK(loader.UnloadSector(1));
    CHECK_FALSE(loader.IsSectorLoaded(1));
}
