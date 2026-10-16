#pragma once

#include <cstdint>
#include <string>
#include <string_view>
#include <vector>
#include <expected>
#include <chrono>

namespace Client::IPC {

enum class AuthState : uint8_t {
    Disconnected,
    Connecting,
    Authenticating,
    Authenticated,
    FetchingSlots,
    SlotsReady,
    CharacterSelecting,
    CharacterCreating,
    EnteringGame
};

enum class AuthError : uint8_t {
    InvalidCredentials,
    InvalidSlot,
    NetworkError,
    ServerFull,
    BannedAccount,
    AlreadyLoggedIn,
    InvalidStateTransition,
    CharacterNameTaken,
    InvalidCharacterName,
    Timeout
};

[[nodiscard]] constexpr std::string_view to_string(AuthError err) noexcept {
    switch (err) {
        case AuthError::InvalidCredentials: return "InvalidCredentials";
        case AuthError::InvalidSlot: return "InvalidSlot";
        case AuthError::NetworkError: return "NetworkError";
        case AuthError::ServerFull: return "ServerFull";
        case AuthError::BannedAccount: return "BannedAccount";
        case AuthError::AlreadyLoggedIn: return "AlreadyLoggedIn";
        case AuthError::InvalidStateTransition: return "InvalidStateTransition";
        case AuthError::CharacterNameTaken: return "CharacterNameTaken";
        case AuthError::InvalidCharacterName: return "InvalidCharacterName";
        case AuthError::Timeout: return "Timeout";
    }
    return "UnknownError";
}

struct CharacterSlotInfo {
    uint8_t slotIndex;
    std::string name;
    uint8_t level;
    uint8_t job;
    uint32_t playtime;
};

class IAuthNetworkProvider {
public:
    virtual ~IAuthNetworkProvider() = default;
    
    virtual std::expected<void, AuthError> SendLogin(std::string_view user, std::string_view pass) = 0;
    virtual std::expected<std::vector<CharacterSlotInfo>, AuthError> RequestCharacterSlots() = 0;
    virtual std::expected<void, AuthError> SendSelectCharacter(uint8_t slotIndex) = 0;
    virtual std::expected<void, AuthError> SendCreateCharacter(uint8_t slotIndex, std::string_view name, uint8_t job, uint8_t shape) = 0;
};

class HarnessAuthProtocolRouter {
public:
    explicit HarnessAuthProtocolRouter(IAuthNetworkProvider& networkProvider);
    ~HarnessAuthProtocolRouter() = default;

    HarnessAuthProtocolRouter(const HarnessAuthProtocolRouter&) = delete;
    HarnessAuthProtocolRouter& operator=(const HarnessAuthProtocolRouter&) = delete;

    std::expected<void, AuthError> SetCredentials(std::string_view user, std::string_view pass);
    std::expected<void, AuthError> Login();
    
    std::expected<std::vector<CharacterSlotInfo>, AuthError> GetCharacterSlots();
    std::expected<void, AuthError> SelectCharacter(uint8_t slotIndex);
    std::expected<void, AuthError> CreateCharacter(uint8_t slotIndex, std::string_view name, uint8_t job, uint8_t shape);

    void SetAutoReconnect(bool enable) noexcept;
    void TickAutoReconnect(std::chrono::milliseconds deltaTime);

    [[nodiscard]] bool HasCredentials() const noexcept;
    [[nodiscard]] AuthState GetState() const noexcept;
    [[nodiscard]] bool IsAutoReconnect() const noexcept;

private:
    IAuthNetworkProvider& m_networkProvider;
    
    std::string m_user{};
    std::string m_pass{};
    
    AuthState m_state{AuthState::Disconnected};
    
    bool m_autoReconnect{false};
    std::chrono::milliseconds m_reconnectTimer{0};
    uint32_t m_reconnectAttempts{0};
    
    bool ValidateCharacterName(std::string_view name) const noexcept;
};

} // namespace Client::IPC
