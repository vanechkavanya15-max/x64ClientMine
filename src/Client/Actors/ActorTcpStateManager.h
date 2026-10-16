#pragma once
#include <cstdint>

namespace Client::Actors {

enum ActorStateBits : uint32_t {
    STATE_NONE     = 0,
    STATE_STUN     = 1 << 0,
    STATE_DEAD     = 1 << 1,
    STATE_POISON   = 1 << 2,
    STATE_SLOW     = 1 << 3,
    STATE_BERSERK  = 1 << 4,
    STATE_BLEEDING = 1 << 5
};

class ActorTcpStateManager {
public:
    ActorTcpStateManager() = default;
    ~ActorTcpStateManager() = default;

    void SetState(uint32_t stateBits) noexcept { m_stateMask |= stateBits; }
    void ClearState(uint32_t stateBits) noexcept { m_stateMask &= ~stateBits; }
    void ResetAll() noexcept { m_stateMask = STATE_NONE; }

    [[nodiscard]] bool HasState(uint32_t stateBits) const noexcept {
        return (m_stateMask & stateBits) == stateBits;
    }
    [[nodiscard]] bool IsStunned() const noexcept { return HasState(STATE_STUN); }
    [[nodiscard]] bool IsDead() const noexcept { return HasState(STATE_DEAD); }
    [[nodiscard]] bool CanMove() const noexcept { return !IsStunned() && !IsDead(); }

private:
    uint32_t m_stateMask{STATE_NONE};
};

} // namespace Client::Actors
