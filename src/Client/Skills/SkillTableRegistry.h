#pragma once
#include <cstdint>
#include <string>
#include <unordered_map>
#include <optional>
#include <span>

#include "Client/Skills/SkillTableLoader.h"
#include "Client/Core/StrongTypes.h"

namespace Client::Skills {

enum class SkillType : uint8_t {
    None = 0,
    Active = 1,
    Passive = 2,
    Guild = 3,
    Horse = 4
};

class SkillTableRegistry {
public:
    SkillTableRegistry() = default;
    ~SkillTableRegistry() = default;

    void Initialize(std::span<const SkillTableRecord> records);

    [[nodiscard]] std::optional<SkillTableRecord> GetSkill(Client::Core::SkillId vnum) const;
    [[nodiscard]] std::optional<SkillType> GetSkillType(Client::Core::SkillId vnum) const;
    
    [[nodiscard]] size_t Count() const noexcept { return m_skills.size(); }

private:
    std::unordered_map<Client::Core::SkillId, SkillTableRecord> m_skills;
};

} // namespace Client::Skills
