#pragma once

#include <cstdint>
#include <optional>
#include "../Core/Result.h"
#include "../Core/DomainErrors.h"
#include "../Gameplay/CombatDomain.h"

namespace Client::Skills {

    struct SkillRequirementContext {
        uint32_t currentHP{0};
        uint32_t currentSP{0};
        uint8_t currentLevel{0};
        std::optional<Client::Gameplay::WeaponType> currentWeapon{std::nullopt};
    };

    struct SkillRequirements {
        uint8_t minLevel{0};
        uint32_t hpCost{0};
        uint32_t spCost{0};
        bool requiresWeapon{false};
        uint32_t allowedWeaponsMask{0xFFFFFFFF}; // Bitmask of Client::Gameplay::WeaponType
    };

    class SkillRequirementValidator {
    public:
        [[nodiscard]] static constexpr Core::Result<void, Core::SkillError> Validate(
            const SkillRequirementContext& context,
            const SkillRequirements& reqs) noexcept 
        {
            if (context.currentLevel < reqs.minLevel) {
                return std::unexpected(Core::SkillError::RequirementNotMet);
            }

            if (context.currentHP <= reqs.hpCost) {
                return std::unexpected(Core::SkillError::RequirementNotMet);
            }

            if (context.currentSP < reqs.spCost) {
                return std::unexpected(Core::SkillError::NotEnoughSP);
            }

            if (reqs.requiresWeapon) {
                if (!context.currentWeapon.has_value()) {
                    return std::unexpected(Core::SkillError::RequirementNotMet);
                }

                const uint32_t weaponBit = 1u << static_cast<uint32_t>(*context.currentWeapon);
                if ((reqs.allowedWeaponsMask & weaponBit) == 0) {
                    return std::unexpected(Core::SkillError::RequirementNotMet);
                }
            }

            return {};
        }
    };

} // namespace Client::Skills
