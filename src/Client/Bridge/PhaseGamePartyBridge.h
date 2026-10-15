#pragma once

#include "Client/Bridge/IPythonRuntime.h"
#include "Client/Network/PartyPacketCodec.h"
#include <string_view>

namespace Client::Bridge {

    /**
     * @brief Zwarty bridge izolujacy obsluge party (grupy) od glownej fazy gry, oparty na SRP.
     * Wykorzystuje IPythonRuntime do testowalnosci bez bezposredniej zaleznosci od CPythonNetworkStream czy UI.
     */
    class PhaseGamePartyBridge {
    public:
        explicit PhaseGamePartyBridge(IPythonRuntime& pythonRuntime) noexcept
            : m_pythonRuntime(pythonRuntime) {}

        bool HandlePartyInvite(const TPacketGCPartyInvite& pack, std::string_view leaderName);
        bool HandlePartyAdd(const TPacketGCPartyAdd& pack);
        bool HandlePartyUpdate(const TPacketGCPartyUpdate& pack);
        bool HandlePartyRemove(const TPacketGCPartyRemove& pack);
        bool HandlePartyParameter(const TPacketGCPartyParameter& pack);

    private:
        IPythonRuntime& m_pythonRuntime;
    };

} // namespace Client::Bridge
