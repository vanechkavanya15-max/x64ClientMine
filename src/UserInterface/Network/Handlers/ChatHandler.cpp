#include "../../StdAfx.h"
#include "ChatHandler.h"
#include "../../PythonChat.h"
#include <string>

namespace Network::Handlers
{
    bool HandleChatMessage(std::span<const uint8_t> buffer)
    {
        if (buffer.size() < sizeof(PacketChatHeader))
        {
            return false;
        }

        const auto* packet = reinterpret_cast<const PacketChatHeader*>(buffer.data());
        size_t messageLen = buffer.size() - sizeof(PacketChatHeader);

        if (messageLen == 0)
        {
            return true;
        }

        const char* textPtr = reinterpret_cast<const char*>(buffer.data() + sizeof(PacketChatHeader));
        std::string message(textPtr, messageLen);

        // Zapis do logiki czatu (odseparowane od okna GUI)
        CPythonChat::Instance().AppendChat(packet->type, message.c_str());

        return true;
    }
}
