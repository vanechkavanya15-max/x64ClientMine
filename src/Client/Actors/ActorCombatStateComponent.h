#pragma once

#include <cstdint>
#include <optional>
#include <algorithm>
#include "../Core/Result.h"
#include "../Core/StrongTypes.h"
#include "../Core/DomainErrors.h"

namespace Client::Actors {

struct CombatStateConfig {
    int32_t maxHp = 0;
    int32_t maxSp = 0;
};

class ActorCombatStateComponent {
public:
    constexpr explicit ActorCombatStateComponent(CombatStateConfig config) noexcept
        : m_config(config), m_hp(config.maxHp), m_sp(config.maxSp), m_isDead(m_hp <= 0), m_isInCombat(false) {}

    [[nodiscard]] constexpr int32_t GetHp() const noexcept { return m_hp; }
    [[nodiscard]] constexpr int32_t GetMaxHp() const noexcept { return m_config.maxHp; }
    [[nodiscard]] constexpr int32_t GetSp() const noexcept { return m_sp; }
    [[nodiscard]] constexpr int32_t GetMaxSp() const noexcept { return m_config.maxSp; }
    
    [[nodiscard]] constexpr bool IsDead() const noexcept { return m_isDead; }
    [[nodiscard]] constexpr bool IsInCombat() const noexcept { return m_isInCombat; }

    [[nodiscard]] constexpr std::optional<Core::EntityVid> GetTarget() const noexcept { return m_target; }

    constexpr void SetTarget(std::optional<Core::EntityVid> target) noexcept {
        m_target = target;
    }

    constexpr void SetInCombat(bool inCombat) noexcept {
        m_isInCombat = inCombat;
    }

    [[nodiscard]] constexpr Core::Result<void, Core::EntityError> TakeDamage(int32_t amount) noexcept {
        if (m_isDead) {
            return std::unexpected(Core::EntityError::Dead);
        }
        if (amount < 0) {
            return {};
        }

        m_hp -= amount;
        if (m_hp <= 0) {
            m_hp = 0;
            m_isDead = true;
            m_target.reset();
            m_isInCombat = false;
        }
        return {};
    }

    [[nodiscard]] constexpr Core::Result<void, Core::EntityError> Heal(int32_t amount) noexcept {
        if (m_isDead) {
            return std::unexpected(Core::EntityError::Dead);
        }
        if (amount > 0) {
            m_hp = std::min(m_hp + amount, m_config.maxHp);
        }
        return {};
    }

    [[nodiscard]] constexpr Core::Result<void, Core::EntityError> ConsumeSp(int32_t amount) noexcept {
        if (m_isDead) {
            return std::unexpected(Core::EntityError::Dead);
        }
        if (amount < 0) {
            return {};
        }
        if (m_sp < amount) {
            return std::unexpected(Core::EntityError::OutOfRange); 
        }
        m_sp -= amount;
        return {};
    }

    [[nodiscard]] constexpr Core::Result<void, Core::EntityError> RecoverSp(int32_t amount) noexcept {
        if (m_isDead) {
            return std::unexpected(Core::EntityError::Dead);
        }
        if (amount > 0) {
            m_sp = std::min(m_sp + amount, m_config.maxSp);
        }
        return {};
    }
    
    constexpr void Revive(int32_t hpPercent) noexcept {
        m_isDead = false;
        hpPercent = std::clamp(hpPercent, 1, 100);
        int64_t hpVal = (static_cast<int64_t>(m_config.maxHp) * hpPercent) / 100;
        m_hp = std::max(1, static_cast<int32_t>(hpVal));
    }

    constexpr void UpdateMaxStats(int32_t newMaxHp, int32_t newMaxSp) noexcept {
        m_config.maxHp = std::max(1, newMaxHp);
        m_config.maxSp = std::max(0, newMaxSp);
        
        m_hp = std::min(m_hp, m_config.maxHp);
        m_sp = std::min(m_sp, m_config.maxSp);

        if (m_hp <= 0 && !m_isDead) {
            m_hp = 1; 
        }
    }

private:
    CombatStateConfig m_config;
    int32_t m_hp;
    int32_t m_sp;
    bool m_isDead;
    bool m_isInCombat;
    std::optional<Core::EntityVid> m_target;
};

} // namespace Client::Actors
