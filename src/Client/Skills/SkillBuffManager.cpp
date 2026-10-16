#include "SkillBuffManager.h"
#include <vector>

namespace Client::Skills {

Client::Core::Result<void, Client::Core::SkillError> SkillBuffManager::AddBuff(Client::Core::SkillId skillId, std::chrono::milliseconds duration) {
    if (!static_cast<bool>(skillId)) {
        return std::unexpected(Client::Core::SkillError::SkillNotFound);
    }
    
    // Add or update the duration
    m_activeBuffs[skillId] = duration;
    
    return {};
}

Client::Core::Result<void, Client::Core::SkillError> SkillBuffManager::RemoveBuff(Client::Core::SkillId skillId) {
    if (!static_cast<bool>(skillId)) {
        return std::unexpected(Client::Core::SkillError::SkillNotFound);
    }
    
    auto it = m_activeBuffs.find(skillId);
    if (it == m_activeBuffs.end()) {
        return std::unexpected(Client::Core::SkillError::SkillNotFound);
    }
    
    m_activeBuffs.erase(it);
    return {};
}

void SkillBuffManager::Update(std::chrono::milliseconds deltaTime) {
    std::erase_if(m_activeBuffs, [deltaTime](auto& pair) {
        pair.second -= deltaTime;
        return pair.second <= std::chrono::milliseconds::zero();
    });
}

bool SkillBuffManager::HasBuff(Client::Core::SkillId skillId) const {
    return m_activeBuffs.contains(skillId);
}

std::optional<std::chrono::milliseconds> SkillBuffManager::GetBuffRemainingTime(Client::Core::SkillId skillId) const {
    auto it = m_activeBuffs.find(skillId);
    if (it != m_activeBuffs.end()) {
        return it->second;
    }
    return std::nullopt;
}

void SkillBuffManager::ClearAll() {
    m_activeBuffs.clear();
}

} // namespace Client::Skills
