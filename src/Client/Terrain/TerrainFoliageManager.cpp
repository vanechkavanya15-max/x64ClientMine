#include "TerrainFoliageManager.h"
#include <cmath>
#include <algorithm>

namespace Client::Terrain {

    TerrainFoliageManager::GridKey TerrainFoliageManager::GetGridKey(const Client::Core::MapCoords& position) const {
        return {
            static_cast<int>(std::floor(position.x / GRID_CELL_SIZE)),
            static_cast<int>(std::floor(position.y / GRID_CELL_SIZE))
        };
    }

    Client::Core::ExpectedResult<void, FoliageError> TerrainFoliageManager::AddFoliage(FoliageId id, const Client::Core::MapCoords& position) {
        if (!id) {
            return Client::Core::MakeUnexpected(FoliageError::InvalidId);
        }

        if (foliagePositions_.contains(id)) {
            return Client::Core::MakeUnexpected(FoliageError::AlreadyExists);
        }

        foliagePositions_.emplace(id, position);
        spatialGrid_[GetGridKey(position)].push_back(id);

        return {};
    }

    Client::Core::ExpectedResult<void, FoliageError> TerrainFoliageManager::RemoveFoliage(FoliageId id) {
        if (!id) {
            return Client::Core::MakeUnexpected(FoliageError::InvalidId);
        }

        auto it = foliagePositions_.find(id);
        if (it == foliagePositions_.end()) {
            return Client::Core::MakeUnexpected(FoliageError::NotFound);
        }

        const auto gridKey = GetGridKey(it->second);
        auto& gridCell = spatialGrid_[gridKey];

        auto cellIt = std::find(gridCell.begin(), gridCell.end(), id);
        if (cellIt != gridCell.end()) {
            gridCell.erase(cellIt);
        }

        foliagePositions_.erase(it);

        return {};
    }

    Client::Core::ExpectedResult<Client::Core::MapCoords, FoliageError> TerrainFoliageManager::GetFoliagePosition(FoliageId id) const {
        if (!id) {
            return Client::Core::MakeUnexpected(FoliageError::InvalidId);
        }

        auto it = foliagePositions_.find(id);
        if (it == foliagePositions_.end()) {
            return Client::Core::MakeUnexpected(FoliageError::NotFound);
        }

        return Client::Core::ExpectedResult<Client::Core::MapCoords, FoliageError>(it->second);
    }

    std::vector<FoliageId> TerrainFoliageManager::GetFoliageInRadius(const Client::Core::MapCoords& center, float radius) const {
        std::vector<FoliageId> result;
        const float radiusSq = radius * radius;

        int minX = static_cast<int>(std::floor((center.x - radius) / GRID_CELL_SIZE));
        int maxX = static_cast<int>(std::floor((center.x + radius) / GRID_CELL_SIZE));
        int minY = static_cast<int>(std::floor((center.y - radius) / GRID_CELL_SIZE));
        int maxY = static_cast<int>(std::floor((center.y + radius) / GRID_CELL_SIZE));

        for (int x = minX; x <= maxX; ++x) {
            for (int y = minY; y <= maxY; ++y) {
                GridKey key{x, y};
                auto it = spatialGrid_.find(key);
                if (it != spatialGrid_.end()) {
                    for (const auto& id : it->second) {
                        auto posIt = foliagePositions_.find(id);
                        if (posIt != foliagePositions_.end()) {
                            const auto& pos = posIt->second;
                            float dx = pos.x - center.x;
                            float dy = pos.y - center.y;
                            float dz = pos.z - center.z;
                            float distSq = dx * dx + dy * dy + dz * dz;

                            if (distSq <= radiusSq) {
                                result.push_back(id);
                            }
                        }
                    }
                }
            }
        }

        return result;
    }

} // namespace Client::Terrain
