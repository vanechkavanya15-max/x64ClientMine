#define DOCTEST_CONFIG_IMPLEMENT_WITH_MAIN
#include "doctest.h"

#include "../src/Client/Terrain/TerrainHeightSampler.h"
#include <vector>
#include <cmath>

using namespace Client::Terrain;

TEST_CASE("TerrainHeightSampler - Initialization") {
    TerrainHeightSampler sampler;

    SUBCASE("Fails when data not loaded") {
        auto heightResult = sampler.GetHeight(0.0f, 0.0f);
        CHECK_FALSE(heightResult.has_value());
        CHECK(heightResult.error() == TerrainError::DataNotLoaded);

        auto normalResult = sampler.GetNormal(0.0f, 0.0f);
        CHECK_FALSE(normalResult.has_value());
        CHECK(normalResult.error() == TerrainError::DataNotLoaded);
    }

    SUBCASE("Fails with invalid grid size") {
        std::vector<float> data = { 1.0f, 2.0f };
        auto result = sampler.Initialize(2, 2, data);
        CHECK_FALSE(result.has_value());
        CHECK(result.error() == TerrainError::InvalidGridSize);
    }

    SUBCASE("Fails with zero dimensions") {
        std::vector<float> data = {};
        auto result = sampler.Initialize(0, 0, data);
        CHECK_FALSE(result.has_value());
        CHECK(result.error() == TerrainError::InvalidGridSize);
    }

    SUBCASE("Succeeds with valid data") {
        std::vector<float> data(4, 1.0f);
        auto result = sampler.Initialize(2, 2, data);
        CHECK(result.has_value());
    }
}

TEST_CASE("TerrainHeightSampler - Bounds Checking") {
    TerrainHeightSampler sampler;
    std::vector<float> data = {
        1.0f, 1.0f, 1.0f,
        1.0f, 1.0f, 1.0f,
        1.0f, 1.0f, 1.0f
    };
    
    REQUIRE(sampler.Initialize(3, 3, data).has_value());

    SUBCASE("In bounds") {
        CHECK(sampler.GetHeight(0.0f, 0.0f).has_value());
        CHECK(sampler.GetHeight(2.0f, 2.0f).has_value());
        CHECK(sampler.GetHeight(1.5f, 1.5f).has_value());
    }

    SUBCASE("Out of bounds - Negative") {
        CHECK(sampler.GetHeight(-0.1f, 0.0f).error() == TerrainError::OutOfBounds);
        CHECK(sampler.GetHeight(0.0f, -0.1f).error() == TerrainError::OutOfBounds);
    }

    SUBCASE("Out of bounds - Exceeding positive") {
        CHECK(sampler.GetHeight(2.1f, 0.0f).error() == TerrainError::OutOfBounds);
        CHECK(sampler.GetHeight(0.0f, 2.1f).error() == TerrainError::OutOfBounds);
        CHECK(sampler.GetHeight(3.0f, 3.0f).error() == TerrainError::OutOfBounds);
    }
}

TEST_CASE("TerrainHeightSampler - Height Interpolation") {
    TerrainHeightSampler sampler;
    // 2x2 grid
    // (0,0): 10.0   (1,0): 20.0
    // (0,1): 30.0   (1,1): 40.0
    std::vector<float> data = {
        10.0f, 20.0f,
        30.0f, 40.0f
    };
    REQUIRE(sampler.Initialize(2, 2, data).has_value());

    SUBCASE("Exact corners") {
        CHECK(sampler.GetHeight(0.0f, 0.0f).value() == doctest::Approx(10.0f));
        CHECK(sampler.GetHeight(1.0f, 0.0f).value() == doctest::Approx(20.0f));
        CHECK(sampler.GetHeight(0.0f, 1.0f).value() == doctest::Approx(30.0f));
        CHECK(sampler.GetHeight(1.0f, 1.0f).value() == doctest::Approx(40.0f));
    }

    SUBCASE("Midpoints on edges") {
        CHECK(sampler.GetHeight(0.5f, 0.0f).value() == doctest::Approx(15.0f)); // Top edge
        CHECK(sampler.GetHeight(0.5f, 1.0f).value() == doctest::Approx(35.0f)); // Bottom edge
        CHECK(sampler.GetHeight(0.0f, 0.5f).value() == doctest::Approx(20.0f)); // Left edge
        CHECK(sampler.GetHeight(1.0f, 0.5f).value() == doctest::Approx(30.0f)); // Right edge
    }

    SUBCASE("Center point") {
        // (15.0 + 35.0) / 2 = 25.0
        CHECK(sampler.GetHeight(0.5f, 0.5f).value() == doctest::Approx(25.0f));
    }
}

TEST_CASE("TerrainHeightSampler - Normal Calculation") {
    TerrainHeightSampler sampler;

    SUBCASE("Flat plane") {
        std::vector<float> data = {
            10.0f, 10.0f,
            10.0f, 10.0f
        };
        REQUIRE(sampler.Initialize(2, 2, data).has_value());

        auto normal = sampler.GetNormal(0.5f, 0.5f).value();
        CHECK(normal.x == doctest::Approx(0.0f));
        CHECK(normal.y == doctest::Approx(0.0f));
        CHECK(normal.z == doctest::Approx(1.0f));
    }

    SUBCASE("Slanted plane X axis") {
        // Slanted up towards positive X
        // height increases by 10 per unit X
        std::vector<float> data = {
            0.0f, 10.0f,
            0.0f, 10.0f
        };
        REQUIRE(sampler.Initialize(2, 2, data).has_value());

        auto normal = sampler.GetNormal(0.5f, 0.5f).value();
        // dfdx = 10, dfdy = 0
        // vec: -10, 0, 1 -> normalized
        float len = std::sqrt(10.0f * 10.0f + 1.0f);
        CHECK(normal.x == doctest::Approx(-10.0f / len));
        CHECK(normal.y == doctest::Approx(0.0f));
        CHECK(normal.z == doctest::Approx(1.0f / len));
    }

    SUBCASE("Slanted plane Y axis") {
        // Slanted up towards positive Y
        // height increases by 5 per unit Y
        std::vector<float> data = {
            0.0f, 0.0f,
            5.0f, 5.0f
        };
        REQUIRE(sampler.Initialize(2, 2, data).has_value());

        auto normal = sampler.GetNormal(0.5f, 0.5f).value();
        // dfdx = 0, dfdy = 5
        // vec: 0, -5, 1 -> normalized
        float len = std::sqrt(5.0f * 5.0f + 1.0f);
        CHECK(normal.x == doctest::Approx(0.0f));
        CHECK(normal.y == doctest::Approx(-5.0f / len));
        CHECK(normal.z == doctest::Approx(1.0f / len));
    }
}
