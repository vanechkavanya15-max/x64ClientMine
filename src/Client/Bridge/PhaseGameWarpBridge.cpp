#include "PhaseGameWarpBridge.h"
#include <string>

namespace Client::Bridge {

    bool PhaseGameWarpBridge::HandleWarp(const TPacketGCWarp& pack) {
        if (pack.wPort == 0) {
            Warp(pack.lX, pack.lY);
        } else {
            m_pythonRuntime.Invoke("network", "ConnectToWarpServer", pack.lAddr, pack.wPort);
        }
        return true;
    }

    void PhaseGameWarpBridge::Warp(int32_t globalX, int32_t globalY) {
        m_pythonRuntime.Invoke("background", "Destroy");
        m_pythonRuntime.Invoke("background", "Create");
        m_pythonRuntime.Invoke("background", "Warp", globalX, globalY);
        m_pythonRuntime.Invoke("background", "RefreshShadowLevel");

        // The background logic handles coordinate conversion normally but since this is an isolated C++ layer, 
        // we'll rely on the Python runtime or application layer to handle exact local translation via an event/call.
        // For the sake of this architectural refactoring, we proxy the necessary business parity to Python.
        m_pythonRuntime.Invoke("app", "SetCenterPositionLocal", globalX, globalY);

        m_pythonRuntime.Invoke("character", "DeleteAllInstances");
        m_pythonRuntime.Invoke("item", "DeleteAllItems");
        m_pythonRuntime.Invoke("texttail", "Clear");
        m_pythonRuntime.Invoke("effect", "DeleteAllInstances");

        m_pythonRuntime.Invoke("player", "StopAllAction");
    }

    void PhaseGameWarpBridge::ShowMapName(const std::string& mapName, int32_t localX, int32_t localY) {
        m_pythonRuntime.Invoke("game", "ShowMapName", mapName, localX, localY);
    }

    void PhaseGameWarpBridge::SetGamePhase() {
        m_pythonRuntime.Invoke("network", "SetGamePhase");
    }

    void PhaseGameWarpBridge::LeaveGamePhase() {
        m_pythonRuntime.Invoke("character", "Destroy");
        m_pythonRuntime.Invoke("item", "Destroy");
        m_pythonRuntime.Invoke("network", "ClearActorManager");
        m_pythonRuntime.Invoke("player", "ClearPVPKeySystem");
    }

} // namespace Client::Bridge
