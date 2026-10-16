#include "SkillTableRegistry.h"

namespace Client::Skills {

void SkillTableRegistry::Initialize(std::span<const SkillTableRecord> records) {
    m_skills.clear();
    m_skills.reserve(records.size());
    for (const auto& record : records) {
        m_skills[record.dwVnum] = record;
    }
}

std::optional<SkillTableRecord> SkillTableRegistry::GetSkill(Client::Core::SkillId vnum) const {
    auto it = m_skills.find(vnum);
    if (it != m_skills.end()) {
        return it->second;
    }
    return std::nullopt;
}

std::optional<SkillType> SkillTableRegistry::GetSkillType(Client::Core::SkillId vnum) const {
    auto it = m_skills.find(vnum);
    if (it != m_skills.end()) {
        if (it->second.bType >= static_cast<uint8_t>(SkillType::Active) && 
            it->second.bType <= static_cast<uint8_t>(SkillType::Horse)) {
            return static_cast<SkillType>(it->second.bType);
        }
        return SkillType::None;
    }
    return std::nullopt;
}

} // namespace Client::Skills
