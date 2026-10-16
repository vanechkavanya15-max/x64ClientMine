#include "Client/Player/PlayerEffectRegistryBridge.h"

namespace Client::Player {

EterBase::Result<void, EffectError> PlayerEffectRegistryBridge::RegisterEffect(
    EffectId id, 
    const std::string& path, 
    bool isLooping, 
    float durationSeconds, 
    float defaultScale) 
{
    if (!id) {
        return EterBase::MakeError(EffectError::InvalidId);
    }
    
    if (m_templates.contains(id)) {
        return EterBase::MakeError(EffectError::AlreadyRegistered);
    }
    
    m_templates.emplace(id, EffectTemplate{
        .id = id,
        .filePath = path,
        .isLooping = isLooping,
        .durationSeconds = durationSeconds,
        .defaultScale = defaultScale
    });
    
    return {};
}

EterBase::Result<EffectTemplate, EffectError> PlayerEffectRegistryBridge::GetEffectTemplate(EffectId id) const {
    if (auto it = m_templates.find(id); it != m_templates.end()) {
        return it->second;
    }
    return EterBase::MakeError(EffectError::NotFound);
}

EterBase::Result<EffectInstanceId, EffectError> PlayerEffectRegistryBridge::ShowEffect(
    Core::EntityVid targetVid, 
    EffectId effectId, 
    EffectAttachmentPoint attachmentPoint, 
    float customScale) 
{
    if (!targetVid) {
        return EterBase::MakeError(EffectError::InvalidTarget);
    }
    
    auto templateResult = GetEffectTemplate(effectId);
    if (!templateResult) {
        return EterBase::MakeError(templateResult.error());
    }
    
    const auto& effectTemplate = templateResult.value();
    
    EffectInstanceId newInstanceId{m_nextInstanceId++};
    
    float initialScale = customScale > 0.0f ? customScale : effectTemplate.defaultScale;
    
    m_instances.emplace(newInstanceId, EffectInstance{
        .instanceId = newInstanceId,
        .effectId = effectId,
        .targetVid = targetVid,
        .attachmentPoint = attachmentPoint,
        .state = EffectState::Playing,
        .playbackTime = 0.0f,
        .duration = effectTemplate.durationSeconds,
        .isLooping = effectTemplate.isLooping,
        .scale = initialScale
    });
    
    return newInstanceId;
}

EterBase::Result<void, EffectError> PlayerEffectRegistryBridge::StopEffect(EffectInstanceId instanceId) {
    auto it = m_instances.find(instanceId);
    if (it == m_instances.end()) {
        return EterBase::MakeError(EffectError::InstanceNotFound);
    }
    
    it->second.state = EffectState::Stopped;
    return {};
}

EterBase::Result<void, EffectError> PlayerEffectRegistryBridge::StopAllEffectsOnTarget(Core::EntityVid targetVid) {
    if (!targetVid) {
        return EterBase::MakeError(EffectError::InvalidTarget);
    }
    
    bool found = false;
    for (auto& [id, instance] : m_instances) {
        if (instance.targetVid == targetVid && instance.state != EffectState::Stopped) {
            instance.state = EffectState::Stopped;
            found = true;
        }
    }
    
    if (!found) {
        return EterBase::MakeError(EffectError::InstanceNotFound);
    }
    
    return {};
}

void PlayerEffectRegistryBridge::UpdateEffects(float deltaTimeSeconds) {
    if (deltaTimeSeconds <= 0.0f) {
        return;
    }
    
    // Uzycie std::erase_if wymaga C++20, bezpieczne usuwanie zatrzymanych efektow
    std::erase_if(m_instances, [deltaTimeSeconds](auto& pair) {
        auto& instance = pair.second;
        
        if (instance.state == EffectState::Stopped) {
            return true;
        }
        
        if (instance.state == EffectState::Playing) {
            instance.playbackTime += deltaTimeSeconds;
            
            if (!instance.isLooping && instance.duration > 0.0f && instance.playbackTime >= instance.duration) {
                instance.state = EffectState::Stopped;
                return true;
            }
        }
        
        return false;
    });
}

std::vector<EffectInstance> PlayerEffectRegistryBridge::GetActiveEffects() const {
    std::vector<EffectInstance> active;
    active.reserve(m_instances.size());
    
    for (const auto& [id, instance] : m_instances) {
        if (instance.state == EffectState::Playing) {
            active.push_back(instance);
        }
    }
    
    return active;
}

std::vector<EffectInstance> PlayerEffectRegistryBridge::GetEffectsOnTarget(Core::EntityVid targetVid) const {
    std::vector<EffectInstance> onTarget;
    
    for (const auto& [id, instance] : m_instances) {
        if (instance.targetVid == targetVid && instance.state == EffectState::Playing) {
            onTarget.push_back(instance);
        }
    }
    
    return onTarget;
}

EterBase::Result<EffectInstance, EffectError> PlayerEffectRegistryBridge::GetEffectInstance(EffectInstanceId instanceId) const {
    if (auto it = m_instances.find(instanceId); it != m_instances.end()) {
        return it->second;
    }
    return EterBase::MakeError(EffectError::InstanceNotFound);
}

} // namespace Client::Player