#pragma once

#include <optional>
#include <span>
#include "Client/Core/StrongTypes.h"
#include "Client/Core/DomainErrors.h"
#include "Client/Skills/SkillTableLoader.h"

namespace Client::Skills {

struct SkillTreeContext {
    uint32_t actorLevel{1};
    uint32_t availableSkillPoints{0};
    uint8_t currentSkillLevel{0};
    std::optional<uint8_t> prerequisiteSkillCurrentLevel{std::nullopt};
};

class SkillTreeNavigator {
public:
    explicit SkillTreeNavigator(std::span<const SkillTableRecord> skills) noexcept;

    [[nodiscard]] Client::Core::Result<void, Client::Core::SkillError> CanLearnSkill(
        Client::Core::SkillId skillId, 
        const SkillTreeContext& context) const noexcept;

    [[nodiscard]] std::optional<SkillTableRecord> GetSkillRecord(Client::Core::SkillId skillId) const noexcept;
    
    [[nodiscard]] std::optional<SkillTableRecord> GetPrerequisiteSkill(Client::Core::SkillId skillId) const noexcept;

private:
    std::span<const SkillTableRecord> skills_;
};

} // namespace Client::Skills
