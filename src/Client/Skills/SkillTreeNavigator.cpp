#include "SkillTreeNavigator.h"

namespace Client::Skills {

SkillTreeNavigator::SkillTreeNavigator(std::span<const SkillTableRecord> skills) noexcept
    : skills_(skills) {}

std::optional<SkillTableRecord> SkillTreeNavigator::GetSkillRecord(Client::Core::SkillId skillId) const noexcept {
    for (const auto& skill : skills_) {
        if (skill.dwVnum == skillId) {
            return skill;
        }
    }
    return std::nullopt;
}

std::optional<SkillTableRecord> SkillTreeNavigator::GetPrerequisiteSkill(Client::Core::SkillId skillId) const noexcept {
    auto recordOpt = GetSkillRecord(skillId);
    if (!recordOpt.has_value()) {
        return std::nullopt;
    }

    if (static_cast<uint32_t>(recordOpt->dwPrerequisiteSkillVnum.get()) != 0) {
        return GetSkillRecord(recordOpt->dwPrerequisiteSkillVnum);
    }
    return std::nullopt;
}

Client::Core::Result<void, Client::Core::SkillError> SkillTreeNavigator::CanLearnSkill(
    Client::Core::SkillId skillId, 
    const SkillTreeContext& context) const noexcept {

    auto recordOpt = GetSkillRecord(skillId);
    if (!recordOpt.has_value()) {
        return std::unexpected(Client::Core::SkillError::SkillNotFound);
    }

    const auto& record = recordOpt.value();

    if (context.availableSkillPoints == 0) {
        return std::unexpected(Client::Core::SkillError::RequirementNotMet);
    }

    if (context.actorLevel < record.bLevelLimit) {
        return std::unexpected(Client::Core::SkillError::RequirementNotMet);
    }

    if (context.currentSkillLevel >= record.bMaxLevel) {
        return std::unexpected(Client::Core::SkillError::RequirementNotMet);
    }

    if (static_cast<uint32_t>(record.dwPrerequisiteSkillVnum.get()) != 0 && record.bPrerequisiteSkillLevel > 0) {
        if (!context.prerequisiteSkillCurrentLevel.has_value() || context.prerequisiteSkillCurrentLevel.value() < record.bPrerequisiteSkillLevel) {
            return std::unexpected(Client::Core::SkillError::RequirementNotMet);
        }
    }

    return {};
}

} // namespace Client::Skills
