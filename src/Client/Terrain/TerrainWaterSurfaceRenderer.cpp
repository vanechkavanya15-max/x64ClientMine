#include "TerrainWaterSurfaceRenderer.h"
#include <cmath>

namespace Client::Terrain
{
    TerrainWaterSurfaceRenderer::TerrainWaterSurfaceRenderer(IWaterRenderBackend* backend) noexcept
        : m_backend(backend)
    {
    }

    std::expected<void, WaterRendererError> TerrainWaterSurfaceRenderer::RenderWaterPatch(
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
                float displacement = std::sin(px * 0.1f + m_waveOffset) * std::cos(py * 0.1f + m_waveOffset) * 5.0f;

                WaterVertex v;
                v.position = { px, py, height + displacement };
                v.normal = { 0.0f, 0.0f, 1.0f }; // In a real scenario, calculate true normals based on displacement derivative
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
