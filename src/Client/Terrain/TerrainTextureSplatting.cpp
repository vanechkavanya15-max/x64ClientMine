#include "TerrainTextureSplatting.h"

namespace Client::Terrain {

std::expected<void, SplatError> TerrainTextureSplatting::AddLayer(uint8_t layerIndex, std::string_view texturePath, float scaleU, float scaleV) {
    if (layerIndex >= MAX_LAYERS) {
        return std::unexpected(SplatError::InvalidLayerIndex);
    }
    
    if (m_layers[layerIndex].has_value()) {
        return std::unexpected(SplatError::LayerAlreadyExists);
    }
    
    m_layers[layerIndex] = SplatLayerInfo{layerIndex, std::string(texturePath), scaleU, scaleV};
    
    return {};
}

std::expected<void, SplatError> TerrainTextureSplatting::RemoveLayer(uint8_t layerIndex) {
    if (layerIndex >= MAX_LAYERS) {
        return std::unexpected(SplatError::InvalidLayerIndex);
    }
    
    if (!m_layers[layerIndex].has_value()) {
        return std::unexpected(SplatError::LayerNotSet);
    }
    
    m_layers[layerIndex].reset();
    
    return {};
}

std::expected<void, SplatError> TerrainTextureSplatting::UpdateLayerScale(uint8_t layerIndex, float newScaleU, float newScaleV) {
    if (layerIndex >= MAX_LAYERS) {
        return std::unexpected(SplatError::InvalidLayerIndex);
    }
    
    if (!m_layers[layerIndex].has_value()) {
        return std::unexpected(SplatError::LayerNotSet);
    }
    
    m_layers[layerIndex]->scaleU = newScaleU;
    m_layers[layerIndex]->scaleV = newScaleV;
    
    return {};
}

std::expected<void, SplatError> TerrainTextureSplatting::ReplaceLayerTexture(uint8_t layerIndex, std::string_view newTexturePath) {
    if (layerIndex >= MAX_LAYERS) {
        return std::unexpected(SplatError::InvalidLayerIndex);
    }
    
    if (!m_layers[layerIndex].has_value()) {
        return std::unexpected(SplatError::LayerNotSet);
    }
    
    m_layers[layerIndex]->texturePath = std::string(newTexturePath);
    
    return {};
}

std::expected<void, SplatError> TerrainTextureSplatting::SwapLayers(uint8_t layerIndexA, uint8_t layerIndexB) {
    if (layerIndexA >= MAX_LAYERS || layerIndexB >= MAX_LAYERS) {
        return std::unexpected(SplatError::InvalidLayerIndex);
    }
    
    if (!m_layers[layerIndexA].has_value() || !m_layers[layerIndexB].has_value()) {
        return std::unexpected(SplatError::LayerNotSet);
    }
    
    std::swap(m_layers[layerIndexA], m_layers[layerIndexB]);
    
    // Update indices after swap
    m_layers[layerIndexA]->layerIndex = layerIndexA;
    m_layers[layerIndexB]->layerIndex = layerIndexB;
    
    return {};
}

std::expected<SplatLayerInfo, SplatError> TerrainTextureSplatting::GetLayer(uint8_t layerIndex) const {
    if (layerIndex >= MAX_LAYERS) {
        return std::unexpected(SplatError::InvalidLayerIndex);
    }
    
    if (!m_layers[layerIndex].has_value()) {
        return std::unexpected(SplatError::LayerNotSet);
    }
    
    return *m_layers[layerIndex];
}

bool TerrainTextureSplatting::HasLayer(uint8_t layerIndex) const noexcept {
    if (layerIndex >= MAX_LAYERS) {
        return false;
    }
    return m_layers[layerIndex].has_value();
}

uint8_t TerrainTextureSplatting::GetActiveLayerCount() const noexcept {
    uint8_t count = 0;
    for (const auto& layer : m_layers) {
        if (layer.has_value()) {
            ++count;
        }
    }
    return count;
}

void TerrainTextureSplatting::Clear() noexcept {
    for (auto& layer : m_layers) {
        layer.reset();
    }
}

std::vector<SplatLayerInfo> TerrainTextureSplatting::GetAllActiveLayers() const {
    std::vector<SplatLayerInfo> activeLayers;
    activeLayers.reserve(GetActiveLayerCount());
    
    for (const auto& layer : m_layers) {
        if (layer.has_value()) {
            activeLayers.push_back(*layer);
        }
    }
    
    return activeLayers;
}

} // namespace Client::Terrain
