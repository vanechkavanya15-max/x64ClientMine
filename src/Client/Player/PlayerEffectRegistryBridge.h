#pragma once

#include <cstdint>
#include <string>
#include <string_view>
#include <unordered_map>
#include <vector>
#include <expected>
#include <algorithm>

#include "Client/Core/StrongTypes.h"
#include "EterBase/Result.h"

namespace Client::Player {

struct EffectIdTag {};
using EffectId = Core::StrongType<EffectIdTag, uint32_t, 0>;

enum class EffectError : uint8_t {
    None = 0,
    InvalidId,
    AlreadyRegistered,
    NotFound,
    InvalidTarget,
    InstanceNotFound,
    InvalidState
};

[[nodiscard]] constexpr std::string_view ToString(EffectError err) noexcept {
    switch (err) {
        case EffectError::None: return "None";
        case EffectError::InvalidId: return "InvalidId";
        case EffectError::AlreadyRegistered: return "AlreadyRegistered";
        case EffectError::NotFound: return "NotFound";
        case EffectError::InvalidTarget: return "InvalidTarget";
        case EffectError::InstanceNotFound: return "InstanceNotFound";
        case EffectError::InvalidState: return "InvalidState";
    }
    return "UnknownEffectError";
}

enum class EffectState : uint8_t {
    Initializing,
    Playing,
    Stopped
};

enum class EffectAttachmentPoint : uint8_t {
    Root,
    Head,
    WeaponRight,
    WeaponLeft,
    Body,
    FootRight,
    FootLeft
};

struct EffectTemplate {
    EffectId id;
    std::string filePath;
    bool isLooping = false;
    float durationSeconds = 0.0f;
    float defaultScale = 1.0f;
};

struct EffectInstanceIdTag {};
using EffectInstanceId = Core::StrongType<EffectInstanceIdTag, uint32_t, 0>;

struct EffectInstance {
    EffectInstanceId instanceId;
    EffectId effectId;
    Core::EntityVid targetVid;
    EffectAttachmentPoint attachmentPoint;
    EffectState state = EffectState::Initializing;
    
    float playbackTime = 0.0f;
    float duration = 0.0f;
    bool isLooping = false;
    float scale = 1.0f;
};

class PlayerEffectRegistryBridge {
public:
    PlayerEffectRegistryBridge() = default;
    ~PlayerEffectRegistryBridge() = default;

    EterBase::Result<void, EffectError> RegisterEffect(EffectId id, const std::string& path, bool isLooping = false, float durationSeconds = 0.0f, float defaultScale = 1.0f);
    EterBase::Result<EffectTemplate, EffectError> GetEffectTemplate(EffectId id) const;

    EterBase::Result<EffectInstanceId, EffectError> ShowEffect(Core::EntityVid targetVid, EffectId effectId, EffectAttachmentPoint attachmentPoint = EffectAttachmentPoint::Root, float customScale = 0.0f);
    EterBase::Result<void, EffectError> StopEffect(EffectInstanceId instanceId);
    EterBase::Result<void, EffectError> StopAllEffectsOnTarget(Core::EntityVid targetVid);

    void UpdateEffects(float deltaTimeSeconds);

    [[nodiscard]] std::vector<EffectInstance> GetActiveEffects() const;
    [[nodiscard]] std::vector<EffectInstance> GetEffectsOnTarget(Core::EntityVid targetVid) const;

    EterBase::Result<EffectInstance, EffectError> GetEffectInstance(EffectInstanceId instanceId) const;

private:
    std::unordered_map<EffectId, EffectTemplate> m_templates;
    std::unordered_map<EffectInstanceId, EffectInstance> m_instances;
    
    uint32_t m_nextInstanceId = 1;
};

} // namespace Client::Player
