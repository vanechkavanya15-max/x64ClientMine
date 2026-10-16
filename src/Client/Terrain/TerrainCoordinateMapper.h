#pragma once
#include <cstdint>

namespace Client::Terrain {

class TerrainCoordinateMapper {
public:
    static constexpr float SECTOR_SIZE = 25600.0f; // 256m * 100

    [[nodiscard]] static uint16_t GetTerrainNum(float fx, float fy, uint16_t terrainCountX = 3) noexcept {
        if (fx < 0.0f || fy < 0.0f) return 0;
        uint16_t sx = static_cast<uint16_t>(fx / SECTOR_SIZE);
        uint16_t sy = static_cast<uint16_t>(fy / SECTOR_SIZE);
        return sy * terrainCountX + sx;
    }
};

} // namespace Client::Terrain
