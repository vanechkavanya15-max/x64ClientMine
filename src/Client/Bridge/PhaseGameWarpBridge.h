#pragma once

#include <cstdint>
#include <string>
#include <span>
#include "Client/Bridge/IPythonRuntime.h"
#include "Client/Network/Protocol/Protocol.h"
#include "Client/Core/EventBus.h"
#include "Client/Core/StrongTypes.h"
#include "Client/Network/ModernPacketDispatcher.h"
#include "EterBase/Result.h"

namespace Client::Bridge {

    /**
     * @brief Zwarty bridge izolujacy obsluge faz gry i warp od glownej fazy gry, oparty na SRP.
     * Wykorzystuje IPythonRuntime do testowalnosci bez bezposredniej zaleznosci od CPythonNetworkStream.
     */
    class PhaseGameWarpBridge : public Client::Network::IPacketHandler {
    public:
        explicit PhaseGameWarpBridge(IPythonRuntime& pythonRuntime) noexcept
            : m_pythonRuntime(pythonRuntime) {}

        [[nodiscard]] EterBase::PacketResult<void> Handle(std::span<const uint8_t> payload) override;
        [[nodiscard]] uint16_t GetExpectedSize() const override;
        [[nodiscard]] bool IsDynamicSize() const override;

        void Warp(int32_t globalX, int32_t globalY);
        void ShowMapName(const std::string& mapName, int32_t localX, int32_t localY);
        void SetGamePhase();
        void LeaveGamePhase();

    private:
        IPythonRuntime& m_pythonRuntime;
    };

} // namespace Client::Bridge
