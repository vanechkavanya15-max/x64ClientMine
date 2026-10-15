#pragma once

#include "Client/Bridge/IPythonRuntime.h"
#include "Client/Network/Protocol/Packets/Packet_DamageInfo.h"
#include "Client/Network/Protocol/Packets/Packet_Dead.h"

namespace Client::Bridge {

    /**
     * @brief Zwarty bridge izolujacy obsluge walki od glownej fazy gry, oparty na SRP.
     * Wykorzystuje IPythonRuntime do testowalnosci bez bezposredniej zaleznosci od CPythonNetworkStream.
     */
    class PhaseGameCombatBridge {
    public:
        explicit PhaseGameCombatBridge(IPythonRuntime& pythonRuntime) noexcept
            : m_pythonRuntime(pythonRuntime) {}

        bool HandleDamageInfo(const TPacketGCDamageInfo& pack);
        bool HandleDead(const TPacketGCDead& pack);

    private:
        IPythonRuntime& m_pythonRuntime;
    };

} // namespace Client::Bridge
