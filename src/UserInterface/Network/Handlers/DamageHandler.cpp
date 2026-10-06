#include "../../StdAfx.h"
#include "DamageHandler.h"
#include "../../PythonCharacterManager.h"
#include "../../PythonPlayer.h"

namespace Network::Handlers
{
    bool HandleDamagePacket(std::span<const uint8_t> buffer)
    {
        if (buffer.size() < sizeof(PacketDamageInfo))
        {
            return false;
        }

        const auto* packet = reinterpret_cast<const PacketDamageInfo*>(buffer.data());

        auto& charMgr = CPythonCharacterManager::Instance();
        auto* victimInstance = charMgr.GetInstancePtr(packet->victimVid);

        if (victimInstance)
        {
            // Aktualizacja wizualna i logiczna efektu uderzenia/obrazen
            bool bSelf = (victimInstance == charMgr.GetMainInstancePtr());
            victimInstance->AddDamageEffect(packet->damageValue, packet->damageFlag, bSelf, false);
        }

        return true;
    }
}
