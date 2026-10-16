#pragma once

#include "Client/Core/StrongTypes.h"
#include <expected>
#include <vector>
#include <unordered_map>
#include <string>
#include <cmath>

namespace Client::Core {
struct AmbientSoundIdTag {};
using AmbientSoundId = StrongType<AmbientSoundIdTag, uint32_t, 0>;
} // namespace Client::Core

namespace Client::World {

enum class AmbientSoundError {
    None = 0,
    InvalidId,
    AlreadyExists,
    NotFound,
    InvalidVolume,
    InvalidRadius
};

struct AmbientSoundRecord {
    Core::AmbientSoundId id;
    std::string filePath;
    Core::MapCoords position;
    float maxRadius;
    float baseVolume;
};

class AreaAmbientSoundManager {
public:
    AreaAmbientSoundManager() = default;
    ~AreaAmbientSoundManager() = default;

    std::expected<void, AmbientSoundError> RegisterSound(
        Core::AmbientSoundId id,
        const std::string& filePath,
        const Core::MapCoords& position,
        float maxRadius,
        float baseVolume);

    std::expected<void, AmbientSoundError> UnregisterSound(Core::AmbientSoundId id);

    [[nodiscard]] std::expected<float, AmbientSoundError> CalculateAttenuatedVolume(
        Core::AmbientSoundId id,
        const Core::MapCoords& viewerPosition) const;

    [[nodiscard]] std::vector<Core::AmbientSoundId> GetActiveSoundsInRange(
        const Core::MapCoords& viewerPosition) const;
        
    [[nodiscard]] std::expected<AmbientSoundRecord, AmbientSoundError> GetSoundRecord(Core::AmbientSoundId id) const;

private:
    std::unordered_map<Core::AmbientSoundId, AmbientSoundRecord> m_sounds;
};

} // namespace Client::World
