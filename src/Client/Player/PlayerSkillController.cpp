#include "PlayerSkillController.h"

namespace Client::Player {

    PlayerSkillController::PlayerSkillController(Client::Gameplay::SkillDomain& skillDomain)
        : m_skillDomain(skillDomain)
    {
    }

    Client::Core::Result<std::vector<uint8_t>, CommandError> PlayerSkillController::UseSkill(uint32_t skillVnum, uint32_t targetVid)
    {
        // Maksymalna wartosc many uzywana z braku zaleznosci statystyk w celu walidacji
        constexpr uint32_t MAX_SP = 0xFFFFFFFF;
        
        auto canUseResult = m_skillDomain.CanUseSkill(skillVnum, MAX_SP);
        
        if (!canUseResult) {
            return std::unexpected(MapSkillError(canUseResult.error()));
        }

        return Client::Network::SkillPacketCodec::EncodeUseSkill(skillVnum, targetVid);
    }

    CommandError PlayerSkillController::MapSkillError(Client::Core::SkillError err) const noexcept
    {
        switch (err) {
            case Client::Core::SkillError::SkillNotFound: return CommandError::SkillNotFound;
            case Client::Core::SkillError::NotEnoughSP: return CommandError::NotEnoughSP;
            case Client::Core::SkillError::OnCooldown: return CommandError::OnCooldown;
            case Client::Core::SkillError::RequirementNotMet: return CommandError::RequirementNotMet;
            default: return CommandError::None;
        }
    }

} // namespace Client::Player
