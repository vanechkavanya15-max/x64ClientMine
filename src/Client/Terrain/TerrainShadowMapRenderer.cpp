#include "TerrainShadowMapRenderer.h"

namespace Client::Terrain {

TerrainShadowMapRenderer::TerrainShadowMapRenderer(std::shared_ptr<IShadowMapRenderBackend> backend) noexcept
    : m_backend(std::move(backend)) {
}

std::expected<void, ShadowMapError> TerrainShadowMapRenderer::Initialize(uint32_t resolution) noexcept {
    if (!m_backend) {
        return std::unexpected(ShadowMapError::BackendNotInitialized);
    }
    
    if (m_isInitialized) {
        return std::unexpected(ShadowMapError::AlreadyInitialized);
    }

    auto result = m_backend->InitializeTarget(resolution, resolution);
    if (!result.has_value()) {
        return result;
    }

    m_isInitialized = true;
    return {};
}

std::expected<void, ShadowMapError> TerrainShadowMapRenderer::BeginRenderPass() noexcept {
    if (!m_isInitialized) {
        return std::unexpected(ShadowMapError::BackendNotInitialized);
    }

    if (m_isPassActive) {
        return std::unexpected(ShadowMapError::RenderPassAlreadyActive);
    }

    auto result = m_backend->BeginTarget();
    if (!result.has_value()) {
        return result;
    }

    m_isPassActive = true;
    return {};
}

std::expected<void, ShadowMapError> TerrainShadowMapRenderer::EndRenderPass() noexcept {
    if (!m_isInitialized) {
        return std::unexpected(ShadowMapError::BackendNotInitialized);
    }

    if (!m_isPassActive) {
        return std::unexpected(ShadowMapError::NoActiveRenderPass);
    }

    auto result = m_backend->EndTarget();
    if (!result.has_value()) {
        return result;
    }

    m_isPassActive = false;
    return {};
}

std::expected<void, ShadowMapError> TerrainShadowMapRenderer::SetLightDirection(const Client::Core::MapCoords& direction) noexcept {
    if (!m_isInitialized) {
        return std::unexpected(ShadowMapError::BackendNotInitialized);
    }

    // Walidacja zerowego wektora kierunku
    if (direction.x == 0.0f && direction.y == 0.0f && direction.z == 0.0f) {
        return std::unexpected(ShadowMapError::InvalidLightDirection);
    }

    m_lightDirection = direction.Normalize();

    // Obliczamy macierz rzutowania cieni (SetShadowMatrix)
    // na podstawie znormalizowanego kierunku swiatla (prosta implementacja bez DirectX).


    float matrix[16] = {
        1.0f, 0.0f, 0.0f, 0.0f,
        0.0f, 1.0f, 0.0f, 0.0f,
        0.0f, 0.0f, 1.0f, 0.0f,
        m_lightDirection.x, m_lightDirection.y, m_lightDirection.z, 1.0f
    };
    return m_backend->SetShadowMatrix(matrix);
    

}

} // namespace Client::Terrain
