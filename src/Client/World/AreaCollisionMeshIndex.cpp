#include "AreaCollisionMeshIndex.h"
#include <mutex>
#include <algorithm>
#include <unordered_set>

namespace Client::World {

AreaCollisionMeshIndex::AreaCollisionMeshIndex(float cellSize) : m_cellSize(cellSize > 0.0f ? cellSize : 1024.0f) {}

AreaCollisionMeshIndex::CellCoords AreaCollisionMeshIndex::GetCellCoords(float x, float y) const {
    return {
        static_cast<int>(std::floor(x / m_cellSize)),
        static_cast<int>(std::floor(y / m_cellSize))
    };
}

bool AreaCollisionMeshIndex::IsAABBValid(const AABB& aabb) const {
    if (!std::isfinite(aabb.min.x) || !std::isfinite(aabb.min.y) || !std::isfinite(aabb.min.z)) return false;
    if (!std::isfinite(aabb.max.x) || !std::isfinite(aabb.max.y) || !std::isfinite(aabb.max.z)) return false;
    return aabb.min.x <= aabb.max.x && aabb.min.y <= aabb.max.y && aabb.min.z <= aabb.max.z;
}

bool AreaCollisionMeshIndex::Intersects(const AABB& a, const AABB& b) const {
    return (a.min.x <= b.max.x && a.max.x >= b.min.x) &&
           (a.min.y <= b.max.y && a.max.y >= b.min.y) &&
           (a.min.z <= b.max.z && a.max.z >= b.min.z);
}

void AreaCollisionMeshIndex::RemoveFromCells(MeshId id, const AABB& aabb) {
    CellCoords minCoords = GetCellCoords(aabb.min.x, aabb.min.y);
    CellCoords maxCoords = GetCellCoords(aabb.max.x, aabb.max.y);

    for (int y = minCoords.y; y <= maxCoords.y; ++y) {
        for (int x = minCoords.x; x <= maxCoords.x; ++x) {
            auto it = m_cells.find({x, y});
            if (it != m_cells.end()) {
                auto& cell = it->second;
                auto cellIt = std::find(cell.begin(), cell.end(), id);
                if (cellIt != cell.end()) {
                    if (cellIt != cell.end() - 1) {
                        std::iter_swap(cellIt, cell.end() - 1);
                    }
                    cell.pop_back();
                }
                if (cell.empty()) {
                    m_cells.erase(it);
                }
            }
        }
    }
}

void AreaCollisionMeshIndex::InsertIntoCells(MeshId id, const AABB& aabb) {
    CellCoords minCoords = GetCellCoords(aabb.min.x, aabb.min.y);
    CellCoords maxCoords = GetCellCoords(aabb.max.x, aabb.max.y);

    for (int y = minCoords.y; y <= maxCoords.y; ++y) {
        for (int x = minCoords.x; x <= maxCoords.x; ++x) {
            m_cells[{x, y}].push_back(id);
        }
    }
}

std::expected<void, std::string> AreaCollisionMeshIndex::Insert(MeshId id, const AABB& aabb) {
    if (!IsAABBValid(aabb)) {
        return std::unexpected("Nieprawidlowe AABB przy probie wstawienia.");
    }

    std::unique_lock lock(m_mutex);
    if (m_meshes.contains(id)) {
        return std::unexpected("Siatka o podanym ID juz istnieje.");
    }

    m_meshes[id] = aabb;
    InsertIntoCells(id, aabb);

    return {};
}

std::expected<void, std::string> AreaCollisionMeshIndex::Update(MeshId id, const AABB& newAabb) {
    if (!IsAABBValid(newAabb)) {
        return std::unexpected("Nieprawidlowe AABB przy probie aktualizacji.");
    }

    std::unique_lock lock(m_mutex);
    auto it = m_meshes.find(id);
    if (it == m_meshes.end()) {
        return std::unexpected("Siatka o podanym ID nie istnieje.");
    }

    // Optimization: check if AABB is exactly the same
    if (it->second.min.x == newAabb.min.x && it->second.min.y == newAabb.min.y && it->second.min.z == newAabb.min.z &&
        it->second.max.x == newAabb.max.x && it->second.max.y == newAabb.max.y && it->second.max.z == newAabb.max.z) {
        return {};
    }

    CellCoords oldMinCoords = GetCellCoords(it->second.min.x, it->second.min.y);
    CellCoords oldMaxCoords = GetCellCoords(it->second.max.x, it->second.max.y);
    CellCoords newMinCoords = GetCellCoords(newAabb.min.x, newAabb.min.y);
    CellCoords newMaxCoords = GetCellCoords(newAabb.max.x, newAabb.max.y);

    if (oldMinCoords == newMinCoords && oldMaxCoords == newMaxCoords) {
        // Only AABB values changed but cells remain the same
        it->second = newAabb;
        return {};
    }

    RemoveFromCells(id, it->second);
    it->second = newAabb;
    InsertIntoCells(id, newAabb);

    return {};
}

std::expected<void, std::string> AreaCollisionMeshIndex::Remove(MeshId id) {
    std::unique_lock lock(m_mutex);
    auto it = m_meshes.find(id);
    if (it == m_meshes.end()) {
        return std::unexpected("Siatka o podanym ID nie istnieje.");
    }

    RemoveFromCells(id, it->second);
    m_meshes.erase(it);

    return {};
}

std::vector<MeshId> AreaCollisionMeshIndex::QueryIntersecting(const AABB& area) const {
    std::vector<MeshId> result;
    if (!IsAABBValid(area)) {
        return result;
    }

    std::shared_lock lock(m_mutex);

    CellCoords minCoords = GetCellCoords(area.min.x, area.min.y);
    CellCoords maxCoords = GetCellCoords(area.max.x, area.max.y);

    size_t spanX = static_cast<size_t>(std::max(1, maxCoords.x - minCoords.x + 1));
    size_t spanY = static_cast<size_t>(std::max(1, maxCoords.y - minCoords.y + 1));

    // O(N) fallback if searching too many cells compared to total elements
    if (spanX > 100 || spanY > 100 || (spanX * spanY) > m_meshes.size()) {
        result.reserve(std::min(size_t(32), m_meshes.size()));
        for (const auto& [id, aabb] : m_meshes) {
            if (Intersects(area, aabb)) {
                result.push_back(id);
            }
        }
        return result;
    }

    // Use a set or similar approach to deduplicate as meshes might span multiple cells
    // Since we expect the result to be small, appending and then std::unique is faster than unordered_set
    result.reserve(32);

    for (int y = minCoords.y; y <= maxCoords.y; ++y) {
        for (int x = minCoords.x; x <= maxCoords.x; ++x) {
            auto it = m_cells.find({x, y});
            if (it != m_cells.end()) {
                for (MeshId id : it->second) {
                    auto meshIt = m_meshes.find(id);
                    if (meshIt != m_meshes.end() && Intersects(area, meshIt->second)) {
                        result.push_back(id);
                    }
                }
            }
        }
    }

    std::sort(result.begin(), result.end());
    result.erase(std::unique(result.begin(), result.end()), result.end());

    return result;
}

std::optional<AABB> AreaCollisionMeshIndex::GetBounds(MeshId id) const {
    std::shared_lock lock(m_mutex);
    auto it = m_meshes.find(id);
    if (it != m_meshes.end()) {
        return it->second;
    }
    return std::nullopt;
}

size_t AreaCollisionMeshIndex::Count() const {
    std::shared_lock lock(m_mutex);
    return m_meshes.size();
}

void AreaCollisionMeshIndex::Clear() {
    std::unique_lock lock(m_mutex);
    m_meshes.clear();
    m_cells.clear();
}

} // namespace Client::World
