#pragma once

#include <cstdint>
#include <string>
#include <span>
#include "Client/Bridge/IPythonRuntime.h"
#include "Client/Network/Protocol/Protocol.h"
#include "Client/Network/ModernPacketDispatcher.h"
#include "EterBase/Result.h"

namespace Client::Bridge {

    /**
     * @brief Zwarty bridge izolujacy obsluge faz gry i wedkowania od glownej fazy gry, oparty na SRP.
     * Wykorzystuje IPythonRuntime do testowalnosci bez bezposredniej zaleznosci od CPythonNetworkStream.
     */
    class PhaseGameFishingBridge : public Client::Network::IPacketHandler {
    public:
        explicit PhaseGameFishingBridge(IPythonRuntime& pythonRuntime) noexcept
            : m_pythonRuntime(pythonRuntime) {}

        [[nodiscard]] EterBase::PacketResult<void> Handle(std::span<const uint8_t> payload) override;
        [[nodiscard]] uint16_t GetExpectedSize() const override;
        [[nodiscard]] bool IsDynamicSize() const override;

    private:
        IPythonRuntime& m_pythonRuntime;
    };

} // namespace Client::Bridge
