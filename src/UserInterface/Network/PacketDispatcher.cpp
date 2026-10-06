#include "../StdAfx.h"
#include "PacketDispatcher.h"
#include "../Packet.h"

// Handlery domenowe Fali 1
#include "Handlers/DeadHandler.h"
#include "Handlers/DamageHandler.h"
#include "Handlers/TargetHPHandler.h"
#include "Handlers/ChatHandler.h"
#include "Handlers/ItemGroundDelHandler.h"
#include "Handlers/CharPositionHandler.h"

namespace Network
{
    PacketDispatcher& PacketDispatcher::Instance()
    {
        static PacketDispatcher instance;
        return instance;
    }

    void PacketDispatcher::RegisterHandler(uint16_t opcode, PacketHandlerFn handler)
    {
        m_handlers[opcode] = handler;
    }

    bool PacketDispatcher::HasHandler(uint16_t opcode) const
    {
        return m_handlers.find(opcode) != m_handlers.end();
    }

    bool PacketDispatcher::Dispatch(uint16_t opcode, std::span<const uint8_t> payload)
    {
        auto it = m_handlers.find(opcode);
        if (it != m_handlers.end() && it->second)
        {
            return it->second(payload);
        }
        return false;
    }

    void PacketDispatcher::RegisterDefaultHandlers()
    {
        // Rejestracja modulow domenowych Fali 1 (Standard C++20 na x64)
        RegisterHandler(GC::DEAD, Handlers::HandleDeadPacket);
        RegisterHandler(GC::DAMAGE_INFO, Handlers::HandleDamagePacket);
        RegisterHandler(GC::TARGET, Handlers::HandleTargetHP);
        RegisterHandler(GC::CHAT, Handlers::HandleChatMessage);
        RegisterHandler(GC::ITEM_GROUND_DEL, Handlers::HandleItemGroundDel);
        RegisterHandler(GC::CHARACTER_POSITION, Handlers::HandleCharacterPosition);
    }
}
