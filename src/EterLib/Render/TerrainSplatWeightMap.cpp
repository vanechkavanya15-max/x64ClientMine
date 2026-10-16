#ifndef TEST_MODE_DISABLE_STDAFX
#include "../StdAfx.h"
#endif

#include "TerrainSplatWeightMap.h"
#include <algorithm>
#include <cstring>

namespace EterLib::Render {

TerrainSplatWeightMap::~TerrainSplatWeightMap() {
    ReleaseGPU();
}

TerrainSplatWeightMap::TerrainSplatWeightMap(TerrainSplatWeightMap&& other) noexcept
    : m_width(other.m_width)
    , m_height(other.m_height)
    , m_weights(std::move(other.m_weights))
    , m_dominantLayers(other.m_dominantLayers)
    , m_gpuTexture(other.m_gpuTexture)
    , m_isDirty(other.m_isDirty) {
    other.m_gpuTexture = nullptr;
    other.m_width = 0;
    other.m_height = 0;
}

TerrainSplatWeightMap& TerrainSplatWeightMap::operator=(TerrainSplatWeightMap&& other) noexcept {
    if (this != &other) {
        ReleaseGPU();
        m_width = other.m_width;
        m_height = other.m_height;
        m_weights = std::move(other.m_weights);
        m_dominantLayers = other.m_dominantLayers;
        m_gpuTexture = other.m_gpuTexture;
        m_isDirty = other.m_isDirty;
        other.m_gpuTexture = nullptr;
        other.m_width = 0;
        other.m_height = 0;
    }
    return *this;
}

void TerrainSplatWeightMap::ReleaseGPU() noexcept {
    if (m_gpuTexture) {
        m_gpuTexture->Release();
        m_gpuTexture = nullptr;
    }
}

void TerrainSplatWeightMap::Initialize(uint32_t width, uint32_t height) {
    if (m_width != width || m_height != height) {
        ReleaseGPU();
        m_width = width;
        m_height = height;
    }
    // Domyslnie pelna waga (255) na warstwie 0 (kanal R)
    m_weights.assign(static_cast<size_t>(width) * height, PackD3DColor(255, 0, 0, 0));
    m_dominantLayers = {1, 0, 0, 0};
    m_isDirty = true;
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

    uint32_t& packed = m_weights[static_cast<size_t>(y) * m_width + x];
    uint8_t w[4];
    UnpackD3DColor(packed, w);

    uint8_t oldW[4] = {w[0], w[1], w[2], w[3]};
    uint32_t sumOther = 0;
    for (uint32_t i = 0; i < 4; ++i) {
        if (i != layerIdx) sumOther += w[i];
    }

    w[layerIdx] = weight;
    int32_t remaining = 255 - static_cast<int32_t>(weight);

    if (sumOther > 0) {
        uint32_t newSumOther = 0;
        for (uint32_t i = 0; i < 4; ++i) {
            if (i != layerIdx) {
                w[i] = static_cast<uint8_t>((static_cast<uint32_t>(w[i]) * remaining) / sumOther);
                newSumOther += w[i];
            }
        }
        int32_t diff = remaining - static_cast<int32_t>(newSumOther);
        for (uint32_t i = 0; i < 4 && diff > 0; ++i) {
            if (i != layerIdx && oldW[i] > 0) {
                w[i]++;
                diff--;
            }
        }
        for (uint32_t i = 0; i < 4 && diff > 0; ++i) {
            if (i != layerIdx) {
                w[i]++;
                diff--;
            }
        }
    } else {
        for (uint32_t i = 0; i < 4; ++i) {
            if (i != layerIdx) {
                w[i] = static_cast<uint8_t>(remaining);
                break;
            }
        }
    }

    packed = PackD3DColor(w[0], w[1], w[2], w[3]);
    m_isDirty = true;
}

uint32_t TerrainSplatWeightMap::GetPackedWeight(uint32_t x, uint32_t y) const noexcept {
    if (x >= m_width || y >= m_height) return 0;
    return m_weights[static_cast<size_t>(y) * m_width + x];
}

std::span<const uint32_t> TerrainSplatWeightMap::GetRawData() const noexcept {
    return {m_weights.data(), m_weights.size()};
}

bool TerrainSplatWeightMap::GenerateFromTileMap(
    const uint8_t* pTileMap, uint32_t rawWidth, uint32_t rawHeight, uint32_t outWidth, uint32_t outHeight) {
    if (!pTileMap || rawWidth == 0 || rawHeight == 0 || outWidth == 0 || outHeight == 0) {
        return false;
    }

    Initialize(outWidth, outHeight);

    const bool hasBorder = (rawWidth >= outWidth + 2 && rawHeight >= outHeight + 2);
    const uint32_t borderOffset = hasBorder ? 1 : 0;

    // Krok 1: Histogram czestosci wystepowania tekstur w tile.raw (bez alokacji sterty)
    std::array<uint32_t, 256> tileHistogram{};
    for (uint32_t y = 0; y < outHeight; ++y) {
        const uint32_t sy = std::min(y + borderOffset, rawHeight - 1);
        for (uint32_t x = 0; x < outWidth; ++x) {
            const uint32_t sx = std::min(x + borderOffset, rawWidth - 1);
            const uint8_t tileId = pTileMap[sy * rawWidth + sx];
            if (tileId > 0) { // 0 to brak tekstury w konwencji Metin2
                tileHistogram[tileId]++;
            }
        }
    }

    // Wybor 4 dominujacych warstw tekstur
    m_dominantLayers = {0, 0, 0, 0};
    for (size_t slot = 0; slot < 4; ++slot) {
        uint32_t bestTile = 0;
        uint32_t bestCount = 0;
        for (uint32_t t = 1; t < 256; ++t) {
            if (tileHistogram[t] > bestCount) {
                bestCount = tileHistogram[t];
                bestTile = t;
            }
        }
        if (bestCount == 0) break;
        m_dominantLayers[slot] = bestTile;
        tileHistogram[bestTile] = 0;
    }

    // Tablica szybkiego mapowania (LUT): globalny ID tekstury -> kanal [0..3]
    std::array<uint8_t, 256> tileToChannel{};
    tileToChannel.fill(0); // Fallback do warstwy bazowej 0
    for (uint8_t ch = 0; ch < 4; ++ch) {
        if (m_dominantLayers[ch] != 0) {
            tileToChannel[m_dominantLayers[ch]] = ch;
        }
    }

    // Krok 2: Filtr 3x3 i generowanie wag RGBA (w ukladzie D3DFMT_A8R8G8B8)
    for (uint32_t y = 0; y < outHeight; ++y) {
        const int sy = static_cast<int>(y + borderOffset);
        for (uint32_t x = 0; x < outWidth; ++x) {
            const int sx = static_cast<int>(x + borderOffset);

            uint32_t layerWeights[4] = {0, 0, 0, 0};
            const uint8_t centerTile = pTileMap[std::clamp(sy, 0, static_cast<int>(rawHeight) - 1) * rawWidth +
                                                std::clamp(sx, 0, static_cast<int>(rawWidth) - 1)];
            const uint8_t dominantChannel = tileToChannel[centerTile];

            for (int dy = -1; dy <= 1; ++dy) {
                const int py = std::clamp(sy + dy, 0, static_cast<int>(rawHeight) - 1);
                for (int dx = -1; dx <= 1; ++dx) {
                    const int px = std::clamp(sx + dx, 0, static_cast<int>(rawWidth) - 1);
                    const uint8_t ch = tileToChannel[pTileMap[py * rawWidth + px]];
                    const uint32_t kernelW = (dx == 0 && dy == 0) ? 8u : ((dx == 0 || dy == 0) ? 2u : 1u);
                    layerWeights[ch] += kernelW;
                }
            }

            constexpr uint32_t totalKernelWeight = 20;
            uint8_t w[4] = {0, 0, 0, 0};
            uint32_t sum = 0;
            for (int i = 0; i < 4; ++i) {
                w[i] = static_cast<uint8_t>((layerWeights[i] * 255u) / totalKernelWeight);
                sum += w[i];
            }

            const int32_t remainder = 255 - static_cast<int32_t>(sum);
            if (remainder > 0) {
                w[dominantChannel] = static_cast<uint8_t>(w[dominantChannel] + remainder);
            }

            m_weights[static_cast<size_t>(y) * outWidth + x] = PackD3DColor(w[0], w[1], w[2], w[3]);
        }
    }

    m_isDirty = true;
    return true;
}

bool TerrainSplatWeightMap::UploadToGPU(LPDIRECT3DDEVICE9 dev) {
    if (!dev || m_width == 0 || m_height == 0 || m_weights.empty()) {
        return false;
    }

    if (!m_gpuTexture) {
        if (FAILED(dev->CreateTexture(m_width, m_height, 1, 0, D3DFMT_A8R8G8B8, D3DPOOL_MANAGED, &m_gpuTexture, nullptr))) {
            return false;
        }
    }

    D3DLOCKED_RECT lockedRect{};
    if (FAILED(m_gpuTexture->LockRect(0, &lockedRect, nullptr, 0))) {
        return false;
    }

    auto* pDstRows = static_cast<uint8_t*>(lockedRect.pBits);
    const uint32_t rowBytes = m_width * sizeof(uint32_t);
    for (uint32_t y = 0; y < m_height; ++y) {
        std::memcpy(pDstRows + y * lockedRect.Pitch, &m_weights[static_cast<size_t>(y) * m_width], rowBytes);
    }

    m_gpuTexture->UnlockRect(0);
    m_isDirty = false;
    return true;
}

} // namespace EterLib::Render
