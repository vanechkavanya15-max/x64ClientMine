#include "TerrainSectorShadowBaker.h"
#include <cmath>
#include <algorithm>

namespace Client::Terrain {

std::expected<std::vector<uint8_t>, TerrainError> TerrainSectorShadowBaker::BakeShadowMask(
    const TerrainHeightSampler& sampler,
    const Vector3& lightDir,
    float startX, float startY,
    float sectorWidth, float sectorHeight,
    uint32_t maskWidth, uint32_t maskHeight) const noexcept
{
    if (maskWidth == 0 || maskHeight == 0)
    {
        return std::unexpected(TerrainError::InvalidGridSize);
    }

    // Ensure light direction is valid and points downwards (z < 0)
    if (lightDir.z >= -0.0001f)
    {
        // Light is parallel to ground or pointing up, everything is in shadow
        return std::vector<uint8_t>(static_cast<size_t>(maskWidth) * maskHeight, 0);
    }

    std::vector<uint8_t> mask;
    mask.resize(static_cast<size_t>(maskWidth) * maskHeight, 255);

    const float stepX = sectorWidth / static_cast<float>(maskWidth);
    const float stepY = sectorHeight / static_cast<float>(maskHeight);

    // Normalize light dir
    const float lightLengthSq = lightDir.x * lightDir.x + lightDir.y * lightDir.y + lightDir.z * lightDir.z;
    if (lightLengthSq < 0.000001f)
    {
        return mask; // No valid light direction, assume illuminated
    }
    const float invLength = 1.0f / std::sqrt(lightLengthSq);
    const Vector3 normLightDir = { lightDir.x * invLength, lightDir.y * invLength, lightDir.z * invLength };
    
    // Raymarch towards the light source, so opposite of light direction
    const Vector3 rayDir = { -normLightDir.x, -normLightDir.y, -normLightDir.z };

    // Max trace distance can be derived from terrain bounds, but we can set a reasonable step
    const float marchStepDistance = std::min(stepX, stepY) * 0.5f; 
    const Vector3 marchStep = {
        rayDir.x * marchStepDistance,
        rayDir.y * marchStepDistance,
        rayDir.z * marchStepDistance
    };
    
    // Safety exit
    const int maxSteps = 1000; 

    for (uint32_t my = 0; my < maskHeight; ++my)
    {
        for (uint32_t mx = 0; mx < maskWidth; ++mx)
        {
            const float posX = startX + (static_cast<float>(mx) + 0.5f) * stepX;
            const float posY = startY + (static_cast<float>(my) + 0.5f) * stepY;

            auto heightRes = sampler.GetHeight(posX, posY);
            if (!heightRes.has_value())
            {
                if (heightRes.error() == TerrainError::DataNotLoaded)
                    return std::unexpected(TerrainError::DataNotLoaded);
                
                // Out of bounds, skip
                continue;
            }

            float currentZ = heightRes.value();
            float currentX = posX;
            float currentY = posY;

            bool isShadowed = false;

            for (int s = 1; s <= maxSteps; ++s)
            {
                currentX += marchStep.x;
                currentY += marchStep.y;
                currentZ += marchStep.z;

                auto sampleRes = sampler.GetHeight(currentX, currentY);
                if (!sampleRes.has_value())
                {
                    // Escaped the terrain bounds, must be illuminated
                    break;
                }

                if (sampleRes.value() > currentZ)
                {
                    // Hit terrain
                    isShadowed = true;
                    break;
                }
            }

            if (isShadowed)
            {
                mask[my * maskWidth + mx] = 0;
            }
        }
    }

    return mask;
}

} // namespace Client::Terrain
