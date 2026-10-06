#pragma once

#include <cstdint>
#include <span>

namespace Network::Handlers
{
#pragma pack(push, 1)
    /**
     * @brief Struktura pakietu oznaczajacego smierc encji (wczesniej TPacketGCDead).
     */
    struct PacketDead
    {
        uint16_t header;
        uint16_t length;
        uint32_t targetId;
    };
#pragma pack(pop)

    /**
     * @brief Zglasza smierc postaci na podstawie pakietu sieciowego i czysci cel walki.
     * @param buffer Bufor z danymi pakietu.
     * @return Zwraca true w przypadku poprawnego przetworzenia pakietu, w przeciwnym razie false.
     */
    bool HandleDeadPacket(std::span<const uint8_t> buffer);
}
