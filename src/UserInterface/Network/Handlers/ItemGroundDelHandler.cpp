#include "../../StdAfx.h"
#include "ItemGroundDelHandler.h"
#include "../../PythonItem.h"

namespace Network::Handlers
{
    bool HandleItemGroundDel(std::span<const uint8_t> buffer)
    {
        if (buffer.size() < sizeof(PacketItemGroundDel))
        {
            return false;
        }

        const auto* packet = reinterpret_cast<const PacketItemGroundDel*>(buffer.data());

        // Usuniecie przedmiotu ze swiata gry
        CPythonItem::Instance().DeleteItemGroundInstance(packet->itemVid);

        return true;
    }
}
