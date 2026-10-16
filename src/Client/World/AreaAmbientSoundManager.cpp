#include "AreaAmbientSoundManager.h"

namespace Client::World {

std::expected<void, AmbientSoundError> AreaAmbientSoundManager::RegisterSound(
    Core::AmbientSoundId id,
    const std::string& filePath,
    const Core::MapCoords& position,
    float maxRadius,
    float baseVolume)
{
    if (!id) {
        return std::unexpected(AmbientSoundError::InvalidId);
    }
    
    if (baseVolume < 0.0f || baseVolume > 1.0f) {
        return std::unexpected(AmbientSoundError::InvalidVolume);
    }
    
    if (maxRadius <= 0.0f) {
        return std::unexpected(AmbientSoundError::InvalidRadius);
    }

    if (m_sounds.contains(id)) {
        return std::unexpected(AmbientSoundError::AlreadyExists);
    }

    m_sounds.emplace(id, AmbientSoundRecord{id, filePath, position, maxRadius, baseVolume});
    return {};
}

std::expected<void, AmbientSoundError> AreaAmbientSoundManager::UnregisterSound(Core::AmbientSoundId id)
{
    if (m_sounds.erase(id) == 0) {
        return std::unexpected(AmbientSoundError::NotFound);
    }
    return {};
}

std::expected<float, AmbientSoundError> AreaAmbientSoundManager::CalculateAttenuatedVolume(
    Core::AmbientSoundId id,
    const Core::MapCoords& viewerPosition) const
{
    auto it = m_sounds.find(id);
    if (it == m_sounds.end()) {
        return std::unexpected(AmbientSoundError::NotFound);
    }

    const auto& record = it->second;
    float distance = record.position.Distance(viewerPosition);
    
    if (distance >= record.maxRadius) {
        return 0.0f;
    }

    // Linear attenuation
    float attenuation = 1.0f - (distance / record.maxRadius);
    return record.baseVolume * attenuation;
}

std::vector<Core::AmbientSoundId> AreaAmbientSoundManager::GetActiveSoundsInRange(
    const Core::MapCoords& viewerPosition) const
{
    std::vector<Core::AmbientSoundId> activeSounds;
    for (const auto& [id, record] : m_sounds) {
        if (record.position.Distance(viewerPosition) < record.maxRadius) {
            activeSounds.push_back(id);
        }
    }
    return activeSounds;
}

std::expected<AmbientSoundRecord, AmbientSoundError> AreaAmbientSoundManager::GetSoundRecord(Core::AmbientSoundId id) const
{
    auto it = m_sounds.find(id);
    if (it == m_sounds.end()) {
        return std::unexpected(AmbientSoundError::NotFound);
    }
    return it->second;
}

} // namespace Client::World
