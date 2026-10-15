#pragma once

#include "Client/Bridge/IPythonRuntime.h"
#include "Client/Network/ExchangePacketCodec.h"
#include "Protocol/Protocol.h"

namespace Client::Bridge {

    /**
     * @brief Zwarty bridge izolujacy obsluge handlu (exchange) od glownej fazy gry, oparty na SRP.
     * Wykorzystuje IPythonRuntime do testowalnosci bez bezposredniej zaleznosci od CPythonNetworkStream i PythonExchange.
     */
    class PhaseGameExchangeBridge {
    public:
        explicit PhaseGameExchangeBridge(IPythonRuntime& pythonRuntime) noexcept
            : m_pythonRuntime(pythonRuntime) {}

        bool HandleExchangePacket(const TPacketGCExchange& pack);

    private:
        bool HandleExchangeSub_Start(const TPacketGCExchange& pack);
        bool HandleExchangeSub_ItemAdd(const TPacketGCExchange& pack);
        bool HandleExchangeSub_ItemDel(const TPacketGCExchange& pack);
        bool HandleExchangeSub_ElkAdd(const TPacketGCExchange& pack);
        bool HandleExchangeSub_Accept(const TPacketGCExchange& pack);
        bool HandleExchangeSub_End(const TPacketGCExchange& pack);
        bool HandleExchangeSub_Already(const TPacketGCExchange& pack);
        bool HandleExchangeSub_LessElk(const TPacketGCExchange& pack);

        IPythonRuntime& m_pythonRuntime;
    };

} // namespace Client::Bridge
