#pragma once

#include "Client/Core/StrongTypes.h"
#include <vector>
#include <cstdint>
#include <expected>

namespace Client::Terrain
{
    // C++23 enum representing specific errors for the water renderer
    enum class WaterRendererError
    {
        InvalidSize,
        InvalidGridResolution,
        BackendFailure,
        InvalidState
    };

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
        virtual std::expected<void, WaterRendererError> DrawWaterPatch(
            const std::vector<WaterVertex>& vertices,
            const std::vector<uint16_t>& indices) = 0;
    };

    class TerrainWaterSurfaceRenderer
    {
    public:
        explicit TerrainWaterSurfaceRenderer(IWaterRenderBackend* backend) noexcept;

        // Renders a water patch via the injected backend.
        // Returns an expected representing success or a strictly typed error.
        std::expected<void, WaterRendererError> RenderWaterPatch(
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
