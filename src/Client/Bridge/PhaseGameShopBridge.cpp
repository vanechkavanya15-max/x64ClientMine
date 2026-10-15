#include "PhaseGameShopBridge.h"
#include "Client/Network/ShopPacketCodec.h"
#include "Client/Network/Handlers/ShopOpenPacketHandler.h"
#include <cstring>
#include <string>

namespace Client::Bridge {

    bool PhaseGameShopBridge::HandleShop(const TPacketGCShop& headerPack, std::span<const uint8_t> payload) {
        switch (headerPack.subheader) {
            case ShopSub::GC::START:
                return HandleShopStart(payload);
            case ShopSub::GC::START_EX:
                return HandleShopStartEx(payload);
            case ShopSub::GC::END:
                return HandleShopEnd();
            case ShopSub::GC::UPDATE_ITEM:
                return HandleShopUpdateItem(payload);
            case ShopSub::GC::UPDATE_PRICE:
                return HandleShopUpdatePrice(payload);
            case ShopSub::GC::NOT_ENOUGH_MONEY:
                m_pythonRuntime.Invoke("game", "OnShopError", "NOT_ENOUGH_MONEY");
                return true;
            case ShopSub::GC::NOT_ENOUGH_MONEY_EX:
                m_pythonRuntime.Invoke("game", "OnShopError", "NOT_ENOUGH_MONEY_EX");
                return true;
            case ShopSub::GC::SOLDOUT:
                m_pythonRuntime.Invoke("game", "OnShopError", "SOLDOUT");
                return true;
            case ShopSub::GC::INVENTORY_FULL:
                m_pythonRuntime.Invoke("game", "OnShopError", "INVENTORY_FULL");
                return true;
            case ShopSub::GC::INVALID_POS:
                m_pythonRuntime.Invoke("game", "OnShopError", "INVALID_POS");
                return true;
            default:
                return true;
        }
    }

    bool PhaseGameShopBridge::HandleShopStart(std::span<const uint8_t> payload) {
        auto res = Client::Network::Handlers::ShopOpenPacketHandler::HandleShopStart(payload, m_shopModel);
        if (!res.has_value()) return false;
        
        m_pythonRuntime.Invoke("game", "StartShop", std::to_string(res.value().ownerVid));
        return true;
    }

    bool PhaseGameShopBridge::HandleShopStartEx(std::span<const uint8_t> payload) {
        auto res = Client::Network::Handlers::ShopOpenPacketHandler::HandleShopStartEx(payload, m_shopModel);
        if (!res.has_value()) return false;
        
        m_pythonRuntime.Invoke("game", "StartShop", std::to_string(res.value().ownerVid));
        return true;
    }

    bool PhaseGameShopBridge::HandleShopEnd() {
        m_pythonRuntime.Invoke("game", "EndShop");
        return true;
    }

    bool PhaseGameShopBridge::HandleShopUpdateItem(std::span<const uint8_t> payload) {
        auto res = Client::Network::ShopPacketCodec::DecodeShopUpdateItem(payload);
        if (!res.has_value()) return false;

        auto updatePacket = res.value();

        UserInterface::Domain::ShopItemData domainItem{};
        domainItem.vnum = EterBase::ItemVnum(updatePacket.item.vnum);
        domainItem.count = updatePacket.item.count;
        domainItem.price = updatePacket.item.price;

        for (size_t s = 0; s < ITEM_SOCKET_SLOT_MAX_NUM; ++s) {
            domainItem.sockets.push_back(updatePacket.item.alSockets[s]);
        }

        for (size_t a = 0; a < ITEM_ATTRIBUTE_SLOT_MAX_NUM; ++a) {
            domainItem.attributes.push_back(UserInterface::Domain::ShopItemAttribute{
                static_cast<uint16_t>(updatePacket.item.aAttr[a].bType), 
                updatePacket.item.aAttr[a].sValue
            });
        }
        
        auto setRes = m_shopModel.setItem(0, EterBase::ItemSlot(updatePacket.pos), domainItem);
        if (!setRes.has_value()) return false;

        m_pythonRuntime.Invoke("game", "RefreshShop");
        return true;
    }

    bool PhaseGameShopBridge::HandleShopUpdatePrice(std::span<const uint8_t> payload) {
        constexpr size_t expected_size = sizeof(TPacketGCShop) + sizeof(int);
        if (payload.size() < expected_size) return false;

        int price;
        std::memcpy(&price, payload.data() + sizeof(TPacketGCShop), sizeof(int));
        
        m_pythonRuntime.Invoke("game", "SetShopSellingPrice", std::to_string(price));
        return true;
    }

    bool PhaseGameShopBridge::HandleShopSign(const TPacketGCShopSign& pack, bool isMainPlayer, bool isShopOpen) {
        std::string_view sign(pack.szSign, strnlen(pack.szSign, sizeof(pack.szSign)));
        if (sign.empty()) {
            m_pythonRuntime.Invoke("game", "BINARY_PrivateShop_Disappear", std::to_string(pack.dwVID));
            if (isMainPlayer && isShopOpen) {
                 // Additional python invoke if needed, but keeping bridge isolated
            }
        } else {
            m_pythonRuntime.Invoke("game", "BINARY_PrivateShop_Appear", std::to_string(pack.dwVID), std::string(sign));
        }
        return true;
    }

} // namespace Client::Bridge
