#include "HarnessAuthProtocolRouter.h"
#include <regex>
#include <algorithm>

namespace Client::IPC {

HarnessAuthProtocolRouter::HarnessAuthProtocolRouter(IAuthNetworkProvider& networkProvider)
    : m_networkProvider(networkProvider) {
}

std::expected<void, AuthError> HarnessAuthProtocolRouter::SetCredentials(std::string_view user, std::string_view pass) {
    if (user.empty() || pass.empty()) {
        return std::unexpected(AuthError::InvalidCredentials);
    }
    
    // Basic length validation
    if (user.length() < 4 || user.length() > 32 || pass.length() < 4 || pass.length() > 32) {
        return std::unexpected(AuthError::InvalidCredentials);
    }

    m_user = std::string(user);
    m_pass = std::string(pass);
    return {};
}

std::expected<void, AuthError> HarnessAuthProtocolRouter::Login() {
    if (m_state != AuthState::Disconnected && m_state != AuthState::Authenticated) {
        return std::unexpected(AuthError::InvalidStateTransition);
    }
    
    if (!HasCredentials()) {
        return std::unexpected(AuthError::InvalidCredentials);
    }
    
    m_state = AuthState::Authenticating;
    
    auto result = m_networkProvider.SendLogin(m_user, m_pass);
    if (!result.has_value()) {
        m_state = AuthState::Disconnected;
        return std::unexpected(result.error());
    }
    
    m_state = AuthState::Authenticated;
    m_reconnectAttempts = 0; // Reset attempts on successful login
    return {};
}

std::expected<std::vector<CharacterSlotInfo>, AuthError> HarnessAuthProtocolRouter::GetCharacterSlots() {
    if (m_state != AuthState::Authenticated && m_state != AuthState::SlotsReady) {
        return std::unexpected(AuthError::InvalidStateTransition);
    }
    
    m_state = AuthState::FetchingSlots;
    
    auto result = m_networkProvider.RequestCharacterSlots();
    if (!result.has_value()) {
        m_state = AuthState::Authenticated; // Fallback to authenticated on error
        return std::unexpected(result.error());
    }
    
    m_state = AuthState::SlotsReady;
    return result.value();
}

std::expected<void, AuthError> HarnessAuthProtocolRouter::SelectCharacter(uint8_t slotIndex) {
    if (m_state != AuthState::SlotsReady) {
        return std::unexpected(AuthError::InvalidStateTransition);
    }
    
    if (slotIndex >= 4) { // Typical max slots limit
        return std::unexpected(AuthError::InvalidSlot);
    }
    
    m_state = AuthState::CharacterSelecting;
    
    auto result = m_networkProvider.SendSelectCharacter(slotIndex);
    if (!result.has_value()) {
        m_state = AuthState::SlotsReady; // Fallback
        return std::unexpected(result.error());
    }
    
    m_state = AuthState::EnteringGame;
    return {};
}

std::expected<void, AuthError> HarnessAuthProtocolRouter::CreateCharacter(uint8_t slotIndex, std::string_view name, uint8_t job, uint8_t shape) {
    if (m_state != AuthState::SlotsReady) {
        return std::unexpected(AuthError::InvalidStateTransition);
    }
    
    if (slotIndex >= 4) {
        return std::unexpected(AuthError::InvalidSlot);
    }
    
    if (!ValidateCharacterName(name)) {
        return std::unexpected(AuthError::InvalidCharacterName);
    }
    
    m_state = AuthState::CharacterCreating;
    
    auto result = m_networkProvider.SendCreateCharacter(slotIndex, name, job, shape);
    if (!result.has_value()) {
        m_state = AuthState::SlotsReady; // Fallback
        return std::unexpected(result.error());
    }
    
    m_state = AuthState::SlotsReady; // Creation goes back to slots ready
    return {};
}

void HarnessAuthProtocolRouter::SetAutoReconnect(bool enable) noexcept {
    m_autoReconnect = enable;
    if (!enable) {
        m_reconnectTimer = std::chrono::milliseconds(0);
        m_reconnectAttempts = 0;
    }
}

void HarnessAuthProtocolRouter::TickAutoReconnect(std::chrono::milliseconds deltaTime) {
    if (!m_autoReconnect || m_state != AuthState::Disconnected) {
        return;
    }

    m_reconnectTimer += deltaTime;
    
    // Exponential backoff logic: 2s, 4s, 8s, 16s, capped at 30s
    uint32_t delaySecs = std::min(30u, 2u << std::min(4u, m_reconnectAttempts));
    std::chrono::milliseconds delayTarget(delaySecs * 1000);

    if (m_reconnectTimer >= delayTarget) {
        m_reconnectTimer = std::chrono::milliseconds(0);
        m_reconnectAttempts++;
        
        // Attempt to login. If it fails immediately (e.g. invalid creds), it will fall back to Disconnected.
        (void)Login();
    }
}

bool HarnessAuthProtocolRouter::HasCredentials() const noexcept {
    return !m_user.empty() && !m_pass.empty();
}

AuthState HarnessAuthProtocolRouter::GetState() const noexcept {
    return m_state;
}

bool HarnessAuthProtocolRouter::IsAutoReconnect() const noexcept {
    return m_autoReconnect;
}

bool HarnessAuthProtocolRouter::ValidateCharacterName(std::string_view name) const noexcept {
    if (name.length() < 3 || name.length() > 16) {
        return false;
    }
    
    // Alphanumeric, no spaces
    return std::ranges::all_of(name, [](char c) {
        return std::isalnum(static_cast<unsigned char>(c));
    });
}

void HarnessAuthProtocolRouter::Reset(AuthState state) noexcept {
    m_state = state;
    m_reconnectTimer = std::chrono::milliseconds(0);
    m_reconnectAttempts = 0;
}

} // namespace Client::IPC
