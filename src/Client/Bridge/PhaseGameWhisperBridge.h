#pragma once
#include "Client/Network/ModernPacketDispatcher.h"
#include "Client/Bridge/IPythonRuntime.h"
#include <span>
#include <cstdint>

namespace Client::Bridge {

#pragma pack(push, 1)
    struct PacketWhisperHeader
    {
        uint16_t header;
        uint16_t length;
        uint8_t type;
        char senderName[64 + 1];
    };
#pragma pack(pop)

    class PhaseGameWhisperBridge final : public Client::Network::IPacketHandler {
    public:
        explicit PhaseGameWhisperBridge(IPythonRuntime& pythonRuntime) noexcept
            : m_pythonRuntime(pythonRuntime) {}

        [[nodiscard]] EterBase::PacketResult<void> Handle(std::span<const uint8_t> payload) override;
        [[nodiscard]] uint16_t GetExpectedSize() const override { return sizeof(PacketWhisperHeader); }
        [[nodiscard]] bool IsDynamicSize() const override { return true; }

    private:
        IPythonRuntime& m_pythonRuntime;
    };
}
