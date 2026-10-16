#define DOCTEST_CONFIG_IMPLEMENT_WITH_MAIN
#include "doctest/doctest.h"
#include "Client/Terrain/TerrainSectorLoader.h"

using namespace Client::Terrain;

TEST_CASE("TerrainSectorLoader: LRU cache and chunk streaming") {
    TerrainSectorLoader loader(3); // Capacity = 3

    SUBCASE("Load and cache chunks") {
        CHECK(loader.LoadSector(1));
        CHECK(loader.LoadSector(2));
        CHECK(loader.LoadSector(3));
        CHECK(loader.GetLoadedSectorCount() == 3);

        // Loading 4th chunk evicts oldest (1)
        CHECK(loader.LoadSector(4));
        CHECK(loader.GetLoadedSectorCount() == 3);
        CHECK_FALSE(loader.IsSectorLoaded(1));
        CHECK(loader.IsSectorLoaded(4));
    }

    SUBCASE("Touch keeps chunk active") {
        loader.LoadSector(1);
        loader.LoadSector(2);
        loader.LoadSector(3);

        // Access sector 1 so it becomes most recently used
        loader.LoadSector(1); // touch

        // Load 4 -> sector 2 should be evicted instead of 1
        loader.LoadSector(4);
        CHECK(loader.IsSectorLoaded(1));
        CHECK_FALSE(loader.IsSectorLoaded(2));
    }
}
