#define DOCTEST_CONFIG_IMPLEMENT_WITH_MAIN
#include <doctest/doctest.h>

#include "Client/Terrain/TerrainSectorShadowBaker.h"
#include "Client/Terrain/TerrainHeightSampler.h"
#include <vector>

using namespace Client::Terrain;

TEST_CASE("TerrainSectorShadowBaker - Invalid Input")
{
    TerrainSectorShadowBaker baker;
    TerrainHeightSampler sampler;
    Vector3 lightDir = { 0.0f, 0.0f, -1.0f };

    SUBCASE("Zero mask size")
    {
        auto res = baker.BakeShadowMask(sampler, lightDir, 0.0f, 0.0f, 100.0f, 100.0f, 0, 10);
        REQUIRE(!res.has_value());
        CHECK(res.error() == TerrainError::InvalidGridSize);
    }

    SUBCASE("Data not loaded")
    {
        auto res = baker.BakeShadowMask(sampler, lightDir, 0.0f, 0.0f, 10.0f, 10.0f, 2, 2);
        REQUIRE(!res.has_value());
        CHECK(res.error() == TerrainError::DataNotLoaded);
    }
}

TEST_CASE("TerrainSectorShadowBaker - Valid Scenarios")
{
    TerrainSectorShadowBaker baker;
    TerrainHeightSampler sampler;
    
    // Create a 10x10 terrain grid
    std::vector<float> heights(100, 0.0f);
    
    // Create a tall block in the middle (x: 4-6, y: 4-6)
    for(int y = 4; y <= 6; ++y)
    {
        for(int x = 4; x <= 6; ++x)
        {
            heights[y * 10 + x] = 50.0f;
        }
    }
    
    auto initRes = sampler.Initialize(10, 10, heights);
    REQUIRE(initRes.has_value());

    SUBCASE("Light pointing straight down")
    {
        // Light from directly above
        Vector3 lightDir = { 0.0f, 0.0f, -1.0f };
        
        // Bake a 10x10 mask mapping exactly to the terrain
        auto res = baker.BakeShadowMask(sampler, lightDir, 0.0f, 0.0f, 10.0f, 10.0f, 10, 10);
        REQUIRE(res.has_value());
        
        const auto& mask = res.value();
        REQUIRE(mask.size() == 100);
        
        // With straight down light, there are no shadows on vertical surfaces 
        // since the ray goes straight up. The tall block itself might shadow its own bottom if we checked that,
        // but since we step straight up, currentZ increases linearly, sample height shouldn't exceed currentZ 
        // if we are already at the top, or if we start at the bottom of the flat ground.
        // Let's verify everything is illuminated since no parts of the terrain overhang.
        for (uint8_t val : mask)
        {
            CHECK(val == 255);
        }
    }

    SUBCASE("Light pointing at an angle")
    {
        // Light from +X direction, slightly downward
        // Raymarch will go towards +X and +Z
        Vector3 lightDir = { -1.0f, 0.0f, -0.5f };
        
        auto res = baker.BakeShadowMask(sampler, lightDir, 0.0f, 0.0f, 10.0f, 10.0f, 10, 10);
        REQUIRE(res.has_value());
        
        const auto& mask = res.value();
        REQUIRE(mask.size() == 100);

        // The tall block is at x: 4..6.
        // Raymarching from x > 6 (e.g. x=7,8,9) towards +X (opposite of lightDir) will hit the tall block!
        // Wait, if light is from +X, lightDir = (-1, 0, -0.5). Light travels towards -X.
        // Raymarch direction is opposite of light: (+1, 0, +0.5).
        // If we start at x=7 and march towards +X, we move AWAY from the block.
        // So x > 6 should NOT be shadowed by the block.
        // If we start at x=2,3 and march towards +X, we move TOWARDS the block.
        // So x=2,3 might be shadowed if the ray hits the 50.0f block before going over it.
        
        // Let's check x=2, y=5. (should be shadowed)
        CHECK(mask[5 * 10 + 2] == 0);
        // x=3, y=5. (should be shadowed)
        CHECK(mask[5 * 10 + 3] == 0);
        
        // Let's check x=8, y=5. (should be illuminated)
        CHECK(mask[5 * 10 + 8] == 255);
    }
}
