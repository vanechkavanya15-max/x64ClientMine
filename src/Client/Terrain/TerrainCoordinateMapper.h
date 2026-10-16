#pragma once
#include <cstdint>
#include <cmath>

namespace Client::Terrain {

struct MapCoordsResult {
    uint16_t terrainNumX{0};
    uint16_t terrainNumY{0};
    int32_t cellX{0};
    int32_t cellY{0};
    uint8_t subCellX{0};
    uint8_t subCellY{0};
};

struct PatchCoordsResult {
    uint8_t patchNumX{0};
    uint8_t patchNumY{0};
};

class TerrainCoordinateMapper {
public:
    static constexpr uint32_t TERRAIN_SIZE = 128;
    static constexpr uint32_t CELLSCALE = 200;
    static constexpr uint32_t HALF_CELLSCALE = CELLSCALE / 2;
    
    static constexpr float SECTOR_SIZE = static_cast<float>(TERRAIN_SIZE * CELLSCALE); // 25600.0f
    
    static constexpr uint32_t PATCH_XSIZE = 16;
    static constexpr uint32_t PATCH_YSIZE = 16;
    
    static constexpr uint32_t HEIGHT_TILE_XRATIO = 2; // TILEMAP_XSIZE (256) / XSIZE (128)
    static constexpr uint32_t HEIGHT_TILE_YRATIO = 2;

    [[nodiscard]] static uint16_t GetTerrainNum(float fx, float fy, uint16_t terrainCountX = 3) noexcept;
    
    [[nodiscard]] static MapCoordsResult ConvertToMapCoords(float fx, float fy) noexcept;

    [[nodiscard]] static PatchCoordsResult GetPatchCoords(float fx, float fy) noexcept;
};

} // namespace Client::Terrain
