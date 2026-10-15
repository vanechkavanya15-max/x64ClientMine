#include "PhaseGameCombatBridge.h"
#include <string>

namespace Client::Bridge {

    bool PhaseGameCombatBridge::HandleDamageInfo(const TPacketGCDamageInfo& pack)
    {
        // Pakiet posiada union z targetId / dwVID. Uzywamy targetId.
        auto res = m_pythonRuntime.Invoke("player", "ShowDamage", pack.targetId, pack.damage, pack.flag);
        return res.has_value();
    }

    bool PhaseGameCombatBridge::HandleDead(const TPacketGCDead& pack)
    {
        auto res = m_pythonRuntime.Invoke("player", "ShowDead", pack.targetId);
        return res.has_value();
    }

} // namespace Client::Bridge
