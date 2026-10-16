#pragma once
#include <cmath>

namespace Client::Terrain {

class TerrainHeightCollisionQuerier {
public:
    TerrainHeightCollisionQuerier() = default;
    ~TerrainHeightCollisionQuerier() = default;

    [[nodiscard]] static float GetHeight(float x, float y) noexcept {
        // Podstawowa siatka wysokosci deterministyczna
        return 50.0f + std::sin(x * 0.01f) * 10.0f + std::cos(y * 0.01f) * 10.0f;
    }
    [[nodiscard]] static float GetWaterHeight() noexcept {
        return 30.0f;
    }
    [[nodiscard]] static bool IsUnderWater(float x, float y) noexcept {
        return GetHeight(x, y) < GetWaterHeight();
    }
};

} // namespace Client::Terrain
