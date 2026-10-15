#pragma once

#include <cstdint>
#include <string_view>
#include <format>
#include <utility>
#include "Client/Core/Result.h"

namespace Client::Player {

/**
 * @brief Enum błędów operacji na statystykach gracza.
 */
enum class PlayerStatError : uint8_t {
    None = 0,
    NotEnoughPoints,
    StatMaxedOut,
    NotEnoughSP
};

/**
 * @brief Konwersja błędu na tekst.
 */
[[nodiscard]] constexpr std::string_view to_string(PlayerStatError error) noexcept {
    switch (error) {
        case PlayerStatError::None: return "PlayerStatError::None";
        case PlayerStatError::NotEnoughPoints: return "PlayerStatError::NotEnoughPoints - Brak punktów do rozdania";
        case PlayerStatError::StatMaxedOut: return "PlayerStatError::StatMaxedOut - Statystyka osiągnęła maksymalny poziom";
        case PlayerStatError::NotEnoughSP: return "PlayerStatError::NotEnoughSP - Niewystarczająca ilość punktów magii (SP)";
        default: return "PlayerStatError::Unknown";
    }
}

/**
 * @brief Typy statystyk podstawowych.
 */
enum class StatType : uint8_t {
    VIT,
    INT,
    STR,
    DEX
};

/**
 * @brief Kontroler statystyk gracza (HP, SP, VIT, INT, STR, DEX).
 *
 * Czysta architektura C++23. Brak zależności od Windows API.
 * Brak dynamicznych alokacji. Stan całkowicie wewnątrz klasy.
 */
class PlayerStatsController {
public:
    static constexpr uint32_t MAX_STAT_VALUE = 90;
    static constexpr uint32_t HP_PER_VIT = 40;
    static constexpr uint32_t SP_PER_INT = 20;

    constexpr PlayerStatsController() noexcept = default;

    /**
     * @brief Inicjuje podstawowe statystyki.
     */
    constexpr void Initialize(uint32_t vit, uint32_t intel, uint32_t str, uint32_t dex) noexcept {
        m_vit = vit;
        m_int = intel;
        m_str = str;
        m_dex = dex;
        
        m_maxHP = 100 + (m_vit * HP_PER_VIT);
        m_maxSP = 50 + (m_int * SP_PER_INT);
        
        m_hp = m_maxHP;
        m_sp = m_maxSP;
    }

    constexpr void SetAvailablePoints(uint32_t points) noexcept {
        m_availablePoints = points;
    }

    constexpr Core::Result<void, PlayerStatError> AllocateStatPoint(StatType stat) noexcept {
        if (m_availablePoints == 0) {
            return std::unexpected(PlayerStatError::NotEnoughPoints);
        }

        uint32_t* targetStat = nullptr;
        switch (stat) {
            case StatType::VIT: targetStat = &m_vit; break;
            case StatType::INT: targetStat = &m_int; break;
            case StatType::STR: targetStat = &m_str; break;
            case StatType::DEX: targetStat = &m_dex; break;
            default: std::unreachable();
        }

        if (*targetStat >= MAX_STAT_VALUE) {
            return std::unexpected(PlayerStatError::StatMaxedOut);
        }

        (*targetStat)++;
        m_availablePoints--;

        // Aktualizacja HP/SP przy podniesieniu odpowiednich statystyk
        if (stat == StatType::VIT) {
            m_maxHP += HP_PER_VIT;
            m_hp += HP_PER_VIT; // Dodajemy to samo hp, by utrzymać różnicę lub uzdrowić? Załóżmy uzdrowienie o przydział
        } else if (stat == StatType::INT) {
            m_maxSP += SP_PER_INT;
            m_sp += SP_PER_INT;
        }

        return {};
    }

    constexpr void TakeDamage(uint32_t amount) noexcept {
        if (amount >= m_hp) {
            m_hp = 0;
        } else {
            m_hp -= amount;
        }
    }

    constexpr void HealHP(uint32_t amount) noexcept {
        if (m_hp == 0) return; // Dead
        m_hp += amount;
        if (m_hp > m_maxHP) {
            m_hp = m_maxHP;
        }
    }

    constexpr Core::Result<void, PlayerStatError> ConsumeSP(uint32_t amount) noexcept {
        if (amount > m_sp) {
            return std::unexpected(PlayerStatError::NotEnoughSP);
        }
        m_sp -= amount;
        return {};
    }

    constexpr void RestoreSP(uint32_t amount) noexcept {
        if (m_hp == 0) return; // Dead
        m_sp += amount;
        if (m_sp > m_maxSP) {
            m_sp = m_maxSP;
        }
    }

    [[nodiscard]] constexpr uint32_t GetHP() const noexcept { return m_hp; }
    [[nodiscard]] constexpr uint32_t GetMaxHP() const noexcept { return m_maxHP; }
    [[nodiscard]] constexpr uint32_t GetSP() const noexcept { return m_sp; }
    [[nodiscard]] constexpr uint32_t GetMaxSP() const noexcept { return m_maxSP; }
    
    [[nodiscard]] constexpr uint32_t GetStat(StatType stat) const noexcept {
        switch (stat) {
            case StatType::VIT: return m_vit;
            case StatType::INT: return m_int;
            case StatType::STR: return m_str;
            case StatType::DEX: return m_dex;
        }
        return 0;
    }

    [[nodiscard]] constexpr uint32_t GetAvailablePoints() const noexcept { return m_availablePoints; }

private:
    uint32_t m_hp{0};
    uint32_t m_maxHP{0};
    uint32_t m_sp{0};
    uint32_t m_maxSP{0};

    uint32_t m_vit{0};
    uint32_t m_int{0};
    uint32_t m_str{0};
    uint32_t m_dex{0};

    uint32_t m_availablePoints{0};
};

} // namespace Client::Player

template <>
struct std::formatter<Client::Player::PlayerStatError> : std::formatter<std::string_view> {
    auto format(Client::Player::PlayerStatError err, std::format_context& ctx) const {
        return std::formatter<std::string_view>::format(Client::Player::to_string(err), ctx);
    }
};
