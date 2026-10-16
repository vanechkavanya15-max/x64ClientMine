#pragma once

#include "Core/StrongTypes.h"
#include "Core/ExpectedResult.h"
#include <unordered_map>
#include <vector>
#include <cstdint>
#include <string_view>

namespace Client::Terrain {

    enum class FoliageError : uint8_t {
        None = 0,
        InvalidId,
        NotFound,
        AlreadyExists
    };

    struct FoliageIdTag {};
    using FoliageId = Client::Core::StrongType<FoliageIdTag, uint32_t, 0>;

    class TerrainFoliageManager {
    public:
        TerrainFoliageManager() = default;
        ~TerrainFoliageManager() = default;

        // No copy or move semantics for managers.
        TerrainFoliageManager(const TerrainFoliageManager&) = delete;
        TerrainFoliageManager& operator=(const TerrainFoliageManager&) = delete;

        Client::Core::ExpectedResult<void, FoliageError> AddFoliage(FoliageId id, const Client::Core::MapCoords& position);
        Client::Core::ExpectedResult<void, FoliageError> RemoveFoliage(FoliageId id);

        Client::Core::ExpectedResult<Client::Core::MapCoords, FoliageError> GetFoliagePosition(FoliageId id) const;
        std::vector<FoliageId> GetFoliageInRadius(const Client::Core::MapCoords& center, float radius) const;

    private:
        static constexpr float GRID_CELL_SIZE = 1000.0f;

        struct GridKey {
            int x;
            int y;

            bool operator==(const GridKey& other) const {
                return x == other.x && y == other.y;
            }
        };

        struct GridKeyHasher {
            std::size_t operator()(const GridKey& key) const {
                return std::hash<int>()(key.x) ^ (std::hash<int>()(key.y) << 1);
            }
        };

        GridKey GetGridKey(const Client::Core::MapCoords& position) const;

        std::unordered_map<FoliageId, Client::Core::MapCoords> foliagePositions_;
        std::unordered_map<GridKey, std::vector<FoliageId>, GridKeyHasher> spatialGrid_;
    };

} // namespace Client::Terrain
