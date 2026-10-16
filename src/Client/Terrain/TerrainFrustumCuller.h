#pragma once

#include <vector>
#include <span>
#include <array>
#include <cstdint>
#include <expected>
#include "../Core/StrongTypes.h"

namespace Client::Terrain {

struct TerrainAABB {
    Client::Core::MapCoords min;
    Client::Core::MapCoords max;
};

struct FrustumPlane {
    float a, b, c, d;
};

enum class CullerError {
    InvalidPlanesCount,
    PlanesNotSet
};

class TerrainFrustumCuller {
public:
    TerrainFrustumCuller() = default;
    ~TerrainFrustumCuller() = default;

    std::expected<void, CullerError> SetFrustumPlanes(std::span<const FrustumPlane> planes) noexcept;
    
    [[nodiscard]] std::expected<bool, CullerError> IsBoxVisible(const TerrainAABB& box) const noexcept;
    
    std::expected<void, CullerError> CullQuadTree(const TerrainAABB& root, std::vector<uint32_t>& visibleNodeIds) const;

private:
    std::array<FrustumPlane, 6> m_planes{};
    bool m_planesSet{false};
};

} // namespace Client::Terrain
