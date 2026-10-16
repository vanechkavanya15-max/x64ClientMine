#ifndef TEST_MODE_DISABLE_STDAFX
#include "../StdAfx.h"
#endif

#include "TerrainSplatWeightMap.h"
#include <algorithm>

namespace EterLib::Render {

void TerrainSplatWeightMap::Initialize(uint32_t patchX, uint32_t patchY) {
    m_width = patchX;
    m_height = patchY;
    m_weights.assign(patchX * patchY, 0x00000000); // 0,0,0,0 initially
}

void TerrainSplatWeightMap::SetLayerWeight(uint32_t layerIdx, uint8_t weight) {
    if (layerIdx > 3) return;
    for (uint32_t y = 0; y < m_height; ++y) {
        for (uint32_t x = 0; x < m_width; ++x) {
            SetLayerWeight(x, y, layerIdx, weight);
        }
    }
}

void TerrainSplatWeightMap::SetLayerWeight(uint32_t x, uint32_t y, uint32_t layerIdx, uint8_t weight) {
    if (layerIdx > 3 || x >= m_width || y >= m_height) return;

    uint32_t& packed = m_weights[y * m_width + x];
    
    uint8_t w[4];
    w[0] = static_cast<uint8_t>(packed & 0xFF);
    w[1] = static_cast<uint8_t>((packed >> 8) & 0xFF);
    w[2] = static_cast<uint8_t>((packed >> 16) & 0xFF);
    w[3] = static_cast<uint8_t>((packed >> 24) & 0xFF);

    uint32_t sumOther = 0;
    for (uint32_t i = 0; i < 4; ++i) {
        if (i != layerIdx) {
            sumOther += w[i];
        }
    }

    // Capture old weights for remainder distribution
    uint8_t oldW[4];
    for (int i = 0; i < 4; ++i) oldW[i] = w[i];

    w[layerIdx] = weight;
    int32_t remaining = 255 - weight;

    if (sumOther > 0) {
        uint32_t newSumOther = 0;
        for (uint32_t i = 0; i < 4; ++i) {
            if (i != layerIdx) {
                uint32_t scaled = (static_cast<uint32_t>(w[i]) * remaining) / sumOther;
                w[i] = static_cast<uint8_t>(scaled);
                newSumOther += w[i];
            }
        }
        
        // Distribute any remainder due to integer division based on original non-zero weights
        int32_t diff = remaining - newSumOther;
        for (uint32_t i = 0; i < 4 && diff > 0; ++i) {
            if (i != layerIdx && oldW[i] > 0) {
                w[i]++;
                diff--;
            }
        }
        // If there is still a difference, distribute it blindly (fallback)
        for (uint32_t i = 0; i < 4 && diff > 0; ++i) {
            if (i != layerIdx) {
                w[i]++;
                diff--;
            }
        }
    } else {
        // If sumOther is 0, we distribute the remaining into lowest index available.
        for (uint32_t i = 0; i < 4; ++i) {
            if (i != layerIdx) {
                w[i] = static_cast<uint8_t>(remaining);
                break;
            }
        }
    }

    packed = (static_cast<uint32_t>(w[0])) |
             (static_cast<uint32_t>(w[1]) << 8) |
             (static_cast<uint32_t>(w[2]) << 16) |
             (static_cast<uint32_t>(w[3]) << 24);
}

uint32_t TerrainSplatWeightMap::GetPackedWeight(uint32_t x, uint32_t y) const noexcept {
    if (x >= m_width || y >= m_height) return 0;
    return m_weights[y * m_width + x];
}

std::span<const uint32_t> TerrainSplatWeightMap::GetRawData() const noexcept {
    return {m_weights.data(), m_weights.size()};
}

bool TerrainSplatWeightMap::GenerateFromTileMap(
    const uint8_t* pTileMap, uint32_t rawWidth, uint32_t rawHeight, uint32_t outWidth, uint32_t outHeight)
{
    if (!pTileMap || rawWidth == 0 || rawHeight == 0 || outWidth == 0 || outHeight == 0) {
        return false;
    }

    Initialize(outWidth, outHeight);

    // Determine border offset if raw tilemap contains 1-pixel border on each side (e.g., 258x258 for 256x256)
    const bool hasBorder = (rawWidth >= outWidth + 2 && rawHeight >= outHeight + 2);
    const uint32_t borderOffset = hasBorder ? 1 : 0;

    for (uint32_t y = 0; y < outHeight; ++y) {
        for (uint32_t x = 0; x < outWidth; ++x) {
            uint32_t sx = x + borderOffset;
            uint32_t sy = y + borderOffset;

            // Gather layer counts in 3x3 neighborhood around (sx, sy) for smooth boundary blending
            uint32_t layerWeights[4] = {0, 0, 0, 0};
            uint8_t centerTile = pTileMap[sy * rawWidth + sx] % 4;
            uint32_t dominantLayer = centerTile;

            // 3x3 filter: center weight 8, orthogonal neighbors 2, diagonal neighbors 1 (total = 20)
            for (int dy = -1; dy <= 1; ++dy) {
                int py = static_cast<int>(sy) + dy;
                if (py < 0 || py >= static_cast<int>(rawHeight)) {
                    py = static_cast<int>(sy);
                }

                for (int dx = -1; dx <= 1; ++dx) {
                    int px = static_cast<int>(sx) + dx;
                    if (px < 0 || px >= static_cast<int>(rawWidth)) {
                        px = static_cast<int>(sx);
                    }

                    uint8_t tile = pTileMap[py * rawWidth + px] % 4;
                    uint32_t weight = 1;
                    if (dx == 0 && dy == 0) {
                        weight = 8;
                    } else if (dx == 0 || dy == 0) {
                        weight = 2;
                    }

                    layerWeights[tile] += weight;
                }
            }

            constexpr uint32_t totalKernelWeight = 20;

            uint8_t w[4] = {0, 0, 0, 0};
            uint32_t sum = 0;
            for (int i = 0; i < 4; ++i) {
                w[i] = static_cast<uint8_t>((layerWeights[i] * 255) / totalKernelWeight);
                sum += w[i];
            }

            int32_t remainder = 255 - static_cast<int32_t>(sum);
            if (remainder > 0) {
                w[dominantLayer] = static_cast<uint8_t>(w[dominantLayer] + remainder);
            }

            m_weights[y * outWidth + x] =
                (static_cast<uint32_t>(w[0])) |
                (static_cast<uint32_t>(w[1]) << 8) |
                (static_cast<uint32_t>(w[2]) << 16) |
                (static_cast<uint32_t>(w[3]) << 24);
        }
    }

    return true;
}

} // namespace EterLib::Render


