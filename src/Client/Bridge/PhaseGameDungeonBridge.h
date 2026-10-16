#pragma once

#include "Client/Bridge/IPythonRuntime.h"
#include "Client/Network/Protocol/Protocol.h"
#include <cstdint>

namespace Client::Bridge {

    /**
     * @brief Zwarty bridge izolujacy obsluge Dungeon od glownej fazy gry, oparty na SRP.
     * Wykorzystuje IPythonRuntime do testowalnosci bez bezposredniej zaleznosci od CPythonNetworkStream.
     */
    class PhaseGameDungeonBridge {
    public:
        explicit PhaseGameDungeonBridge(IPythonRuntime& pythonRuntime) noexcept
            : m_pythonRuntime(pythonRuntime) {}

        bool HandleDungeon(const TPacketGCDungeon& pack, uint32_t destX = 0, uint32_t destY = 0);

    private:
        IPythonRuntime& m_pythonRuntime;
    };

} // namespace Client::Bridge
