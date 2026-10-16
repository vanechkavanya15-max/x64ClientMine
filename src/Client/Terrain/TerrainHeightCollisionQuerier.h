#pragma once

#include "../Core/Result.h"
#include "../Core/StrongTypes.h"
#include <cstdint>
#include <vector>
#include <memory>
#include <expected>
#include <shared_mutex>
#include <unordered_map>
#include <optional>
#include <cmath>

namespace Client::Terrain {

/**
 * @brief Domain errors for terrain height and collision queries.
 */
enum class TerrainQueryError : uint8_t {
    None = 0,
    PatchNotLoaded,
    OutOfBounds,
    InvalidCoordinates,
    RaycastTimeout
};

[[nodiscard]] inline std::string_view to_string(TerrainQueryError error) noexcept {
    switch (error) {
        case TerrainQueryError::None: return "TerrainQueryError::None";
        case TerrainQueryError::PatchNotLoaded: return "TerrainQueryError::PatchNotLoaded - Terrain patch is not loaded in memory";
        case TerrainQueryError::OutOfBounds: return "TerrainQueryError::OutOfBounds - Query coordinates are outside patch bounds";
        case TerrainQueryError::InvalidCoordinates: return "TerrainQueryError::InvalidCoordinates - Invalid coordinate values provided";
        case TerrainQueryError::RaycastTimeout: return "TerrainQueryError::RaycastTimeout - Raycast exceeded maximum iteration depth";
        default: return "TerrainQueryError::Unknown";
    }
}

/**
 * @brief Result structure for a successful raycast.
 */
struct RaycastHit {
    Client::Core::MapCoords hitPoint;
    Client::Core::MapCoords surfaceNormal;
    float distance;
};

/**
 * @brief Immutable patch configuration constants.
 */
struct TerrainConfig {
    static constexpr uint16_t PATCH_SIZE_X = 128;
    static constexpr uint16_t PATCH_SIZE_Y = 128;
    static constexpr float GRID_SIZE = 200.0f; // 2 meters per grid unit
    static constexpr float PATCH_WORLD_SIZE_X = PATCH_SIZE_X * GRID_SIZE;
    static constexpr float PATCH_WORLD_SIZE_Y = PATCH_SIZE_Y * GRID_SIZE;
    static constexpr float MAX_RAYCAST_DISTANCE = 50000.0f; // 500 meters
    static constexpr uint32_t MAX_RAYCAST_STEPS = 1000;
};

/**
 * @brief Internal structural representation of a single 128x128 terrain patch.
 */
class TerrainPatch {
public:
    TerrainPatch(int32_t patchX, int32_t patchY);
    ~TerrainPatch() = default;

    TerrainPatch(const TerrainPatch&) = delete;
    TerrainPatch& operator=(const TerrainPatch&) = delete;
    TerrainPatch(TerrainPatch&&) noexcept = default;
    TerrainPatch& operator=(TerrainPatch&&) noexcept = default;

    void SetHeightData(std::vector<float> heights);
    void SetWaterData(std::vector<float> waterHeights);

    [[nodiscard]] Client::Core::Result<float, TerrainQueryError> GetHeight(float localX, float localY) const noexcept;
    [[nodiscard]] Client::Core::Result<float, TerrainQueryError> GetWaterHeight(float localX, float localY) const noexcept;
    [[nodiscard]] Client::Core::Result<Client::Core::MapCoords, TerrainQueryError> GetNormal(float localX, float localY) const noexcept;
    [[nodiscard]] Client::Core::Result<bool, TerrainQueryError> IsUnderWater(float localX, float localY) const noexcept;

    [[nodiscard]] int32_t GetPatchX() const noexcept { return patchX_; }
    [[nodiscard]] int32_t GetPatchY() const noexcept { return patchY_; }

private:
    int32_t patchX_;
    int32_t patchY_;
    std::vector<float> heights_; // Size: PATCH_SIZE_X * PATCH_SIZE_Y
    std::vector<float> waterHeights_; // Size: PATCH_SIZE_X * PATCH_SIZE_Y

    [[nodiscard]] float GetRawHeight(uint16_t x, uint16_t y) const noexcept;
};

/**
 * @brief Thread-safe manager for terrain height, water, and raycast collision queries.
 * Adheres to SRP by handling spatial translations and query dispatching without any rendering/OS dependencies.
 */
class TerrainHeightCollisionQuerier {
public:
    TerrainHeightCollisionQuerier() = default;
    ~TerrainHeightCollisionQuerier() = default;

