#pragma once

#include "Client/IPC/HarnessAuthProtocolRouter.h"
#include <string>
#include <string_view>
#include <vector>
#include <expected>

namespace UserInterface::TestHarness
{
    /**
     * @brief Implementacja dostawcy sieciowego IAuthNetworkProvider dla HarnessAuthProtocolRouter.
     * Przekazuje wywolania logowania i zarzadzania postaciami do CAccountConnector oraz CPythonNetworkStream.
     */
    class HarnessNetworkAuthProvider : public Client::IPC::IAuthNetworkProvider
    {
    public:
        HarnessNetworkAuthProvider() = default;
        ~HarnessNetworkAuthProvider() override = default;

        HarnessNetworkAuthProvider(const HarnessNetworkAuthProvider&) = delete;
        HarnessNetworkAuthProvider& operator=(const HarnessNetworkAuthProvider&) = delete;

        void SetConnectionTarget(std::string_view serverIp, int authPort, int gamePort, std::string_view pin = "", int slot = 0);

        std::expected<void, Client::IPC::AuthError> SendLogin(std::string_view user, std::string_view pass) override;
        std::expected<std::vector<Client::IPC::CharacterSlotInfo>, Client::IPC::AuthError> RequestCharacterSlots() override;
        std::expected<void, Client::IPC::AuthError> SendSelectCharacter(uint8_t slotIndex) override;
        std::expected<void, Client::IPC::AuthError> SendCreateCharacter(uint8_t slotIndex, std::string_view name, uint8_t job, uint8_t shape) override;

        [[nodiscard]] const std::string& GetServerIp() const noexcept { return m_serverIp; }
        [[nodiscard]] int GetAuthPort() const noexcept { return m_authPort; }
        [[nodiscard]] int GetGamePort() const noexcept { return m_gamePort; }
        [[nodiscard]] const std::string& GetPin() const noexcept { return m_pin; }
        [[nodiscard]] int GetDirectSlot() const noexcept { return m_slot; }

    private:
        std::string m_serverIp{"127.0.0.1"};
        int m_authPort{11000};
        int m_gamePort{11011};
        std::string m_pin{""};
        int m_slot{0};
    };
}
