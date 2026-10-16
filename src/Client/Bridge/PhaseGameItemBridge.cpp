#include "PhaseGameItemBridge.h"
#include <string>

namespace Client::Bridge {

    bool PhaseGameItemBridge::HandleItemSet(const TPacketGCItemSet& pack) {
        m_pythonRuntime.Invoke("player", "SetItemData", std::to_string(pack.pos.cell), std::to_string(pack.vnum), std::to_string(pack.count));
        return true;
    }

    bool PhaseGameItemBridge::HandleItemDel(const TPacketGCItemDel& pack) {
        m_pythonRuntime.Invoke("player", "SetItemCount", std::to_string(pack.pos.cell), "0");
        return true;
    }

    bool PhaseGameItemBridge::HandleItemOwnership(const TPacketGCItemOwnership& pack) {
        // Find null terminator or use full length
        std::string_view nameView(pack.szName, sizeof(pack.szName));
        auto nullPos = nameView.find('\0');
        if (nullPos != std::string_view::npos) {
            nameView = nameView.substr(0, nullPos);
        }
        m_pythonRuntime.Invoke("item", "SetOwnership", std::to_string(pack.dwVID), std::string(nameView));
        return true;
    }

    bool PhaseGameItemBridge::HandleItemGroundAdd(const TPacketGCItemGroundAdd& pack) {
        m_pythonRuntime.Invoke("item", "AddItem", std::to_string(pack.dwVID), std::to_string(pack.dwVnum), std::to_string(pack.lX), std::to_string(pack.lY), std::to_string(pack.lZ));
        return true;
    }

    bool PhaseGameItemBridge::HandleItemGroundDel(const TPacketGCItemGroundDel& pack) {
        m_pythonRuntime.Invoke("item", "DelItem", std::to_string(pack.vid));
        return true;
    }

} // namespace Client::Bridge
