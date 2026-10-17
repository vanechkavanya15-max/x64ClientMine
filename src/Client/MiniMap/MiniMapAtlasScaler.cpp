#include "MiniMapAtlasScaler.h"
#include <cmath>

namespace Client::MiniMap {

ScalerResult<void> MiniMapAtlasScaler::Initialize(uint32_t atlasWidth, uint32_t atlasHeight, float minWorldX, float minWorldY, float maxWorldX, float maxWorldY) noexcept {
    if (atlasWidth == 0 || atlasHeight == 0) {
        return std::unexpected(ScalerError::InvalidDimensions);
    }

    if (minWorldX >= maxWorldX || minWorldY >= maxWorldY) {
        return std::unexpected(ScalerError::InvalidBounds);
    }

    m_atlasWidth = atlasWidth;
    m_atlasHeight = atlasHeight;
    m_minWorldX = minWorldX;
    m_minWorldY = minWorldY;
    m_maxWorldX = maxWorldX;
    m_maxWorldY = maxWorldY;

    float worldWidth = m_maxWorldX - m_minWorldX;
    float worldHeight = m_maxWorldY - m_minWorldY;

    m_scaleX = static_cast<float>(m_atlasWidth) / worldWidth;
    m_scaleY = static_cast<float>(m_atlasHeight) / worldHeight;

    m_initialized = true;

    return {};
}

ScalerResult<AtlasPosition> MiniMapAtlasScaler::WorldToAtlas(const WorldPosition& worldPos) const noexcept {
    if (!m_initialized) {
        return std::unexpected(ScalerError::NotInitialized);
    }

    // Pozwalamy na lekki margines bledu float, ale glownie sprawdzamy bounds
    if (worldPos.x < m_minWorldX || worldPos.x > m_maxWorldX ||
        worldPos.y < m_minWorldY || worldPos.y > m_maxWorldY) {
        return std::unexpected(ScalerError::OutOfBounds);
    }

    float localX = worldPos.x - m_minWorldX;
    float localY = worldPos.y - m_minWorldY;

    uint32_t atlasX = static_cast<uint32_t>(std::round(localX * m_scaleX));
    uint32_t atlasY = static_cast<uint32_t>(std::round(localY * m_scaleY));

    // Clamp by zapewnic ze nie przekroczymy wymiarow tablicy/tekstury
    if (atlasX >= m_atlasWidth) atlasX = m_atlasWidth > 0 ? m_atlasWidth - 1 : 0;
    if (atlasY >= m_atlasHeight) atlasY = m_atlasHeight > 0 ? m_atlasHeight - 1 : 0;

    return AtlasPosition{atlasX, atlasY};
}

ScalerResult<WorldPosition> MiniMapAtlasScaler::AtlasToWorld(const AtlasPosition& atlasPos) const noexcept {
    if (!m_initialized) {
        return std::unexpected(ScalerError::NotInitialized);
    }

    if (atlasPos.x >= m_atlasWidth || atlasPos.y >= m_atlasHeight) {
        return std::unexpected(ScalerError::OutOfBounds);
    }

    float localX = static_cast<float>(atlasPos.x) / m_scaleX;
    float localY = static_cast<float>(atlasPos.y) / m_scaleY;

    float worldX = m_minWorldX + localX;
    float worldY = m_minWorldY + localY;

    return WorldPosition{worldX, worldY};
}

} // namespace Client::MiniMap