#pragma once

#include "../../EterBase/Result.h"
#include <format>
#include "../Gameplay/PlayerStatsDomain.h"

namespace Client::Player {

enum class PlayerStatsError : uint8_t {
    None = 0,
    DomainNotAttached,
    InvalidLevel,
    InvalidStatPoint
};

[[nodiscard]] constexpr std::string_view ToString(PlayerStatsError err) noexcept {
    switch (err) {
        case PlayerStatsError::None: return "None";
        case PlayerStatsError::DomainNotAttached: return "DomainNotAttached";
        case PlayerStatsError::InvalidLevel: return "InvalidLevel";
        case PlayerStatsError::InvalidStatPoint: return "InvalidStatPoint";
    }
    return "UnknownPlayerStatsError";
}

} // namespace Client::Player

template <>
struct std::formatter<Client::Player::PlayerStatsError> : std::formatter<std::string_view> {
    auto format(Client::Player::PlayerStatsError err, std::format_context& ctx) const {
        return std::formatter<std::string_view>::format(Client::Player::ToString(err), ctx);
    }
};

namespace Client::Player {

/**
 * @brief Kontroler zarzadzania statystykami gracza.
 * Realizuje warstwe posrednia i weryfikacje przed aktualizacja `PlayerStatsDomain`.
 */
class PlayerStatsController {
public:
    explicit PlayerStatsController(Client::Gameplay::PlayerStatsDomain* domain = nullptr) noexcept;
    ~PlayerStatsController() = default;

    void SetDomain(Client::Gameplay::PlayerStatsDomain* domain) noexcept;

    // HP & SP
    EterBase::Result<void, PlayerStatsError> UpdateHP(int64_t hp) const;
    EterBase::Result<void, PlayerStatsError> UpdateMaxHP(int64_t maxHp) const;
    EterBase::Result<void, PlayerStatsError> UpdateSP(int64_t sp) const;
    EterBase::Result<void, PlayerStatsError> UpdateMaxSP(int64_t maxSp) const;

    // Experience & Level
    EterBase::Result<void, PlayerStatsError> AddExperience(uint64_t expAmount) const;
    EterBase::Result<void, PlayerStatsError> SetLevel(uint32_t level) const;

    // Status Points
    EterBase::Result<void, PlayerStatsError> SetStatusPoint(uint32_t stIndex, uint16_t value) const;

    // Gold
    EterBase::Result<void, PlayerStatsError> SetGold(int64_t gold) const;

private:
    Client::Gameplay::PlayerStatsDomain* m_domain;
};

} // namespace Client::Player
