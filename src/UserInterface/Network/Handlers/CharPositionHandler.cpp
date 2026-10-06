#include "../../StdAfx.h"
#include "CharPositionHandler.h"
#include "../../PythonCharacterManager.h"

namespace Network::Handlers
{
    bool HandleCharacterPosition(std::span<const uint8_t> buffer)
    {
        if (buffer.size() < sizeof(PacketCharPosition))
        {
            return false;
        }

        const auto* packet = reinterpret_cast<const PacketCharPosition*>(buffer.data());

        auto* instance = CPythonCharacterManager::Instance().GetInstancePtr(packet->characterVid);
        if (instance)
        {
            instance->NEW_SetPixelPosition(TPixelPosition(static_cast<float>(packet->x), static_cast<float>(packet->y), 0.0f));
        }

        return true;
    }
}
