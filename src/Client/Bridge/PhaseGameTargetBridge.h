#pragma once

#include "Client/Bridge/IPythonRuntime.h"
#include "Client/Network/TargetPacketCodec.h"
#include "EterBase/Result.h"
#include <span>
#include <cstdint>

namespace Client::Bridge {

    /**
     * @brief Zwarty bridge izolujacy obsluge celow i lotow od glownej fazy gry, oparty na SRP.
     * Wykorzystuje IPythonRuntime do testowalnosci bez bezposredniej zaleznosci od CPythonNetworkStream.
     */
    class PhaseGameTargetBridge {
    public:
        explicit PhaseGameTargetBridge(IPythonRuntime& pythonRuntime) noexcept
            : m_pythonRuntime(pythonRuntime) {}

        EterBase::PacketResult<void> HandleTargetCreate(std::span<const uint8_t> payload);
        EterBase::PacketResult<void> HandleTargetUpdate(std::span<const uint8_t> payload);
        EterBase::PacketResult<void> HandleTargetDelete(std::span<const uint8_t> payload);
        EterBase::PacketResult<void> HandleCreateFly(std::span<const uint8_t> payload);

    private:
        IPythonRuntime& m_pythonRuntime;
    };

} // namespace Client::Bridge
