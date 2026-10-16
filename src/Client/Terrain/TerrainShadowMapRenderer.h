#pragma once

#include "IShadowMapRenderBackend.h"
#include <memory>
#include <expected>

namespace Client::Terrain {

// Renderer map cieni terenu izolujacy logike renderowania od bezposrednich zaleznosci Windows/DirectX API.
class TerrainShadowMapRenderer {
public:
    explicit TerrainShadowMapRenderer(std::shared_ptr<IShadowMapRenderBackend> backend) noexcept;
    
    // Brak kopiowania i przenoszenia (RAII)
    TerrainShadowMapRenderer(const TerrainShadowMapRenderer&) = delete;
    TerrainShadowMapRenderer& operator=(const TerrainShadowMapRenderer&) = delete;
    TerrainShadowMapRenderer(TerrainShadowMapRenderer&&) = delete;
    TerrainShadowMapRenderer& operator=(TerrainShadowMapRenderer&&) = delete;
    
    ~TerrainShadowMapRenderer() = default;

    // Inicjalizuje struktury rendera dla map cieni.
    std::expected<void, ShadowMapError> Initialize(uint32_t resolution) noexcept;

    // Rozpoczyna rendering do mapy cieni.
    std::expected<void, ShadowMapError> BeginRenderPass() noexcept;

    // Konczy rendering do mapy cieni.
    std::expected<void, ShadowMapError> EndRenderPass() noexcept;

    // Ustawia wektor kierunku swiatla (np. slonca).
    std::expected<void, ShadowMapError> SetLightDirection(const Client::Core::MapCoords& direction) noexcept;

private:
    std::shared_ptr<IShadowMapRenderBackend> m_backend;
    bool m_isInitialized{false};
    bool m_isPassActive{false};
    Client::Core::MapCoords m_lightDirection{0.0f, -1.0f, 0.0f}; // Domyslny kierunek swiatla (w dol)
};

} // namespace Client::Terrain
