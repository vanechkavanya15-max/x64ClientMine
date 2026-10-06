#pragma once

#include <cstdint>
#include <span>
#include <string_view>

namespace Network::Handlers
{
#pragma pack(push, 1)
    /**
     * @brief Naglowek pakietu odebranej wiadomosci czatu (TPacketGCChat).
     */
    struct PacketChatHeader
    {
        uint8_t header;
        uint16_t size;
        uint8_t type;
        uint32_t senderId;
    };
#pragma pack(pop)

    /**
     * @brief Przetwarza odebrana wiadomosc czatu i kieruje ja do bufora logiki.
     * @param buffer Pelny bufor strumienia wiadomosci czatu.
     * @return true w przypadku powodzenia.
     */
    bool HandleChatMessage(std::span<const uint8_t> buffer);
}
