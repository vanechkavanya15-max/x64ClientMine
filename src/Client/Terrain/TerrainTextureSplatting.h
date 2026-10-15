#pragma once

#include <cstdint>
#include <string>
#include <vector>
#include <expected>
#include <optional>
#include <array>
#include <string_view>

namespace Client::Terrain {

enum class SplatError {
    InvalidLayerIndex,
    LayerAlreadyExists,
    LayerNotSet,
    MaxLayersReached
};

struct SplatLayerInfo {
    uint8_t layerIndex;
    std::string texturePath;
    float scaleU;
    float scaleV;
};

class TerrainTextureSplatting {
public:
    static constexpr uint8_t MAX_LAYERS = 4;

    TerrainTextureSplatting() = default;
    ~TerrainTextureSplatting() = default;

    TerrainTextureSplatting(const TerrainTextureSplatting&) = default;
    TerrainTextureSplatting& operator=(const TerrainTextureSplatting&) = default;
    TerrainTextureSplatting(TerrainTextureSplatting&&) noexcept = default;
    TerrainTextureSplatting& operator=(TerrainTextureSplatting&&) noexcept = default;

    std::expected<void, SplatError> AddLayer(uint8_t layerIndex, std::string_view texturePath, float scaleU = 10.0f, float scaleV = 10.0f);
    std::expected<void, SplatError> RemoveLayer(uint8_t layerIndex);
    
    std::expected<SplatLayerInfo, SplatError> GetLayer(uint8_t layerIndex) const;
    
    bool HasLayer(uint8_t layerIndex) const noexcept;
    uint8_t GetActiveLayerCount() const noexcept;
    
    void Clear() noexcept;

    std::vector<SplatLayerInfo> GetAllActiveLayers() const;

private:
    std::array<std::optional<SplatLayerInfo>, MAX_LAYERS> m_layers;
};

} // namespace Client::Terrain
