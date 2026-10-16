#include "PlayerSkillController.h"

namespace Client::Player {

    PlayerSkillController::PlayerSkillController(Client::Gameplay::SkillDomain& skillDomain, Client::Gameplay::IPlayerStatsService& statsService)
        : m_skillDomain(skillDomain), m_statsService(statsService)
    {
    }

    Client::Core::Result<std::vector<uint8_t>, CommandError> PlayerSkillController::UseSkill(uint32_t skillVnum, uint32_t targetVid)
    {
        constexpr uint32_t POINT_SP = 7;
        uint32_t currentSP = static_cast<uint32_t>(m_statsService.GetPoint(POINT_SP));
        
        auto canUseResult = m_skillDomain.CanUseSkill(skillVnum, currentSP);
        
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
