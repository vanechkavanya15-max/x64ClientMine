#pragma once

#include <cstdint>
#include <vector>
#include <span>
#include <expected>
#include "Core/StrongTypes.h"

namespace Client::Terrain {

    enum class NormalMapError : uint8_t {
        InvalidGridSize,
        InvalidSpanSize,
        AllocationFailed
    };

    class TerrainNormalMapGenerator {
    public:
        TerrainNormalMapGenerator() = default;
        ~TerrainNormalMapGenerator() = default;

        [[nodiscard]] std::expected<std::vector<Client::Core::MapCoords>, NormalMapError> Generate(
            uint32_t width, 
            uint32_t height, 
            std::span<const float> heights) const noexcept;
    };

} // namespace Client::Terrain
