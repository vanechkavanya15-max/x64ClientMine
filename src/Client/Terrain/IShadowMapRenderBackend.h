#pragma once

#include <expected>
#include <string_view>
#include <cstdint>
#include <format>
#include "../Core/StrongTypes.h"

namespace Client::Terrain {

enum class ShadowMapError : uint8_t {
    BackendNotInitialized,
    AlreadyInitialized,
    InvalidLightDirection,
    RenderPassAlreadyActive,
    NoActiveRenderPass,
    RenderTargetCreationFailure,
    TextureBindingFailure
};

[[nodiscard]] constexpr std::string_view to_string(ShadowMapError error) noexcept {
    switch (error) {
        case ShadowMapError::BackendNotInitialized: return "BackendNotInitialized";
        case ShadowMapError::AlreadyInitialized: return "AlreadyInitialized";
        case ShadowMapError::InvalidLightDirection: return "InvalidLightDirection";
        case ShadowMapError::RenderPassAlreadyActive: return "RenderPassAlreadyActive";
        case ShadowMapError::NoActiveRenderPass: return "NoActiveRenderPass";
        case ShadowMapError::RenderTargetCreationFailure: return "RenderTargetCreationFailure";
        case ShadowMapError::TextureBindingFailure: return "TextureBindingFailure";
        default: return "Unknown";
    }
}

class IShadowMapRenderBackend {
public:
    virtual ~IShadowMapRenderBackend() = default;

    virtual std::expected<void, ShadowMapError> InitializeTarget(uint32_t width, uint32_t height) noexcept = 0;
    virtual std::expected<void, ShadowMapError> BeginTarget() noexcept = 0;
    virtual std::expected<void, ShadowMapError> EndTarget() noexcept = 0;
    virtual std::expected<void, ShadowMapError> SetShadowMatrix(const float* matrixData) noexcept = 0;
};

} // namespace Client::Terrain

template <>
struct std::formatter<Client::Terrain::ShadowMapError> : std::formatter<std::string_view> {
    auto format(Client::Terrain::ShadowMapError err, std::format_context& ctx) const {
        return std::formatter<std::string_view>::format(Client::Terrain::to_string(err), ctx);
    }
};
