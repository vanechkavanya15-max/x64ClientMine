#pragma once
#include <cstdint>
#include <string>

namespace Client::IPC {

class HarnessAuthProtocolRouter {
public:
    HarnessAuthProtocolRouter() = default;
    ~HarnessAuthProtocolRouter() = default;

    bool SetCredentials(const std::string& user, const std::string& pass);
    bool SelectSlot(uint8_t slot);
    void SetAutoReconnect(bool enable) noexcept { m_autoReconnect = enable; }

    [[nodiscard]] bool HasCredentials() const noexcept { return !m_user.empty(); }
    [[nodiscard]] uint8_t GetSelectedSlot() const noexcept { return m_slot; }
    [[nodiscard]] bool IsAutoReconnect() const noexcept { return m_autoReconnect; }

private:
    std::string m_user{};
    std::string m_pass{};
    uint8_t m_slot{0};
    bool m_autoReconnect{false};
};

} // namespace Client::IPC
