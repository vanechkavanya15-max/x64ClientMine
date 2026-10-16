#include "PhaseGameDungeonBridge.h"
#include <string>

namespace Client::Bridge {

    bool PhaseGameDungeonBridge::HandleDungeon(const TPacketGCDungeon& pack, uint32_t destX, uint32_t destY) {
        switch (pack.subheader) {
            case DungeonSub::GC::TIME_ATTACK_START: {
                return true;
            }
            case DungeonSub::GC::DESTINATION_POSITION: {
                auto result = m_pythonRuntime.Invoke("player", "SetDungeonDestinationPosition", destX, destY);
                return result.has_value();
            }
            default:
                return false;
        }
    }

} // namespace Client::Bridge
