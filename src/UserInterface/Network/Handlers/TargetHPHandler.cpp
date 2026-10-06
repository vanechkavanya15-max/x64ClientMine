#include "TargetHPHandler.h"
#include "../../StdAfx.h"
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

        // Aktualizacja modelu celu bezposrednio w CPythonPlayer
        CPythonPlayer::Instance().SetTargetHP(packet->targetVid, packet->hpPercentage);

        return true;
    }
}
