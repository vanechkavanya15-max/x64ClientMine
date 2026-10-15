#pragma once

#include <cstdint>
#include <vector>
#include <unordered_map>
#include <shared_mutex>
#include <expected>
#include <string>
#include <optional>
#include <cmath>

#include "../Core/StrongTypes.h"
#include "CollisionDetector.h"

namespace Client::World {

struct MeshIdTag {};
using MeshId = Client::Core::StrongType<MeshIdTag, uint32_t, 0>;

class AreaCollisionMeshIndex {
public:
    explicit AreaCollisionMeshIndex(float cellSize = 1024.0f);
    ~AreaCollisionMeshIndex() = default;

    std::expected<void, std::string> Insert(MeshId id, const AABB& aabb);
    std::expected<void, std::string> Update(MeshId id, const AABB& newAabb);
    std::expected<void, std::string> Remove(MeshId id);
    
    [[nodiscard]] std::vector<MeshId> QueryIntersecting(const AABB& area) const;
    [[nodiscard]] std::optional<AABB> GetBounds(MeshId id) const;
    [[nodiscard]] size_t Count() const;
    void Clear();

private:
    struct CellCoords {
        int x;
        int y;

        bool operator==(const CellCoords& other) const {
            return x == other.x && y == other.y;
        }
    };

    struct CellCoordsHash {
        std::size_t operator()(const CellCoords& coords) const {
            std::size_t h1 = std::hash<int>()(coords.x);
            std::size_t h2 = std::hash<int>()(coords.y);
            return h1 ^ (h2 + 0x9e3779b9 + (h1 << 6) + (h1 >> 2));
        }
    };

    mutable std::shared_mutex m_mutex;
    float m_cellSize;
    std::unordered_map<MeshId, AABB> m_meshes;
    std::unordered_map<CellCoords, std::vector<MeshId>, CellCoordsHash> m_cells;

    CellCoords GetCellCoords(float x, float y) const;
    bool IsAABBValid(const AABB& aabb) const;
    bool Intersects(const AABB& a, const AABB& b) const;
    void RemoveFromCells(MeshId id, const AABB& aabb);
    void InsertIntoCells(MeshId id, const AABB& aabb);
};

} // namespace Client::World
