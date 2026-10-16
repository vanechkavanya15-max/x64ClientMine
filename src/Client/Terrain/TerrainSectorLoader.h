#pragma once
#include <cstdint>
#include <unordered_set>

namespace Client::Terrain {

class TerrainSectorLoader {
public:
    TerrainSectorLoader() = default;
    ~TerrainSectorLoader() = default;

    bool LoadSector(uint16_t sectorId) {
        return m_loadedSectors.insert(sectorId).second;
    }
    bool UnloadSector(uint16_t sectorId) {
        return m_loadedSectors.erase(sectorId) > 0;
    }
    [[nodiscard]] bool IsSectorLoaded(uint16_t sectorId) const noexcept {
        return m_loadedSectors.find(sectorId) != m_loadedSectors.end();
    }
    [[nodiscard]] size_t GetLoadedSectorCount() const noexcept { return m_loadedSectors.size(); }

private:
    std::unordered_set<uint16_t> m_loadedSectors;
};

} // namespace Client::Terrain
