#pragma once
#include <cstdint>

namespace Client::Bridge {

class PhaseGameDuelPvpBridge {
public:
    PhaseGameDuelPvpBridge() = default;
    ~PhaseGameDuelPvpBridge() = default;

    void StartDuel(uint32_t opponentVid) noexcept {
        m_opponentVid = opponentVid;
        m_inDuel = true;
    }
    void EndDuel() noexcept {
        m_opponentVid = 0;
        m_inDuel = false;
    }
    [[nodiscard]] bool IsInDuel() const noexcept { return m_inDuel; }
    [[nodiscard]] uint32_t GetOpponentVid() const noexcept { return m_opponentVid; }

private:
    uint32_t m_opponentVid{0};
    bool m_inDuel{false};
};

} // namespace Client::Bridge
