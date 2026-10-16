#pragma once

#include <cstdint>
#include <string>
#include <vector>
#include <variant>
#include <map>
#include <string_view>
#include <chrono>

#include "Client/Core/StrongTypes.h"
#include "Client/Core/Result.h"

namespace Client::IPC {

enum class RpcRoutingError : uint8_t {
    None = 0,
    InvalidTarget,
    AntiSpamCooldown,
    MessageTooLong,
    OutOfBounds,
    InvalidState,
    NoTarget,
    SkillOnCooldown,
    ItemSlotInvalid,
    ChatTypeInvalid
};

[[nodiscard]] std::string_view to_string(RpcRoutingError error) noexcept;

struct CommandMoveToPosition {
    Client::Core::MapCoords destination;
    bool isRunning;
};

struct CommandStop {
};

struct CommandActionAttack {
    Client::Core::EntityVid targetVid;
    uint8_t attackType;
};

struct CommandActionUseItem {
    Client::Core::ItemSlot slot;
};

struct CommandSetTarget {
    Client::Core::EntityVid targetVid;
};

struct CommandClearTarget {
};

struct CommandUseSkill {
    Client::Core::SkillIndex skillId;
    Client::Core::EntityVid targetVid; // can be empty (0)
};

struct CommandPickCloseItem {
};

struct CommandSendChat {
    std::string message;
    uint8_t chatType;
};

using RpcCommandPayload = std::variant<
    CommandMoveToPosition,
    CommandStop,
    CommandActionAttack,
    CommandActionUseItem,
    CommandSetTarget,
    CommandClearTarget,
    CommandUseSkill,
    CommandPickCloseItem,
    CommandSendChat
>;

struct PlayerRpcCommand {
    uint64_t timestamp;
    RpcCommandPayload payload;
};

struct RouterState {
    bool isMoving{false};
    Client::Core::MapCoords currentPosition{0.0f, 0.0f, 0.0f};
    Client::Core::MapCoords targetPosition{0.0f, 0.0f, 0.0f};
    
    Client::Core::EntityVid currentTarget{0};
    
    // Cooldown tracking
    uint64_t lastChatTime{0};
    uint64_t lastAttackTime{0};
    uint64_t lastPickTime{0};
    uint32_t chatMessageCountInWindow{0};
    uint64_t chatWindowStartTime{0};

    std::map<uint32_t, uint64_t> skillCooldowns; // SkillIndex.get() -> timestamp
};

class HarnessPlayerRpcRouter {
public:
    HarnessPlayerRpcRouter();
    ~HarnessPlayerRpcRouter() = default;

    HarnessPlayerRpcRouter(const HarnessPlayerRpcRouter&) = delete;
    HarnessPlayerRpcRouter& operator=(const HarnessPlayerRpcRouter&) = delete;

    // Command Routing methods
    Client::Core::Result<void, RpcRoutingError> RouteMoveToPosition(const Client::Core::MapCoords& destination, bool isRunning = true);
    Client::Core::Result<void, RpcRoutingError> RouteStop();
    Client::Core::Result<void, RpcRoutingError> RouteActionAttack(Client::Core::EntityVid targetVid, uint8_t attackType = 0);
    Client::Core::Result<void, RpcRoutingError> RouteActionUseItem(Client::Core::ItemSlot slot);
    Client::Core::Result<void, RpcRoutingError> RouteSetTarget(Client::Core::EntityVid targetVid);
    Client::Core::Result<void, RpcRoutingError> RouteClearTarget();
    Client::Core::Result<void, RpcRoutingError> RouteUseSkill(Client::Core::SkillIndex skillId, Client::Core::EntityVid targetVid = Client::Core::EntityVid{0});
    Client::Core::Result<void, RpcRoutingError> RoutePickCloseItem();
    Client::Core::Result<void, RpcRoutingError> RouteSendChat(const std::string& message, uint8_t chatType = 1);

    // State Management / Test Hooks
    void UpdateCurrentPosition(const Client::Core::MapCoords& position) noexcept;
    void SetCurrentTime(uint64_t timestampMs) noexcept; // For dependency injection of time
    [[nodiscard]] const RouterState& GetState() const noexcept { return m_state; }

    [[nodiscard]] const std::vector<PlayerRpcCommand>& GetCommandHistory() const noexcept {
        return m_history;
    }
    void ClearHistory() noexcept { m_history.clear(); }

private:
    uint64_t GetCurrentTimeMs() const noexcept;
    void PushCommand(RpcCommandPayload payload);
    
    std::vector<PlayerRpcCommand> m_history;
    RouterState m_state;
    
    uint64_t m_mockTimeMs{0};
    bool m_useMockTime{false};
};

} // namespace Client::IPC

// Formatting support for RpcRoutingError
template <>
struct std::formatter<Client::IPC::RpcRoutingError> : std::formatter<std::string_view> {
    auto format(Client::IPC::RpcRoutingError err, std::format_context& ctx) const {
        return std::formatter<std::string_view>::format(Client::IPC::to_string(err), ctx);
    }
};
