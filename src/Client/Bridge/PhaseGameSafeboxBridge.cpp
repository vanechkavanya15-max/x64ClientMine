#include "PhaseGameSafeboxBridge.h"
#include <string>

namespace Client::Bridge {

    bool PhaseGameSafeboxBridge::HandleSafeboxWrongPassword() {
        auto result = m_pythonRuntime.Invoke("game", "OnSafeBoxError");
        return result.has_value();
    }

    bool PhaseGameSafeboxBridge::HandleSafeboxSize(uint8_t size) {
        auto result = m_pythonRuntime.Invoke("game", "OpenSafeboxWindow", std::to_string(size));
        return result.has_value();
    }

    bool PhaseGameSafeboxBridge::HandleSafeboxMoneyChange([[maybe_unused]] int32_t money) {
        // Python UI fetches the money from CPythonSafeBox singleton directly, no args needed.
        auto result = m_pythonRuntime.Invoke("game", "RefreshSafeboxMoney");
        return result.has_value();
    }

    bool PhaseGameSafeboxBridge::HandleMallOpen(uint8_t size) {
        auto result = m_pythonRuntime.Invoke("game", "OpenMallWindow", std::to_string(size));
        return result.has_value();
    }

} // namespace Client::Bridge
