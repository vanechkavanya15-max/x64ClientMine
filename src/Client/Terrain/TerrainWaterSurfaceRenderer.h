#pragma once

#include "Client/Core/StrongTypes.h"
#include <vector>
#include <cstdint>
#include <expected>
#include "Client/Core/Result.h"
#include <string_view>
#include <format>

namespace Client::Terrain
{
    // C++23 enum representing specific errors for the water renderer
    enum class WaterRendererError : uint8_t
    {
        InvalidSize,
        InvalidGridResolution,
        BackendFailure,
        InvalidState
    };

    constexpr std::string_view to_string(WaterRendererError err) noexcept
    {
        switch (err)
        {
            case WaterRendererError::InvalidSize: return "InvalidSize";
            case WaterRendererError::InvalidGridResolution: return "InvalidGridResolution";
            case WaterRendererError::BackendFailure: return "BackendFailure";
            case WaterRendererError::InvalidState: return "InvalidState";
            default: return "Unknown WaterRendererError";
        }
    }

    // Abstract backend interface to completely decouple from D3D9/Windows API
    struct WaterVertex
    {
        Client::Core::MapCoords position;
        Client::Core::MapCoords normal;
        float u, v;
    };

    class IWaterRenderBackend
    {
    public:
        virtual ~IWaterRenderBackend() = default;
        virtual Client::Core::Result<void, WaterRendererError> DrawWaterPatch(
            const std::vector<WaterVertex>& vertices,
            const std::vector<uint16_t>& indices) = 0;
    };

    class TerrainWaterSurfaceRenderer
    {
    public:
        explicit TerrainWaterSurfaceRenderer(IWaterRenderBackend* backend) noexcept;

        // Renders a water patch via the injected backend.
        // Returns an expected representing success or a strictly typed error.
        Client::Core::Result<void, WaterRendererError> RenderWaterPatch(
            const Client::Core::MapCoords& worldPos,
            float size,
            float height,
            uint32_t gridResolution) const;

        void UpdateWaveOffset(float timeSeconds) noexcept;

    private:
        IWaterRenderBackend* m_backend;
        float m_waveOffset = 0.0f;
    };
} // namespace Client::Terrain

template <>
struct std::formatter<Client::Terrain::WaterRendererError> : std::formatter<std::string_view> {
    auto format(Client::Terrain::WaterRendererError err, std::format_context& ctx) const {
        return std::formatter<std::string_view>::format(Client::Terrain::to_string(err), ctx);
    }
};

