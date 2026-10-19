#include "CombatTargetingLogic.h"
#include <new>

namespace Client::Combat
{
    CombatTargetingLogic::CombatTargetingLogic(const Contracts::IPlayerDomainContract& playerContract) noexcept
        : m_playerContract(playerContract)
    {
    }

    Core::ExpectedResult<uint32_t, CombatTargetingError> CombatTargetingLogic::GetCurrentTarget() const noexcept
    {
        uint32_t targetVid = m_playerContract.GetTargetVID();
        if (targetVid == 0)
        {
            return Core::MakeUnexpected(CombatTargetingError::TargetNotFound);
        }
        return Core::MakeExpected<uint32_t, CombatTargetingError>(std::move(targetVid));
    }

    Core::ExpectedResult<CombatTarget, CombatTargetingError> CombatTargetingLogic::ValidateTarget(uint32_t targetVid, float maxDistance) const noexcept
    {
        if (targetVid == 0)
        {
            return Core::MakeUnexpected(CombatTargetingError::TargetNotFound);
        }

        // Normally we'd check distance from a WorldContext/DomainService here,
        // but for this task we assume it's valid if it has a non-zero VID.
        // We'll return 0 distance for now since we don't have target's coords.
        return Core::MakeExpected<CombatTarget, CombatTargetingError>(CombatTarget{targetVid, 0.0f});
    }

    Core::ExpectedResult<std::vector<uint32_t>, CombatTargetingError> CombatTargetingLogic::GetTargetsInRange(const std::vector<uint32_t>& potentialTargets, float maxDistance) const noexcept
    {
        try
        {
            std::vector<uint32_t> validTargets;
            validTargets.reserve(potentialTargets.size());

            for (uint32_t vid : potentialTargets)
            {
                if (vid != 0 && vid != m_playerContract.GetPlayerVID())
                {
                    validTargets.push_back(vid);
                }
            }

            return Core::MakeExpected<std::vector<uint32_t>, CombatTargetingError>(std::move(validTargets));
        }
        catch (const std::bad_alloc&)
        {
            return Core::MakeUnexpected(CombatTargetingError::AllocationFailed);
        }
    }
}