    TerrainHeightCollisionQuerier(const TerrainHeightCollisionQuerier&) = delete;
    TerrainHeightCollisionQuerier& operator=(const TerrainHeightCollisionQuerier&) = delete;

    /**
     * @brief Loads or updates a terrain patch with raw height/water data.
     * @param patchX The global X coordinate of the patch.
     * @param patchY The global Y coordinate of the patch.
     * @param heights A vector of 128x128 floats representing terrain height.
     * @param waterHeights A vector of 128x128 floats representing water height.
     */
    void LoadPatch(int32_t patchX, int32_t patchY, std::vector<float> heights, std::vector<float> waterHeights);

    /**
     * @brief Unloads a terrain patch from memory.
     * @param patchX The global X coordinate of the patch.
     * @param patchY The global Y coordinate of the patch.
     */
    void UnloadPatch(int32_t patchX, int32_t patchY);
    
    /**
     * @brief Clears all loaded patches.
     */
    void UnloadAllPatches();

    /**
     * @brief Queries the interpolated terrain height at global world coordinates.
     * @param x Global X coordinate.
     * @param y Global Y coordinate.
     * @return std::expected containing the height, or a TerrainQueryError.
     */
    [[nodiscard]] Client::Core::Result<float, TerrainQueryError> GetHeight(float x, float y) const noexcept;

    /**
     * @brief Queries the interpolated water height at global world coordinates.
     * @param x Global X coordinate.
     * @param y Global Y coordinate.
     * @return std::expected containing the water height, or a TerrainQueryError.
     */
    [[nodiscard]] Client::Core::Result<float, TerrainQueryError> GetWaterHeight(float x, float y) const noexcept;

    /**
     * @brief Determines if a specific global coordinate is submerged under water.
     * @param x Global X coordinate.
     * @param y Global Y coordinate.
     * @return std::expected containing true if underwater, or a TerrainQueryError.
     */
    [[nodiscard]] Client::Core::Result<bool, TerrainQueryError> IsUnderWater(float x, float y) const noexcept;

    /**
     * @brief Calculates the surface normal vector at global world coordinates via central difference.
     * @param x Global X coordinate.
     * @param y Global Y coordinate.
     * @return std::expected containing the normalized MapCoords vector, or a TerrainQueryError.
     */
    [[nodiscard]] Client::Core::Result<Client::Core::MapCoords, TerrainQueryError> GetNormal(float x, float y) const noexcept;

    /**
     * @brief Performs a 3D raycast against the terrain heightmap using a robust DDA traversal algorithm.
     * @param origin The starting point of the ray.
     * @param direction The normalized direction vector of the ray.
     * @param maxDistance The maximum distance to traverse.
     * @return std::expected containing RaycastHit details if a collision occurs, or a TerrainQueryError if none/timeout.
     */
    [[nodiscard]] Client::Core::Result<RaycastHit, TerrainQueryError> Raycast(
        const Client::Core::MapCoords& origin, 
        const Client::Core::MapCoords& direction, 
        float maxDistance = TerrainConfig::MAX_RAYCAST_DISTANCE) const noexcept;

    /**
     * @brief Determines the internal patch coordinates for a given global coordinate.
     */
    [[nodiscard]] static std::pair<int32_t, int32_t> WorldToPatchCoords(float x, float y) noexcept;

    /**
     * @brief Computes local coordinates within a specific patch from global coordinates.
     */
    [[nodiscard]] static std::pair<float, float> WorldToLocalCoords(float x, float y, int32_t patchX, int32_t patchY) noexcept;

private:
    mutable std::shared_mutex mutex_;
    
    // Hash function for std::pair
    struct PairHash {
        std::size_t operator()(const std::pair<int32_t, int32_t>& p) const noexcept {
            std::size_t h1 = std::hash<int32_t>{}(p.first);
            std::size_t h2 = std::hash<int32_t>{}(p.second);
            return h1 ^ (h2 + 0x9e3779b9 + (h1 << 6) + (h1 >> 2));
        }
    };

    std::unordered_map<std::pair<int32_t, int32_t>, std::unique_ptr<TerrainPatch>, PairHash> patches_;

    [[nodiscard]] const TerrainPatch* GetPatchInternal(int32_t patchX, int32_t patchY) const noexcept;
};

} // namespace Client::Terrain
