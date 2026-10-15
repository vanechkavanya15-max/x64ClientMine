#pragma once

#include <cstdint>
#include <optional>
#include "Client/Core/StrongTypes.h"
#include "Client/Core/Result.h"

namespace Client::Actors {

/**
 * @brief Zero-DirectX, headless C++23 component for tracking actor combat state.
 */
class ActorCombatStateComponent {
public:
    ActorCombatStateComponent() noexcept = default;
    ~ActorCombatStateComponent() noexcept = default;

    ActorCombatStateComponent(const ActorCombatStateComponent&) = delete;
    ActorCombatStateComponent& operator=(const ActorCombatStateComponent&) = delete;
    ActorCombatStateComponent(ActorCombatStateComponent&&) noexcept = default;
    ActorCombatStateComponent& operator=(ActorCombatStateComponent&&) noexcept = default;

    // HP getters and setters
    [[nodiscard]] uint32_t GetHp() const noexcept;
    [[nodiscard]] uint32_t GetMaxHp() const noexcept;
    void SetHp(uint32_t hp) noexcept;
    void SetMaxHp(uint32_t maxHp) noexcept;

    // SP getters and setters
    [[nodiscard]] uint32_t GetSp() const noexcept;
    [[nodiscard]] uint32_t GetMaxSp() const noexcept;
    void SetSp(uint32_t sp) noexcept;
    void SetMaxSp(uint32_t maxSp) noexcept;

    // Target tracking
    [[nodiscard]] std::optional<Client::Core::EntityVid> GetTargetVid() const noexcept;
    void SetTargetVid(Client::Core::EntityVid vid) noexcept;
    void ClearTargetVid() noexcept;

    // Death state
    [[nodiscard]] bool IsDead() const noexcept;
    void SetDead(bool dead) noexcept;

    // Combat logic
    Client::Core::Result<void, Client::Core::ActorError> TakeDamage(uint32_t damage) noexcept;

private:
    uint32_t hp_{0};
    uint32_t maxHp_{0};
    uint32_t sp_{0};
    uint32_t maxSp_{0};
    bool isDead_{false};
    std::optional<Client::Core::EntityVid> targetVid_;
};

} // namespace Client::Actors
