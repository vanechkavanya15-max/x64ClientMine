#include "TerrainSectorLoader.h"

namespace Client::Terrain {

bool TerrainSectorLoader::LoadSector(uint16_t sectorId, const SectorChunkData& data) {
    if (IsSectorLoaded(sectorId)) {
        Touch(sectorId);
        return false;
    }

    if (m_chunks.size() >= m_maxCapacity && !m_lruList.empty()) {
        uint16_t oldest = m_lruList.back();
        UnloadSector(oldest);
    }

    SectorChunkData chunk = data;
    chunk.sectorId = sectorId;
    m_chunks[sectorId] = chunk;
    m_lruList.push_front(sectorId);
    m_lruMap[sectorId] = m_lruList.begin();
    return true;
}

bool TerrainSectorLoader::UnloadSector(uint16_t sectorId) {
    auto it = m_chunks.find(sectorId);
    if (it == m_chunks.end()) return false;

    m_chunks.erase(it);
    auto lruIt = m_lruMap.find(sectorId);
    if (lruIt != m_lruMap.end()) {
        m_lruList.erase(lruIt->second);
        m_lruMap.erase(lruIt);
    }
    return true;
}

bool TerrainSectorLoader::IsSectorLoaded(uint16_t sectorId) const noexcept {
    return m_chunks.find(sectorId) != m_chunks.end();
}

const SectorChunkData* TerrainSectorLoader::GetSectorData(uint16_t sectorId) const {
    auto it = m_chunks.find(sectorId);
    return it != m_chunks.end() ? &it->second : nullptr;
}

void TerrainSectorLoader::Touch(uint16_t sectorId) {
    auto it = m_lruMap.find(sectorId);
    if (it != m_lruMap.end()) {
        m_lruList.erase(it->second);
        m_lruList.push_front(sectorId);
        it->second = m_lruList.begin();
    }
}

void TerrainSectorLoader::ClearAll() noexcept {
    m_chunks.clear();
    m_lruList.clear();
    m_lruMap.clear();
}

} // namespace Client::Terrain
