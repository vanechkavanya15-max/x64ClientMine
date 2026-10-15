#pragma once

#include "Client/Bridge/IPythonRuntime.h"
#include "Client/Network/Protocol/Protocol.h"
#include "Client/Core/EventBus.h"

namespace Client::Bridge {

    /**
     * @brief Zwarty bridge izolujacy obsluge faz gry i warp od glownej fazy gry, oparty na SRP.
     * Wykorzystuje IPythonRuntime do testowalnosci bez bezposredniej zaleznosci od CPythonNetworkStream.
     */
    class PhaseGameWarpBridge {
    public:
        explicit PhaseGameWarpBridge(IPythonRuntime& pythonRuntime) noexcept
            : m_pythonRuntime(pythonRuntime) {}

        bool HandleWarp(const TPacketGCWarp& pack);
        void Warp(int32_t globalX, int32_t globalY);
        void ShowMapName(const std::string& mapName, int32_t localX, int32_t localY);
        void SetGamePhase();
        void LeaveGamePhase();

    private:
        IPythonRuntime& m_pythonRuntime;
    };

} // namespace Client::Bridge
