#pragma once

#include "src/Client/Contracts/IPlayerDomainContract.h"
#include "src/Client/Core/ExpectedResult.h"
#include "src/Client/Core/StrongTypes.h"
#include <vector>
#include <cstdint>

namespace Client::Combat
{
    enum class CombatTargetingError : uint8_t
    {
        None = 0,
        TargetNotFound,
        TargetIsDead,
        TargetOutOfRange,
        AllocationFailed,
        InvalidPlayerState
    };

    struct CombatTarget
    {
        uint32_t vid{0};
        float distance{0.0f};
    };

    class CombatTargetingLogic
    {
    public:
        explicit CombatTargetingLogic(const Contracts::IPlayerDomainContract& playerContract) noexcept;
        ~CombatTargetingLogic() = default;

        [[nodiscard]] Core::ExpectedResult<uint32_t, CombatTargetingError> GetCurrentTarget() const noexcept;
        [[nodiscard]] Core::ExpectedResult<CombatTarget, CombatTargetingError> ValidateTarget(uint32_t targetVid, float maxDistance) const noexcept;
        [[nodiscard]] Core::ExpectedResult<std::vector<uint32_t>, CombatTargetingError> GetTargetsInRange(const std::vector<uint32_t>& potentialTargets, float maxDistance) const noexcept;

    private:
        const Contracts::IPlayerDomainContract& m_playerContract;
    };
}
