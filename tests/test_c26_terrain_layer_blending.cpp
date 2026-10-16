#define DOCTEST_CONFIG_IMPLEMENT_WITH_MAIN
#include "doctest.h"
#include "../src/Client/Terrain/TerrainLayerBlendingCalculator.h"

using namespace Client::Terrain;

TEST_CASE("TerrainLayerBlendingCalculator - Valid operations") {
    TerrainLayerBlendingCalculator calc;

    SUBCASE("Podstawowe blendowanie - suma do 255") {
        std::array<uint8_t, 4> current = {0, 0, 0, 0};
        auto result = calc.CalculateNewWeights(current, 0, 255);
        REQUIRE(result.has_value());
        auto w = result.value();
        CHECK(w[0] == 255);
        CHECK(w[1] == 0);
        CHECK(w[2] == 0);
        CHECK(w[3] == 0);
    }

    SUBCASE("Proporcjonalne zmniejszanie") {
        std::array<uint8_t, 4> current = {100, 100, 55, 0};
        auto result = calc.CalculateNewWeights(current, 3, 100);
        REQUIRE(result.has_value());
        auto w = result.value();
        CHECK(w[3] == 100);
        int sum = w[0] + w[1] + w[2] + w[3];
        CHECK(sum == 255);
    }
}

TEST_CASE("TerrainLayerBlendingCalculator - Invalid layer index") {
    TerrainLayerBlendingCalculator calc;
    std::array<uint8_t, 4> current = {0, 0, 0, 0};
    auto result = calc.CalculateNewWeights(current, 4, 100);
    REQUIRE(!result.has_value());
    CHECK(result.error() == BlendingError::InvalidLayerIndex);
}
