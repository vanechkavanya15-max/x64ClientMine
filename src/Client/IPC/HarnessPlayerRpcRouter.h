#pragma once
#include <cstdint>
#include <string>
#include <vector>

namespace Client::IPC {

struct PlayerRpcCommand {
    std::string method{};
    uint32_t targetVid{0};
    uint32_t itemSlot{0};
    float posX{0.0f};
    float posY{0.0f};
    std::string chatMsg{};
};

class HarnessPlayerRpcRouter {
public:
    HarnessPlayerRpcRouter() = default;
    ~HarnessPlayerRpcRouter() = default;

    bool RouteMove(float x, float y);
    bool RouteAttack(uint32_t targetVid);
    bool RouteUseItem(uint32_t slot);
    bool RouteSendChat(const std::string& msg);

    [[nodiscard]] const std::vector<PlayerRpcCommand>& GetCommandHistory() const noexcept {
        return m_history;
    }
    void ClearHistory() noexcept { m_history.clear(); }

private:
    std::vector<PlayerRpcCommand> m_history;
};

} // namespace Client::IPC
