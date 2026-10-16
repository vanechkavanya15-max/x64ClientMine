#pragma once

#include "TerrainHeightSampler.h"
#include <expected>
#include <span>
#include <vector>
#include <cstdint>

namespace Client::Terrain {

    class TerrainSectorShadowBaker {
    public:
        TerrainSectorShadowBaker() = default;
        ~TerrainSectorShadowBaker() = default;

        // Bakes a shadow mask for a given terrain sector.
        // The sector extends from (startX, startY) with the given sectorWidth and sectorHeight.
        // It produces a mask of resolution (maskWidth x maskHeight), where each pixel corresponds to a terrain area.
        // The returned mask is a flat array of uint8_t (0 = completely shadowed, 255 = fully illuminated).
        [[nodiscard]] std::expected<std::vector<uint8_t>, TerrainError> BakeShadowMask(
            const TerrainHeightSampler& sampler,
            const Vector3& lightDir,
            float startX, float startY,
            float sectorWidth, float sectorHeight,
            uint32_t maskWidth, uint32_t maskHeight) const noexcept;
    };

} // namespace Client::Terrain
