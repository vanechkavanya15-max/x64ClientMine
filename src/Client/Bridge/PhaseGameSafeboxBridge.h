#pragma once

#include "Client/Bridge/IPythonRuntime.h"
#include <cstdint>

namespace Client::Bridge {

    /**
     * @brief Zwarty bridge izolujacy obsluge Safebox / Mall od glownej fazy gry, oparty na SRP.
     * Wykorzystuje IPythonRuntime do testowalnosci bez bezposredniej zaleznosci od CPythonNetworkStream.
     */
    class PhaseGameSafeboxBridge {
    public:
        explicit PhaseGameSafeboxBridge(IPythonRuntime& pythonRuntime) noexcept
            : m_pythonRuntime(pythonRuntime) {}

        bool HandleSafeboxWrongPassword();
        bool HandleSafeboxSize(uint8_t size);
        bool HandleSafeboxMoneyChange(int32_t money);
        bool HandleMallOpen(uint8_t size);

    private:
        IPythonRuntime& m_pythonRuntime;
    };

} // namespace Client::Bridge
