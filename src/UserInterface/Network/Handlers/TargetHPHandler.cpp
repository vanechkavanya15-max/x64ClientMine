#include "../../StdAfx.h"
#include "TargetHPHandler.h"
#include "../../PythonPlayer.h"

namespace Network::Handlers
{
    bool HandleTargetHP(std::span<const uint8_t> buffer)
    {
        if (buffer.size() < sizeof(PacketTargetHP))
        {
            return false;
        }

        const auto* packet = reinterpret_cast<const PacketTargetHP*>(buffer.data());

        // Aktualizacja celu w CPythonPlayer
        CPythonPlayer::Instance().SetTarget(packet->targetVid);

        return true;
    }
}
