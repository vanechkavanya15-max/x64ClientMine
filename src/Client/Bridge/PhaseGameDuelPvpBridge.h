#pragma once
#include <cstdint>
#include <span>
#include <vector>
#include "Client/Bridge/IPythonRuntime.h"
#include "Client/Network/Protocol/Protocol_Combat.h"
#include "EterBase/Result.h"

namespace Client::Bridge {

class PhaseGameDuelPvpBridge {
public:
    explicit PhaseGameDuelPvpBridge(IPythonRuntime& pythonRuntime) noexcept
        : m_pythonRuntime(pythonRuntime) {}
    ~PhaseGameDuelPvpBridge() = default;

    PhaseGameDuelPvpBridge(const PhaseGameDuelPvpBridge&) = delete;
    PhaseGameDuelPvpBridge& operator=(const PhaseGameDuelPvpBridge&) = delete;

    [[nodiscard]] EterBase::PacketResult<void> HandlePvpPacket(const TPacketGCPVP& packet);
    [[nodiscard]] EterBase::PacketResult<void> HandleDuelStartPacket(std::span<const uint32_t> duelPairs, uint32_t mainCharacterVid);

    // Legacy headless state logic for completeness in tests/behavior
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
    IPythonRuntime& m_pythonRuntime;
    uint32_t m_opponentVid{0};
    bool m_inDuel{false};
};

} // namespace Client::Bridge
