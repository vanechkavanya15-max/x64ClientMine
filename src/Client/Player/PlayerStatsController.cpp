#include "PlayerStatsController.h"

namespace Client::Player {

constexpr uint32_t POINT_LEVEL = 1;
constexpr uint32_t POINT_EXP = 3;
constexpr uint32_t POINT_HP = 5;
constexpr uint32_t POINT_MAX_HP = 6;
constexpr uint32_t POINT_SP = 7;
constexpr uint32_t POINT_MAX_SP = 8;
constexpr uint32_t POINT_GOLD = 11;

PlayerStatsController::PlayerStatsController(Client::Gameplay::PlayerStatsDomain* domain) noexcept
    : m_domain(domain) {
}

void PlayerStatsController::SetDomain(Client::Gameplay::PlayerStatsDomain* domain) noexcept {
    m_domain = domain;
}

EterBase::Result<void, PlayerStatsError> PlayerStatsController::UpdateHP(int64_t hp) const {
    if (!m_domain) {
        return EterBase::MakeError(PlayerStatsError::DomainNotAttached);
    }
    m_domain->SetPoint(POINT_HP, hp);
    return {};
}

EterBase::Result<void, PlayerStatsError> PlayerStatsController::UpdateMaxHP(int64_t maxHp) const {
    if (!m_domain) {
        return EterBase::MakeError(PlayerStatsError::DomainNotAttached);
    }
    m_domain->SetPoint(POINT_MAX_HP, maxHp);
    return {};
}

EterBase::Result<void, PlayerStatsError> PlayerStatsController::UpdateSP(int64_t sp) const {
    if (!m_domain) {
        return EterBase::MakeError(PlayerStatsError::DomainNotAttached);
    }
    m_domain->SetPoint(POINT_SP, sp);
    return {};
}

EterBase::Result<void, PlayerStatsError> PlayerStatsController::UpdateMaxSP(int64_t maxSp) const {
    if (!m_domain) {
        return EterBase::MakeError(PlayerStatsError::DomainNotAttached);
    }
    m_domain->SetPoint(POINT_MAX_SP, maxSp);
    return {};
}

EterBase::Result<void, PlayerStatsError> PlayerStatsController::AddExperience(uint64_t expAmount) const {
    if (!m_domain) {
        return EterBase::MakeError(PlayerStatsError::DomainNotAttached);
    }
    uint64_t currentExp = m_domain->GetExp();
    m_domain->SetPoint(POINT_EXP, static_cast<int64_t>(currentExp + expAmount));
    return {};
}

EterBase::Result<void, PlayerStatsError> PlayerStatsController::SetLevel(uint32_t level) const {
    if (!m_domain) {
        return EterBase::MakeError(PlayerStatsError::DomainNotAttached);
    }
    if (level == 0) {
        return EterBase::MakeError(PlayerStatsError::InvalidLevel);
    }
    m_domain->SetPoint(POINT_LEVEL, static_cast<int64_t>(level));
    return {};
}

EterBase::Result<void, PlayerStatsError> PlayerStatsController::SetStatusPoint(uint32_t stIndex, uint16_t value) const {
    if (!m_domain) {
        return EterBase::MakeError(PlayerStatsError::DomainNotAttached);
    }
    if (stIndex >= 255) {
        return EterBase::MakeError(PlayerStatsError::InvalidStatPoint);
    }
    m_domain->SetStatusPoint(stIndex, value);
    return {};
}

EterBase::Result<void, PlayerStatsError> PlayerStatsController::SetGold(int64_t gold) const {
    if (!m_domain) {
        return EterBase::MakeError(PlayerStatsError::DomainNotAttached);
    }
    m_domain->SetPoint(POINT_GOLD, gold);
    return {};
}

} // namespace Client::Player
