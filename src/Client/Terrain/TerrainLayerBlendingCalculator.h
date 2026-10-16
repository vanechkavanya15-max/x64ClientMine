#pragma once

#include <cstdint>
#include <array>
#include <expected>

namespace Client::Terrain {

    enum class BlendingError : uint8_t {
        InvalidLayerIndex
    };

    class TerrainLayerBlendingCalculator {
    public:
        TerrainLayerBlendingCalculator() = default;
        ~TerrainLayerBlendingCalculator() = default;

        std::expected<std::array<uint8_t, 4>, BlendingError> CalculateNewWeights(
            const std::array<uint8_t, 4>& currentWeights,
            uint8_t layerIndex,
            uint8_t newWeight) const;
    };

} // namespace Client::Terrain
