#define DOCTEST_CONFIG_IMPLEMENT_WITH_MAIN
#include "../vendor/doctest/doctest.h"
#include "../src/Client/Terrain/TerrainHeightCollisionQuerier.h"
#include <vector>

using namespace Client::Terrain;
using namespace Client::Core;

TEST_CASE("TerrainHeightCollisionQuerier: Patch Loading and coordinate translation") {
    TerrainHeightCollisionQuerier querier;

    SUBCASE("World to Patch Coords") {
        auto [px, py] = TerrainHeightCollisionQuerier::WorldToPatchCoords(15000.0f, 25000.0f);
        CHECK(px == 0);
        CHECK(py == 0); // 128 * 200 = 25600

        auto [px2, py2] = TerrainHeightCollisionQuerier::WorldToPatchCoords(26000.0f, -5000.0f);
        CHECK(px2 == 1);
        CHECK(py2 == -1);
    }
}

TEST_CASE("TerrainHeightCollisionQuerier: Height and water level checks") {
    TerrainHeightCollisionQuerier querier;
    
    // Create dummy patch data
    const size_t patchSize = TerrainConfig::PATCH_SIZE_X * TerrainConfig::PATCH_SIZE_Y;
    std::vector<float> heights(patchSize, 50.0f);
    std::vector<float> waters(patchSize, 30.0f);
    
    // Set a slope
    for(int y=0; y<TerrainConfig::PATCH_SIZE_Y; ++y) {
        for(int x=0; x<TerrainConfig::PATCH_SIZE_X; ++x) {
            heights[y * TerrainConfig::PATCH_SIZE_X + x] = 50.0f + (x * 10.0f); 
            // 10 units of height per grid unit (200 world units)
            
            if (x < 10) {
                waters[y * TerrainConfig::PATCH_SIZE_X + x] = 100.0f; // High water early on
            }
        }
    }
    
    querier.LoadPatch(0, 0, heights, waters);

    SUBCASE("Valid Height Query") {
        auto res = querier.GetHeight(0.0f, 0.0f);
        REQUIRE(res.has_value());
        CHECK(res.value() == doctest::Approx(50.0f));

        // Halfway across a grid cell (100 units)
        auto resHalf = querier.GetHeight(100.0f, 0.0f);
        REQUIRE(resHalf.has_value());
        CHECK(resHalf.value() == doctest::Approx(55.0f)); // Interpolated
    }

    SUBCASE("Invalid Height Query (Out of bounds/Unloaded)") {
        auto res = querier.GetHeight(30000.0f, 0.0f); // Patch 1, 0
        REQUIRE_FALSE(res.has_value());
        CHECK(res.error() == TerrainQueryError::PatchNotLoaded);
        
        auto resNan = querier.GetHeight(NAN, 0.0f);
        REQUIRE_FALSE(resNan.has_value());
        CHECK(resNan.error() == TerrainQueryError::InvalidCoordinates);
    }

    SUBCASE("Water and IsUnderWater") {
        auto wRes = querier.GetWaterHeight(0.0f, 0.0f);
        REQUIRE(wRes.has_value());
        CHECK(wRes.value() == 100.0f);

        auto uRes = querier.IsUnderWater(0.0f, 0.0f);
        REQUIRE(uRes.has_value());
        CHECK(uRes.value() == true); // 50 < 100

        auto uResFar = querier.IsUnderWater(4000.0f, 0.0f); // x = 20 grid units
        REQUIRE(uResFar.has_value());
        CHECK(uResFar.value() == false); // 50 + 200 > 30
    }
}

TEST_CASE("TerrainHeightCollisionQuerier: Normal calculation") {
    TerrainHeightCollisionQuerier querier;
    
    const size_t patchSize = TerrainConfig::PATCH_SIZE_X * TerrainConfig::PATCH_SIZE_Y;
    std::vector<float> heights(patchSize, 0.0f);
    std::vector<float> waters(patchSize, 0.0f);
    
    // Slanted plane: z = x (in grid units)
    for(int y=0; y<TerrainConfig::PATCH_SIZE_Y; ++y) {
        for(int x=0; x<TerrainConfig::PATCH_SIZE_X; ++x) {
            heights[y * TerrainConfig::PATCH_SIZE_X + x] = x * 200.0f; // 1 unit height per world unit X
        }
    }
    
    querier.LoadPatch(0, 0, heights, waters);

    auto res = querier.GetNormal(1000.0f, 1000.0f);
    REQUIRE(res.has_value());
    
    MapCoords normal = res.value();
    
    // slope is 1, so vector is (-1, 0, 1), normalized
    float expectedLen = std::sqrt(1.0f + 1.0f);
    CHECK(normal.x == doctest::Approx(-1.0f / expectedLen));
    CHECK(normal.y == doctest::Approx(0.0f));
    CHECK(normal.z == doctest::Approx(1.0f / expectedLen));
}

TEST_CASE("TerrainHeightCollisionQuerier: Raycast") {
    TerrainHeightCollisionQuerier querier;
    
    const size_t patchSize = TerrainConfig::PATCH_SIZE_X * TerrainConfig::PATCH_SIZE_Y;
    std::vector<float> heights(patchSize, 50.0f); // Flat terrain at z=50
    std::vector<float> waters(patchSize, 0.0f);
    querier.LoadPatch(0, 0, heights, waters);

    SUBCASE("Hit from above") {
        MapCoords origin{1000.0f, 1000.0f, 200.0f};
        MapCoords dir{0.0f, 0.0f, -1.0f};
        
        auto res = querier.Raycast(origin, dir);
        REQUIRE(res.has_value());
        CHECK(res.value().hitPoint.x == doctest::Approx(1000.0f));
        CHECK(res.value().hitPoint.y == doctest::Approx(1000.0f));
        CHECK(res.value().hitPoint.z == doctest::Approx(50.0f));
        CHECK(res.value().distance == doctest::Approx(150.0f)); // 200 -> 50
    }
    
    SUBCASE("Start below terrain") {
        MapCoords origin{1000.0f, 1000.0f, 0.0f}; // below z=50
        MapCoords dir{0.0f, 0.0f, 1.0f};
        
        auto res = querier.Raycast(origin, dir);
        REQUIRE(res.has_value());
        CHECK(res.value().distance == doctest::Approx(0.0f)); // Instant hit
        CHECK(res.value().hitPoint.z == doctest::Approx(50.0f));
    }
    
    SUBCASE("Miss (shoot parallel)") {
        MapCoords origin{1000.0f, 1000.0f, 200.0f};
        MapCoords dir{1.0f, 0.0f, 0.0f};
        
        auto res = querier.Raycast(origin, dir, 5000.0f);
        REQUIRE_FALSE(res.has_value());
    }
}
