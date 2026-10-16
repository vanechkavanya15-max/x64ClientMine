#define DOCTEST_CONFIG_IMPLEMENT_WITH_MAIN
#include <doctest/doctest.h>
#include "Terrain/TerrainFoliageManager.h"

using namespace Client::Terrain;
using namespace Client::Core;

TEST_CASE("TerrainFoliageManager - Addition and Basic Querying") {
    TerrainFoliageManager manager;

    FoliageId id1(1);
    MapCoords pos1{100.0f, 200.0f, 0.0f};

    SUBCASE("Successfully add foliage") {
        auto result = manager.AddFoliage(id1, pos1);
        CHECK(result.has_value());

        auto getResult = manager.GetFoliagePosition(id1);
        REQUIRE(getResult.has_value());
        CHECK(getResult.value() == pos1);
    }

    SUBCASE("Fail to add invalid id") {
        FoliageId invalidId(0); // 0 is default/invalid for FoliageId
        auto result = manager.AddFoliage(invalidId, pos1);
        REQUIRE(!result.has_value());
        CHECK(result.error() == FoliageError::InvalidId);
    }

    SUBCASE("Fail to add duplicate id") {
        manager.AddFoliage(id1, pos1);
        auto result = manager.AddFoliage(id1, {300.0f, 400.0f, 0.0f});
        REQUIRE(!result.has_value());
        CHECK(result.error() == FoliageError::AlreadyExists);
    }
}

TEST_CASE("TerrainFoliageManager - Removal") {
    TerrainFoliageManager manager;

    FoliageId id1(1);
    MapCoords pos1{100.0f, 200.0f, 0.0f};
    manager.AddFoliage(id1, pos1);

    SUBCASE("Successfully remove foliage") {
        auto result = manager.RemoveFoliage(id1);
        CHECK(result.has_value());

        auto getResult = manager.GetFoliagePosition(id1);
        REQUIRE(!getResult.has_value());
        CHECK(getResult.error() == FoliageError::NotFound);
    }

    SUBCASE("Fail to remove invalid id") {
        FoliageId invalidId(0);
        auto result = manager.RemoveFoliage(invalidId);
        REQUIRE(!result.has_value());
        CHECK(result.error() == FoliageError::InvalidId);
    }

    SUBCASE("Fail to remove non-existent id") {
        FoliageId id2(2);
        auto result = manager.RemoveFoliage(id2);
        REQUIRE(!result.has_value());
        CHECK(result.error() == FoliageError::NotFound);
    }
}

TEST_CASE("TerrainFoliageManager - GetFoliagePosition Errors") {
    TerrainFoliageManager manager;

    SUBCASE("Invalid ID") {
        FoliageId invalidId(0);
        auto result = manager.GetFoliagePosition(invalidId);
        REQUIRE(!result.has_value());
        CHECK(result.error() == FoliageError::InvalidId);
    }

    SUBCASE("Not Found") {
        FoliageId id1(1);
        auto result = manager.GetFoliagePosition(id1);
        REQUIRE(!result.has_value());
        CHECK(result.error() == FoliageError::NotFound);
    }
}

TEST_CASE("TerrainFoliageManager - Spatial Queries") {
    TerrainFoliageManager manager;

    FoliageId id1(1);
    MapCoords pos1{100.0f, 100.0f, 0.0f};

    FoliageId id2(2);
    MapCoords pos2{150.0f, 150.0f, 0.0f}; // distance ~ 70.7

    FoliageId id3(3);
    MapCoords pos3{500.0f, 500.0f, 0.0f}; // distance ~ 565.6

    FoliageId id4(4);
    MapCoords pos4{1200.0f, 1200.0f, 0.0f}; // different grid cell

    manager.AddFoliage(id1, pos1);
    manager.AddFoliage(id2, pos2);
    manager.AddFoliage(id3, pos3);
    manager.AddFoliage(id4, pos4);

    SUBCASE("Query with small radius") {
        auto result = manager.GetFoliageInRadius(pos1, 10.0f);
        REQUIRE(result.size() == 1);
        CHECK(result[0] == id1);
    }

    SUBCASE("Query with medium radius") {
        auto result = manager.GetFoliageInRadius(pos1, 100.0f);
        REQUIRE(result.size() == 2);
        
        bool foundId1 = false;
        bool foundId2 = false;
        for (auto id : result) {
            if (id == id1) foundId1 = true;
            if (id == id2) foundId2 = true;
        }
        CHECK(foundId1);
        CHECK(foundId2);
    }

    SUBCASE("Query crossing grid cell boundaries") {
        // Query near boundary of 1000x1000 grid cell
        MapCoords queryCenter{900.0f, 900.0f, 0.0f};
        auto result = manager.GetFoliageInRadius(queryCenter, 500.0f);
        
        bool foundId3 = false; // dist from (900,900) to (500,500) = ~565, should be out
        bool foundId4 = false; // dist from (900,900) to (1200,1200) = ~424, should be in

        for (auto id : result) {
            if (id == id3) foundId3 = true;
            if (id == id4) foundId4 = true;
        }
        CHECK(!foundId3);
        CHECK(foundId4);
    }
}
