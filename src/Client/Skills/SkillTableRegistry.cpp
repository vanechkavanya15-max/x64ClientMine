#include "SkillTableRegistry.h"

namespace Client::Skills {

bool SkillTableRegistry::RegisterSkill(uint32_t vnum, const std::string& name, uint8_t type, uint32_t spCost) {
    m_skills[vnum] = SkillInfo{vnum, name, type, 40, spCost};
    return true;
}

std::optional<SkillInfo> SkillTableRegistry::GetSkill(uint32_t vnum) const {
    auto it = m_skills.find(vnum);
    if (it != m_skills.end()) return it->second;
    return std::nullopt;
}

} // namespace Client::Skills
