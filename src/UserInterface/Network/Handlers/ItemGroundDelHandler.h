#pragma once

#include <cstdint>
#include <span>

namespace Network::Handlers
{
#pragma pack(push, 1)
    /**
     * @brief Struktura pakietu usuniecia przedmiotu lezacego na ziemi (TPacketGCItemGroundDel).
     */
    struct PacketItemGroundDel
    {
        uint8_t header;
        uint32_t itemVid;
    };
#pragma pack(pop)

    /**
     * @brief Usuwa przedmiot z ziemi po podniesieniu lub zniknieciu.
     * @param buffer Bufor bajtow pakietu.
     * @return true w przypadku poprawnego usuniecia encji.
     */
    bool HandleItemGroundDel(std::span<const uint8_t> buffer);
}
