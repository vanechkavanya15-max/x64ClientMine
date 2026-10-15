#include "doctest.h"
#include <vector>
#include <cmath>
#include <iostream>

#include "Client/Terrain/TerrainHeightSampler.h"

using namespace Client::Terrain;

TEST_CASE("TerrainHeightSampler - Bilinear Interpolation") {
    TerrainHeightSampler sampler;

    // 2x2 grid representing 4 corners
    // h00 = 10.0f, h10 = 20.0f
    // h01 = 30.0f, h11 = 40.0f
    std::vector<float> grid = {
         10.0f, 20.0f,
         30.0f, 40.0f
    };
    
    // Scale = 100.0f (each grid cell is 100x100 world units)
    sampler.SetHeightData(grid, 2, 2, 100.0f);

    SUBCASE("Corners") {
        CHECK((std::abs(sampler.GetHeight(0.0f, 0.0f) - 10.0f) < 0.001f));
        CHECK((std::abs(sampler.GetHeight(100.0f, 0.0f) - 20.0f) < 0.001f));
        CHECK((std::abs(sampler.GetHeight(0.0f, 100.0f) - 30.0f) < 0.001f));
        CHECK((std::abs(sampler.GetHeight(100.0f, 100.0f) - 40.0f) < 0.001f));
    }

    SUBCASE("Midpoints") {
        // Midpoint of top edge (0,0 to 100,0)
        CHECK((std::abs(sampler.GetHeight(50.0f, 0.0f) - 15.0f) < 0.001f));
        
        // Midpoint of bottom edge (0,100 to 100,100)
        CHECK((std::abs(sampler.GetHeight(50.0f, 100.0f) - 35.0f) < 0.001f));
        
        // Midpoint of left edge (0,0 to 0,100)
        CHECK((std::abs(sampler.GetHeight(0.0f, 50.0f) - 20.0f) < 0.001f));
        
        // Midpoint of right edge (100,0 to 100,100)
        CHECK((std::abs(sampler.GetHeight(100.0f, 50.0f) - 30.0f) < 0.001f));
    }

    SUBCASE("Center of square") {
        // (50, 50) -> should be average of all 4 corners = (10+20+30+40)/4 = 25.0f
        CHECK((std::abs(sampler.GetHeight(50.0f, 50.0f) - 25.0f) < 0.001f));
    }

    SUBCASE("Out of bounds") {
        // Should clamp to nearest edge/corner
        CHECK((std::abs(sampler.GetHeight(-50.0f, -50.0f) - 10.0f) < 0.001f));
        CHECK((std::abs(sampler.GetHeight(150.0f, 50.0f) - 30.0f) < 0.001f)); // right edge midpoint (100, 50) -> clamped to (100, 50) -> Wait, clamp applies to grid coordinates. 150/100 -> 1.5 -> clamped grid is 1. x clamped to 1, y is 0.5. Height should be interpolating between (100, 0) and (100, 100), which are 20 and 40. So 50% between 20 and 40 is 30.
        CHECK((std::abs(sampler.GetHeight(150.0f, 150.0f) - 40.0f) < 0.001f)); // clamped to (100, 100)
    }
}

TEST_CASE("TerrainHeightSampler - Normal Calculation") {
    TerrainHeightSampler sampler;

    // 3x3 grid (flat except for a bump in the middle)
    std::vector<float> grid = {
        0.0f, 0.0f, 0.0f,
        0.0f, 10.0f, 0.0f,
        0.0f, 0.0f, 0.0f
    };
     
    // Scale = 1.0f
    sampler.SetHeightData(grid, 3, 3, 1.0f);
    
    SUBCASE("Normal at slope") {
        Vector3 n = sampler.GetNormal(0.5f, 1.0f);
        // Expect normal pointing away from the bump
        CHECK((n.z > 0.0f));
        CHECK((std::abs(n.x) > 0.0f));
    }
}
