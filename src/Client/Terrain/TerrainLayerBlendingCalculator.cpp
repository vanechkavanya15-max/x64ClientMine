#include "TerrainLayerBlendingCalculator.h"

namespace Client::Terrain {

std::expected<std::array<uint8_t, 4>, BlendingError> TerrainLayerBlendingCalculator::CalculateNewWeights(
    const std::array<uint8_t, 4>& currentWeights,
    uint8_t layerIndex,
    uint8_t newWeight) const 
{
    if (layerIndex >= 4) {
        return std::unexpected(BlendingError::InvalidLayerIndex);
    }

    std::array<uint8_t, 4> w = currentWeights;
    
    uint32_t sumOther = 0;
    for (uint32_t i = 0; i < 4; ++i) {
        if (i != layerIndex) {
            sumOther += w[i];
        }
    }

    std::array<uint8_t, 4> oldW = w;

    w[layerIndex] = newWeight;
    int32_t remaining = 255 - newWeight;

    if (sumOther > 0) {
        uint32_t newSumOther = 0;
        for (uint32_t i = 0; i < 4; ++i) {
            if (i != layerIndex) {
                uint32_t scaled = (static_cast<uint32_t>(w[i]) * remaining) / sumOther;
                w[i] = static_cast<uint8_t>(scaled);
                newSumOther += w[i];
            }
        }
        
        int32_t diff = remaining - newSumOther;
        for (uint32_t i = 0; i < 4 && diff > 0; ++i) {
            if (i != layerIndex && oldW[i] > 0) {
                w[i]++;
                diff--;
            }
        }
        for (uint32_t i = 0; i < 4 && diff > 0; ++i) {
            if (i != layerIndex) {
                w[i]++;
                diff--;
            }
        }
    } else {
        for (uint32_t i = 0; i < 4; ++i) {
            if (i != layerIndex) {
                w[i] = static_cast<uint8_t>(remaining);
                break;
            }
        }
    }

    return w;
}

} // namespace Client::Terrain
