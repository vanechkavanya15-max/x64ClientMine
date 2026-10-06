#include "../../StdAfx.h"
#include "DeadHandler.h"
#include "../../PythonCharacterManager.h"
#include "../../PythonPlayer.h"

namespace Network::Handlers
{
    bool HandleDeadPacket(std::span<const uint8_t> buffer)
    {
        if (buffer.size() < sizeof(PacketDead))
        {
            return false;
        }

        const auto* packet = reinterpret_cast<const PacketDead*>(buffer.data());
        
        auto& characterManager = CPythonCharacterManager::Instance();
        auto* targetInstance = characterManager.GetInstancePtr(packet->targetId);

        if (targetInstance)
        {
            // Oznaczenie encji jako martwej
            targetInstance->Die();

            // Wyczyszczenie celu walki, jesli to aktualny cel
            CPythonPlayer::Instance().NotifyCharacterDead(packet->targetId);

            auto* mainInstance = characterManager.GetMainInstancePtr();
            if (mainInstance == targetInstance)
            {
                // Notyfikacja gracza o wlasnej smierci (bezposrednie GUI omijane, oddelegowane do struktury logiki)
                CPythonPlayer::Instance().NotifyDeadMainCharacter();
            }
        }

        return true;
    }
}
