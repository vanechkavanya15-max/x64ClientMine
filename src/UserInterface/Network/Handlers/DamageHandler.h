#pragma once

#include <cstdint>
#include <span>

namespace Network::Handlers
{
#pragma pack(push, 1)
    /**
     * @brief Struktura pakietu otrzymanych lub zadanych obrazen (TPacketGCDamageInfo).
     */
    struct PacketDamageInfo
    {
        uint8_t header;
        uint32_t victimVid;
        uint8_t damageFlag;
        int32_t damageValue;
    };
#pragma pack(pop)

    /**
     * @brief Przetwarza pakiet obrazen postaci.
     * @param buffer Bufor bajtow ze strumienia sieciowego.
     * @return true w przypadku poprawnego odczytania pakietu.
     */
    bool HandleDamagePacket(std::span<const uint8_t> buffer);
}
