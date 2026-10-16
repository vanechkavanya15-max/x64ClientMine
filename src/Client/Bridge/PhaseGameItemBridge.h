#pragma once

#include "Client/Bridge/IPythonRuntime.h"
#include "Client/Network/Protocol/Protocol.h"
#include <string_view>
#include <cstdint>
#include <span>

namespace Client::Bridge {

    /**
     * @brief Zwarty bridge izolujacy obsluge przedmiotow od glownej fazy gry, oparty na SRP.
     * Wykorzystuje IPythonRuntime do testowalnosci bez bezposredniej zaleznosci od CPythonNetworkStream czy UI.
     */
    class PhaseGameItemBridge {
    public:
        explicit PhaseGameItemBridge(IPythonRuntime& pythonRuntime) noexcept
            : m_pythonRuntime(pythonRuntime) {}

        bool HandleItemSet(const TPacketGCItemSet& pack);
        bool HandleItemDel(const TPacketGCItemDel& pack);
        bool HandleItemOwnership(const TPacketGCItemOwnership& pack);
        bool HandleItemGroundAdd(const TPacketGCItemGroundAdd& pack);
        bool HandleItemGroundDel(const TPacketGCItemGroundDel& pack);

    private:
        IPythonRuntime& m_pythonRuntime;
    };

} // namespace Client::Bridge
