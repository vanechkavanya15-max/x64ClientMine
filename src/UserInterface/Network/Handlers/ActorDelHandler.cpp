#include "../../StdAfx.h"
#include "../../PythonNetworkStream.h"
#include "../../NetworkActorManager.h"
#include <cstdint>

/**
 * @brief Handler for character deletion from the world (e.g., going out of range or dying).
 * @return true on successful parsing and deletion, false on failure to read the packet.
 */
bool CPythonNetworkStream::RecvCharacterDeletePacket()
{
    TPacketGCCharacterDelete packet;

    if (!Recv(sizeof(packet), &packet))
    {
        TraceError("CPythonNetworkStream::RecvCharacterDeletePacket - Packet read error");
        return false;
    }

    uint32_t targetId = packet.dwVID;

    // Remove from C++ Subsystem
    if (!m_rokNetActorMgr.IsNull())
    {
        m_rokNetActorMgr->RemoveActor(targetId);
    }

    // Decoupling from direct UI calls: We do not call PyCallClassMemberFunc here anymore.
    // Instead, the CharacterManager or Python UI loop should poll or react to the C++ memory state.

    return true;
}

/**
 * @brief Beavium protocol specific version for character deletion.
 * @return true on successful parsing and deletion, false on failure to read the packet.
 */
bool CPythonNetworkStream::__RecvCharacterDelBeavium()
{
    Beavium::TPacketGCCharacterDelBeavium packet;
    
    if (!Recv(sizeof(packet), &packet))
    {
        TraceError("[BEAVIUM] RecvCharacterDel - PACKET READ ERROR");
        return false;
    }

    uint32_t targetId = packet.dwVID;

    // Remove from C++ Subsystem
    if (!m_rokNetActorMgr.IsNull())
    {
        m_rokNetActorMgr->RemoveActor(targetId);
    }

    // Decoupling from direct UI calls: We do not call PyCallClassMemberFunc here anymore.

    return true;
}
