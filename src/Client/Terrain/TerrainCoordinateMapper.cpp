#include "TerrainCoordinateMapper.h"

namespace Client::Terrain {

uint16_t TerrainCoordinateMapper::GetTerrainNum(float fx, float fy, uint16_t terrainCountX) noexcept {
    if (fx < 0.0f || fy < 0.0f) return 0;
    uint16_t sx = static_cast<uint16_t>(fx / SECTOR_SIZE);
    uint16_t sy = static_cast<uint16_t>(fy / SECTOR_SIZE);
    return sy * terrainCountX + sx;
}

MapCoordsResult TerrainCoordinateMapper::ConvertToMapCoords(float fx, float fy) noexcept {
    MapCoordsResult result{};

    if (fy < 0.0f) {
        fy = -fy;
    }

    float wrappedFx = std::fmod(fx, SECTOR_SIZE);
    if (wrappedFx < 0.0f) {
        wrappedFx += SECTOR_SIZE;
    }

    float wrappedFy = std::fmod(fy, SECTOR_SIZE);
    if (wrappedFy < 0.0f) {
        wrappedFy += SECTOR_SIZE;
    }

    int32_t origIx = static_cast<int32_t>(fx);
    int32_t origIy = static_cast<int32_t>(fy);

    result.terrainNumX = static_cast<uint16_t>(origIx / static_cast<int32_t>(SECTOR_SIZE));
    result.terrainNumY = static_cast<uint16_t>(origIy / static_cast<int32_t>(SECTOR_SIZE));

    constexpr float fooScale = 1.0f / static_cast<float>(HALF_CELLSCALE);
    
    result.cellX = static_cast<int32_t>(wrappedFx * fooScale);
    result.cellY = static_cast<int32_t>(wrappedFy * fooScale);

    constexpr float ratioScale = static_cast<float>(HEIGHT_TILE_XRATIO) * fooScale;
    
    uint32_t subCellX = static_cast<uint32_t>(wrappedFx * ratioScale);
    uint32_t subCellY = static_cast<uint32_t>(wrappedFy * ratioScale);
    
    result.subCellX = static_cast<uint8_t>(subCellX % HEIGHT_TILE_XRATIO);
    result.subCellY = static_cast<uint8_t>(subCellY % HEIGHT_TILE_YRATIO);

    return result;
}

PatchCoordsResult TerrainCoordinateMapper::GetPatchCoords(float fx, float fy) noexcept {
    if (fy < 0.0f) {
        fy = -fy;
    }

    float wrappedFx = std::fmod(fx, SECTOR_SIZE);
    if (wrappedFx < 0.0f) {
        wrappedFx += SECTOR_SIZE;
    }

    float wrappedFy = std::fmod(fy, SECTOR_SIZE);
    if (wrappedFy < 0.0f) {
        wrappedFy += SECTOR_SIZE;
    }

    int32_t x0 = static_cast<int32_t>(wrappedFx / static_cast<float>(CELLSCALE));
    int32_t y0 = static_cast<int32_t>(wrappedFy / static_cast<float>(CELLSCALE));

    PatchCoordsResult result{};
    result.patchNumX = static_cast<uint8_t>(x0 / PATCH_XSIZE);
    result.patchNumY = static_cast<uint8_t>(y0 / PATCH_YSIZE);

    return result;
}

} // namespace Client::Terrain
