#pragma once

#include <cstdint>
#include <span>

namespace Network::Handlers
{
#pragma pack(push, 1)
    /**
     * @brief Struktura pakietu aktualizacji punktow zycia celu (TPacketGCTarget).
     */
    struct PacketTargetHP
    {
        uint8_t header;
        uint32_t targetVid;
        uint8_t hpPercentage;
    };
#pragma pack(pop)

    /**
     * @brief Przetwarza pakiet aktualizacji poziomu HP celu.
     * @param buffer Bufor bajtow pakietu.
     * @return true w przypadku poprawnego przetworzenia.
     */
    bool HandleTargetHP(std::span<const uint8_t> buffer);
}
