#pragma once

#include <expected>
#include <optional>
#include <string_view>
#include <cstdint>
#include "Client/Core/StrongTypes.h"
#include "EterBase/Result.h"

namespace Client::Skills {

enum class SkillValidationError : uint8_t {
    None = 0,
    NotEnoughSP,
    NotEnoughHP,
    LevelTooLow,
    OnCooldown,
    InvalidWeapon,
    InvalidTarget,
    SkillNotLearned
};

[[nodiscard]] constexpr std::string_view ToString(SkillValidationError err) noexcept {
    switch (err) {
        case SkillValidationError::None: return "None";
        case SkillValidationError::NotEnoughSP: return "NotEnoughSP";
        case SkillValidationError::NotEnoughHP: return "NotEnoughHP";
        case SkillValidationError::LevelTooLow: return "LevelTooLow";
        case SkillValidationError::OnCooldown: return "OnCooldown";
        case SkillValidationError::InvalidWeapon: return "InvalidWeapon";
        case SkillValidationError::InvalidTarget: return "InvalidTarget";
        case SkillValidationError::SkillNotLearned: return "SkillNotLearned";
    }
    return "UnknownSkillError";
}

template <typename T = void>
using SkillResult = std::expected<T, SkillValidationError>;

struct SkillCastContext {
    uint32_t currentSP{0};
    uint32_t currentHP{0};
    uint32_t actorLevel{1};
    bool isOnCooldown{false};
    std::optional<uint32_t> equippedWeaponSubtype{std::nullopt};
    bool hasTarget{false};
    bool isSkillLearned{false};
};

struct SkillRequirements {
    uint32_t requiredSP{0};
    uint32_t requiredHP{0};
    uint32_t minLevel{1};
    std::optional<uint32_t> requiredWeaponSubtype{std::nullopt};
    bool requiresTarget{false};
};

class SkillRequirementValidator {
public:
    [[nodiscard]] static constexpr SkillResult<void> Validate(const SkillCastContext& context, const SkillRequirements& reqs) noexcept {
        if (!context.isSkillLearned) {
            return std::unexpected(SkillValidationError::SkillNotLearned);
        }
        
        if (context.isOnCooldown) {
            return std::unexpected(SkillValidationError::OnCooldown);
        }

        if (context.actorLevel < reqs.minLevel) {
            return std::unexpected(SkillValidationError::LevelTooLow);
        }

        if (context.currentSP < reqs.requiredSP) {
            return std::unexpected(SkillValidationError::NotEnoughSP);
        }

        if (context.currentHP < reqs.requiredHP) {
            return std::unexpected(SkillValidationError::NotEnoughHP);
        }

        if (reqs.requiresTarget && !context.hasTarget) {
            return std::unexpected(SkillValidationError::InvalidTarget);
        }

        if (reqs.requiredWeaponSubtype.has_value()) {
            if (!context.equippedWeaponSubtype.has_value() || 
                context.equippedWeaponSubtype.value() != reqs.requiredWeaponSubtype.value()) {
                return std::unexpected(SkillValidationError::InvalidWeapon);
            }
        }

        return {};
    }
};

} // namespace Client::Skills
