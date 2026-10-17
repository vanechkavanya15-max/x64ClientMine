#pragma once

#include "Client/Bridge/IPythonRuntime.h"
#include <vector>
#include <span>
#include <cstdint>

namespace Client::Bridge {

    /**
     * @brief Zwarty bridge izolujacy obsluge znajomych (Messenger) od glownej fazy gry, oparty na SRP.
     * Wykorzystuje IPythonRuntime do testowalnosci bez bezposredniej zaleznosci od globalnych komponentow GUI.
     */
    class PhaseGameMessengerBridge {
    public:
        explicit PhaseGameMessengerBridge(IPythonRuntime& pythonRuntime) noexcept
            : m_pythonRuntime(pythonRuntime) {}

        bool HandleMessenger(std::span<const uint8_t> buffer);

    private:
        IPythonRuntime& m_pythonRuntime;
    };

} // namespace Client::Bridge
