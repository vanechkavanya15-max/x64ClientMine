#pragma once

#include <cstdint>
#include <span>
#include <vector>

namespace EterLib::Render {

class TerrainSplatWeightMap {
public:
    TerrainSplatWeightMap() = default;
    ~TerrainSplatWeightMap() = default;

    TerrainSplatWeightMap(const TerrainSplatWeightMap&) = default;
    TerrainSplatWeightMap& operator=(const TerrainSplatWeightMap&) = default;
    TerrainSplatWeightMap(TerrainSplatWeightMap&&) noexcept = default;
    TerrainSplatWeightMap& operator=(TerrainSplatWeightMap&&) noexcept = default;

    void Initialize(uint32_t patchX, uint32_t patchY);
    void SetLayerWeight(uint32_t layerIdx, uint8_t weight);
    void SetLayerWeight(uint32_t x, uint32_t y, uint32_t layerIdx, uint8_t weight);
    uint32_t GetPackedWeight(uint32_t x, uint32_t y) const noexcept;
    std::span<const uint32_t> GetRawData() const noexcept;

    // Connects 4-channel weight generation with tile.raw data
    bool GenerateFromTileMap(const uint8_t* pTileMap, uint32_t rawWidth, uint32_t rawHeight, uint32_t outWidth = 256, uint32_t outHeight = 256);

    uint32_t GetWidth() const noexcept { return m_width; }
    uint32_t GetHeight() const noexcept { return m_height; }

private:
    uint32_t m_width{0};
    uint32_t m_height{0};
    std::vector<uint32_t> m_weights;
};

} // namespace EterLib::Render

