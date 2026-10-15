#pragma once

#include "Client/Bridge/IPythonRuntime.h"
#include "Client/Network/QuestPacketCodec.h"
#include <string>

namespace Client::Bridge {

    /**
     * @brief Zwarty bridge izolujacy obsluge zadan od glownej fazy gry, oparty na SRP.
     * Wykorzystuje IPythonRuntime do testowalnosci bez bezposredniej zaleznosci od CPythonNetworkStream.
     */
    class PhaseGameQuestBridge {
    public:
        explicit PhaseGameQuestBridge(IPythonRuntime& pythonRuntime) noexcept
            : m_pythonRuntime(pythonRuntime) {}

        bool HandleQuestInfo(const TPacketGCQuestInfo& pack);
        bool HandleQuestConfirm(const TPacketGCQuestConfirm& pack);
        bool HandleScript(const std::string& script);

    private:
        IPythonRuntime& m_pythonRuntime;
    };

} // namespace Client::Bridge
