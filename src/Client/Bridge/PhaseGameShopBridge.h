#pragma once

#include "Client/Bridge/IPythonRuntime.h"
#include "UserInterface/Contracts/ShopInventoryModel.h"
#include "Client/Network/Protocol/Protocol.h"
#include <span>
#include <vector>

namespace Client::Bridge {

    class PhaseGameShopBridge {
    public:
        PhaseGameShopBridge(IPythonRuntime& pythonRuntime, UserInterface::Domain::ShopInventoryModel& shopModel) noexcept
            : m_pythonRuntime(pythonRuntime), m_shopModel(shopModel) {}

        bool HandleShop(const TPacketGCShop& headerPack, std::span<const uint8_t> payload);
        bool HandleShopSign(const TPacketGCShopSign& pack, bool isMainPlayer, bool isShopOpen);

    private:
        bool HandleShopStart(std::span<const uint8_t> payload);
        bool HandleShopStartEx(std::span<const uint8_t> payload);
        bool HandleShopEnd();
        bool HandleShopUpdateItem(std::span<const uint8_t> payload);
        bool HandleShopUpdatePrice(std::span<const uint8_t> payload);

        IPythonRuntime& m_pythonRuntime;
        UserInterface::Domain::ShopInventoryModel& m_shopModel;
    };

} // namespace Client::Bridge
