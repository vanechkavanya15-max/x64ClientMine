#include "TerrainHeightSampler.h"

#include <cmath>
#include <algorithm>
#include <utility>

namespace Client::Terrain {

std::expected<void, TerrainError> TerrainHeightSampler::Initialize(uint32_t width, uint32_t height, std::span<const float> heights)
{
    if (width == 0 || height == 0)
    {
        return std::unexpected(TerrainError::InvalidGridSize);
    }

    if (heights.size() != static_cast<size_t>(width) * height)
    {
        return std::unexpected(TerrainError::InvalidGridSize);
    }

    m_width = width;
    m_height = height;
    m_gridHeights.assign(heights.begin(), heights.end());

    return {};
}

bool TerrainHeightSampler::IsInBoundsFloat(float x, float y) const noexcept
{
    return x >= 0.0f && x <= static_cast<float>(m_width - 1) &&
           y >= 0.0f && y <= static_cast<float>(m_height - 1);
}

float TerrainHeightSampler::GetRawHeight(uint32_t gridX, uint32_t gridY) const noexcept
{
    return m_gridHeights[gridY * m_width + gridX];
}

std::expected<float, TerrainError> TerrainHeightSampler::GetHeight(float x, float y) const noexcept
{
    if (m_gridHeights.empty())
    {
        return std::unexpected(TerrainError::DataNotLoaded);
    }

    if (!IsInBoundsFloat(x, y))
    {
        return std::unexpected(TerrainError::OutOfBounds);
    }

    const float floorX = std::floor(x);
    const float floorY = std::floor(y);

    const uint32_t x0 = static_cast<uint32_t>(floorX);
    const uint32_t y0 = static_cast<uint32_t>(floorY);

    const uint32_t x1 = std::min(x0 + 1, m_width - 1);
    const uint32_t y1 = std::min(y0 + 1, m_height - 1);

    const float fracX = x - floorX;
    const float fracY = y - floorY;

    const float h00 = GetRawHeight(x0, y0);
    const float h10 = GetRawHeight(x1, y0);
    const float h01 = GetRawHeight(x0, y1);
    const float h11 = GetRawHeight(x1, y1);

    const float interpTop = std::lerp(h00, h10, fracX);
    const float interpBottom = std::lerp(h01, h11, fracX);
    const float finalHeight = std::lerp(interpTop, interpBottom, fracY);

    return finalHeight;
}

std::expected<Vector3, TerrainError> TerrainHeightSampler::GetNormal(float x, float y) const noexcept
{
    if (m_gridHeights.empty())
    {
        return std::unexpected(TerrainError::DataNotLoaded);
    }

    if (!IsInBoundsFloat(x, y))
    {
        return std::unexpected(TerrainError::OutOfBounds);
    }

    const float floorX = std::floor(x);
    const float floorY = std::floor(y);

    const uint32_t x0 = static_cast<uint32_t>(floorX);
    const uint32_t y0 = static_cast<uint32_t>(floorY);

    const uint32_t x1 = std::min(x0 + 1, m_width - 1);
    const uint32_t y1 = std::min(y0 + 1, m_height - 1);

    const float fracX = x - floorX;
    const float fracY = y - floorY;

    const float h00 = GetRawHeight(x0, y0);
    const float h10 = GetRawHeight(x1, y0);
    const float h01 = GetRawHeight(x0, y1);
    const float h11 = GetRawHeight(x1, y1);

    const float dfdx = (h10 - h00) * (1.0f - fracY) + (h11 - h01) * fracY;
    const float dfdy = (h01 - h00) * (1.0f - fracX) + (h11 - h10) * fracX;

    Vector3 normal = { -dfdx, -dfdy, 1.0f };

    const float lengthSquared = (normal.x * normal.x) + (normal.y * normal.y) + (normal.z * normal.z);
    
    if (lengthSquared > 0.000001f)
    {
        const float inverseLength = 1.0f / std::sqrt(lengthSquared);
        normal.x *= inverseLength;
        normal.y *= inverseLength;
        normal.z *= inverseLength;
    }
    else
    {
        normal.x = 0.0f;
        normal.y = 0.0f;
        normal.z = 1.0f;
    }

    return normal;
}

} // namespace Client::Terrain
