#pragma once
#include <cstdint>
#include <unordered_map>
#include <list>
#include <vector>

namespace Client::Terrain {

struct SectorChunkData {
    uint16_t sectorId{0};
    uint32_t vertexCount{0};
    float minX{0.0f};
    float minY{0.0f};
    float maxX{0.0f};
    float maxY{0.0f};
};

class TerrainSectorLoader {
public:
    explicit TerrainSectorLoader(size_t maxCachedChunks = 16) : m_maxCapacity(maxCachedChunks) {}
    ~TerrainSectorLoader() = default;

    bool LoadSector(uint16_t sectorId, const SectorChunkData& data = {});
    bool UnloadSector(uint16_t sectorId);
    [[nodiscard]] bool IsSectorLoaded(uint16_t sectorId) const noexcept;
    [[nodiscard]] size_t GetLoadedSectorCount() const noexcept { return m_chunks.size(); }
    [[nodiscard]] const SectorChunkData* GetSectorData(uint16_t sectorId) const;

    void SetCapacity(size_t cap) noexcept { m_maxCapacity = cap > 0 ? cap : 1; }
    void ClearAll() noexcept;

private:
    void Touch(uint16_t sectorId);

    size_t m_maxCapacity{16};
    std::unordered_map<uint16_t, SectorChunkData> m_chunks;
    std::list<uint16_t> m_lruList;
    std::unordered_map<uint16_t, std::list<uint16_t>::iterator> m_lruMap;
};

} // namespace Client::Terrain
