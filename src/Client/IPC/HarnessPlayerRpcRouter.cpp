#include "HarnessPlayerRpcRouter.h"

namespace Client::IPC {

std::string_view to_string(RpcRoutingError error) noexcept {
    switch (error) {
        case RpcRoutingError::None: return "None";
        case RpcRoutingError::InvalidTarget: return "InvalidTarget";
        case RpcRoutingError::AntiSpamCooldown: return "AntiSpamCooldown";
        case RpcRoutingError::MessageTooLong: return "MessageTooLong";
        case RpcRoutingError::OutOfBounds: return "OutOfBounds";
        case RpcRoutingError::InvalidState: return "InvalidState";
        case RpcRoutingError::NoTarget: return "NoTarget";
        case RpcRoutingError::SkillOnCooldown: return "SkillOnCooldown";
        case RpcRoutingError::ItemSlotInvalid: return "ItemSlotInvalid";
        case RpcRoutingError::ChatTypeInvalid: return "ChatTypeInvalid";
        default: return "Unknown";
    }
}

HarnessPlayerRpcRouter::HarnessPlayerRpcRouter() {
    m_history.reserve(128);
}

uint64_t HarnessPlayerRpcRouter::GetCurrentTimeMs() const noexcept {
    if (m_useMockTime) {
        return m_mockTimeMs;
    }
    return std::chrono::duration_cast<std::chrono::milliseconds>(
        std::chrono::steady_clock::now().time_since_epoch()
    ).count();
}

void HarnessPlayerRpcRouter::SetCurrentTime(uint64_t timestampMs) noexcept {
    m_mockTimeMs = timestampMs;
    m_useMockTime = true;
}

void HarnessPlayerRpcRouter::UpdateCurrentPosition(const Client::Core::MapCoords& position) noexcept {
    m_state.currentPosition = position;
}

void HarnessPlayerRpcRouter::PushCommand(RpcCommandPayload payload) {
    m_history.push_back({GetCurrentTimeMs(), std::move(payload)});
}

Client::Core::Result<void, RpcRoutingError> HarnessPlayerRpcRouter::RouteMoveToPosition(const Client::Core::MapCoords& destination, bool isRunning) {
    // Bounds checking (example game world limits: -1,000,000 to 1,000,000)
    if (destination.x < -1000000.0f || destination.x > 1000000.0f ||
        destination.y < -1000000.0f || destination.y > 1000000.0f) {
        return std::unexpected(RpcRoutingError::OutOfBounds);
    }
    
    // Check if destination is exactly the same as current position
    if (m_state.currentPosition.x == destination.x && 
        m_state.currentPosition.y == destination.y) {
        return std::unexpected(RpcRoutingError::InvalidState);
    }

    m_state.isMoving = true;
    m_state.targetPosition = destination;
    
    PushCommand(CommandMoveToPosition{destination, isRunning});
    return {};
}

Client::Core::Result<void, RpcRoutingError> HarnessPlayerRpcRouter::RouteStop() {
    if (!m_state.isMoving) {
        return std::unexpected(RpcRoutingError::InvalidState);
    }

    m_state.isMoving = false;
    PushCommand(CommandStop{});
    return {};
}

Client::Core::Result<void, RpcRoutingError> HarnessPlayerRpcRouter::RouteActionAttack(Client::Core::EntityVid targetVid, uint8_t attackType) {
    if (!targetVid) {
        return std::unexpected(RpcRoutingError::InvalidTarget);
    }

    uint64_t now = GetCurrentTimeMs();
    // 300ms attack cooldown
    if (now - m_state.lastAttackTime < 300) {
        return std::unexpected(RpcRoutingError::AntiSpamCooldown);
    }

    m_state.lastAttackTime = now;
    // Set target automatically on attack
    m_state.currentTarget = targetVid;
    
    PushCommand(CommandActionAttack{targetVid, attackType});
    return {};
}

Client::Core::Result<void, RpcRoutingError> HarnessPlayerRpcRouter::RouteActionUseItem(Client::Core::ItemSlot slot) {
    if (!slot) {
        return std::unexpected(RpcRoutingError::ItemSlotInvalid);
    }

    // Limit inventory slots up to 500 for validation
    if (slot.get() >= 500) {
        return std::unexpected(RpcRoutingError::ItemSlotInvalid);
    }

    PushCommand(CommandActionUseItem{slot});
    return {};
}

Client::Core::Result<void, RpcRoutingError> HarnessPlayerRpcRouter::RouteSetTarget(Client::Core::EntityVid targetVid) {
    if (!targetVid) {
        return std::unexpected(RpcRoutingError::InvalidTarget);
    }

    m_state.currentTarget = targetVid;
    PushCommand(CommandSetTarget{targetVid});
    return {};
}

Client::Core::Result<void, RpcRoutingError> HarnessPlayerRpcRouter::RouteClearTarget() {
    if (!m_state.currentTarget) {
        return std::unexpected(RpcRoutingError::NoTarget);
    }

    m_state.currentTarget = Client::Core::EntityVid{0};
    PushCommand(CommandClearTarget{});
    return {};
}

Client::Core::Result<void, RpcRoutingError> HarnessPlayerRpcRouter::RouteUseSkill(Client::Core::SkillIndex skillId, Client::Core::EntityVid targetVid) {
    if (!skillId) {
        return std::unexpected(RpcRoutingError::SkillOnCooldown); // Using this error to imply invalid skill config
    }

    uint64_t now = GetCurrentTimeMs();
    auto it = m_state.skillCooldowns.find(skillId.get());
    if (it != m_state.skillCooldowns.end()) {
        // Assume all skills have a base 1000ms cooldown for now for validation
        if (now - it->second < 1000) {
            return std::unexpected(RpcRoutingError::SkillOnCooldown);
        }
    }

    m_state.skillCooldowns[skillId.get()] = now;
    PushCommand(CommandUseSkill{skillId, targetVid});
    return {};
}

Client::Core::Result<void, RpcRoutingError> HarnessPlayerRpcRouter::RoutePickCloseItem() {
    uint64_t now = GetCurrentTimeMs();
    // 500ms pick cooldown
    if (now - m_state.lastPickTime < 500) {
        return std::unexpected(RpcRoutingError::AntiSpamCooldown);
    }

    m_state.lastPickTime = now;
    PushCommand(CommandPickCloseItem{});
    return {};
}

Client::Core::Result<void, RpcRoutingError> HarnessPlayerRpcRouter::RouteSendChat(const std::string& message, uint8_t chatType) {
    if (message.empty() || message.length() > 512) {
        return std::unexpected(RpcRoutingError::MessageTooLong);
    }

    if (chatType > 10) { // arbitrary validation of chat types
        return std::unexpected(RpcRoutingError::ChatTypeInvalid);
    }

    uint64_t now = GetCurrentTimeMs();
    
    // Simple Anti-Spam: max 5 messages per 2 seconds
    if (now - m_state.chatWindowStartTime > 2000) {
        m_state.chatWindowStartTime = now;
        m_state.chatMessageCountInWindow = 0;
    }
    
    if (m_state.chatMessageCountInWindow >= 5) {
        return std::unexpected(RpcRoutingError::AntiSpamCooldown);
    }

    m_state.chatMessageCountInWindow++;
    m_state.lastChatTime = now;

    PushCommand(CommandSendChat{message, chatType});
    return {};
}

} // namespace Client::IPC
