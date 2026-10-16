#include "TerrainWaterSurfaceRenderer.h"
#include <cmath>

namespace Client::Terrain
{
    TerrainWaterSurfaceRenderer::TerrainWaterSurfaceRenderer(IWaterRenderBackend* backend) noexcept
        : m_backend(backend)
    {
    }

    Client::Core::Result<void, WaterRendererError> TerrainWaterSurfaceRenderer::RenderWaterPatch(
        const Client::Core::MapCoords& worldPos,
        float size,
        float height,
        uint32_t gridResolution) const
    {
        if (size <= 0.0f)
        {
            return std::unexpected(WaterRendererError::InvalidSize);
        }

        if (gridResolution == 0 || gridResolution > 128) // Reasonable upper bound
        {
            return std::unexpected(WaterRendererError::InvalidGridResolution);
        }

        if (!m_backend)
        {
            return std::unexpected(WaterRendererError::InvalidState);
        }

        std::vector<WaterVertex> vertices;
        std::vector<uint16_t> indices;

        // Reserve memory for the vectors to avoid reallocations
        vertices.reserve((gridResolution + 1) * (gridResolution + 1));
        indices.reserve(gridResolution * gridResolution * 6);

        const float cellSize = size / static_cast<float>(gridResolution);

        for (uint32_t y = 0; y <= gridResolution; ++y)
        {
            for (uint32_t x = 0; x <= gridResolution; ++x)
            {
                float px = worldPos.x + static_cast<float>(x) * cellSize;
                float py = worldPos.y + static_cast<float>(y) * cellSize;

                // Procedural displacement (simple sine wave for demo/legacy parity)
                // Procedural displacement function: z(x,y) = A * sin(kx * x + wt) * cos(ky * y + wt)
                // A = 5.0, kx = 0.1, ky = 0.1, wt = m_waveOffset
                float A = 5.0f;
                float kx = 0.1f;
                float ky = 0.1f;
                float wt = m_waveOffset;
                
                float sinX = std::sin(kx * px + wt);
                float cosX = std::cos(kx * px + wt);
                float sinY = std::sin(ky * py + wt);
                float cosY = std::cos(ky * py + wt);
                
                float displacement = A * sinX * cosY;

                // Analytical normal using partial derivatives
                // dz/dx = A * kx * cos(kx * x + wt) * cos(ky * y + wt)
                // dz/dy = A * ky * sin(kx * x + wt) * -sin(ky * y + wt)
                float dzdx = A * kx * cosX * cosY;
                float dzdy = -A * ky * sinX * sinY;
                
                // Normal is cross product of tangent vectors: (-dz/dx, -dz/dy, 1)
                Client::Core::MapCoords normalRaw = { -dzdx, -dzdy, 1.0f };
                Client::Core::MapCoords normal = normalRaw.Normalize();

                WaterVertex v;
                v.position = { px, py, height + displacement };
                v.normal = normal;
                v.u = static_cast<float>(x) / static_cast<float>(gridResolution);
                v.v = static_cast<float>(y) / static_cast<float>(gridResolution);

                vertices.push_back(v);
            }
        }

        for (uint32_t y = 0; y < gridResolution; ++y)
        {
            for (uint32_t x = 0; x < gridResolution; ++x)
            {
                uint16_t i0 = static_cast<uint16_t>(y * (gridResolution + 1) + x);
                uint16_t i1 = i0 + 1;
                uint16_t i2 = static_cast<uint16_t>((y + 1) * (gridResolution + 1) + x);
                uint16_t i3 = i2 + 1;

                // Triangle 1
                indices.push_back(i0);
                indices.push_back(i1);
                indices.push_back(i2);

                // Triangle 2
                indices.push_back(i1);
                indices.push_back(i3);
                indices.push_back(i2);
            }
        }

        auto result = m_backend->DrawWaterPatch(vertices, indices);
        if (!result)
        {
            // Propagate the backend error (or map it if desired)
            return std::unexpected(WaterRendererError::BackendFailure);
        }

        return {};
    }

    void TerrainWaterSurfaceRenderer::UpdateWaveOffset(float timeSeconds) noexcept
    {
        m_waveOffset = timeSeconds;
    }
} // namespace Client::Terrain
