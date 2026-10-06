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

    PacketDispatcher::PacketDispatcher()
    {
        m_handlers.fill(nullptr);
    }

    void PacketDispatcher::RegisterHandler(uint8_t opcode, PacketHandlerFn handler)
    {
        m_handlers[opcode] = handler;
    }

    bool PacketDispatcher::HasHandler(uint8_t opcode) const
    {
        return m_handlers[opcode] != nullptr;
    }

    bool PacketDispatcher::Dispatch(uint8_t opcode, std::span<const uint8_t> payload)
    {
        auto handler = m_handlers[opcode];
        if (handler)
        {
            return handler(payload);
        }
        return false;
    }

    void PacketDispatcher::RegisterDefaultHandlers()
    {
        // Rejestracja modulow domenowych Fali 1 (Standard C++20)
        RegisterHandler(HEADER_GC_DEAD, Handlers::HandleDeadPacket);
        RegisterHandler(HEADER_GC_DAMAGE_INFO, Handlers::HandleDamagePacket);
        RegisterHandler(HEADER_GC_TARGET, Handlers::HandleTargetHP);
        RegisterHandler(HEADER_GC_CHAT, Handlers::HandleChatMessage);
        RegisterHandler(HEADER_GC_ITEM_GROUND_DEL, Handlers::HandleItemGroundDel);
        RegisterHandler(HEADER_GC_CHARACTER_POSITION, Handlers::HandleCharacterPosition);
    }
}
