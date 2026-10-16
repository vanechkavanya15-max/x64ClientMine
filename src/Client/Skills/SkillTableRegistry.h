#pragma once
#include <cstdint>
#include <string>
#include <unordered_map>
#include <optional>

namespace Client::Skills {

struct SkillInfo {
    uint32_t vnum{0};
    std::string name{};
    uint8_t type{0};
    uint8_t maxLevel{40};
    uint32_t spCost{0};
};

class SkillTableRegistry {
public:
    SkillTableRegistry() = default;
    ~SkillTableRegistry() = default;

    bool RegisterSkill(uint32_t vnum, const std::string& name, uint8_t type, uint32_t spCost);
    [[nodiscard]] std::optional<SkillInfo> GetSkill(uint32_t vnum) const;
    [[nodiscard]] size_t Count() const noexcept { return m_skills.size(); }

private:
    std::unordered_map<uint32_t, SkillInfo> m_skills;
};

} // namespace Client::Skills
