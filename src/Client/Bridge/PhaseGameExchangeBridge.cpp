#include "PhaseGameExchangeBridge.h"

namespace Client::Bridge {

    bool PhaseGameExchangeBridge::HandleExchangePacket(const TPacketGCExchange& pack) {
        switch (pack.subheader) {
            case ExchangeSub::GC::START:    return HandleExchangeSub_Start(pack);
            case ExchangeSub::GC::ITEM_ADD: return HandleExchangeSub_ItemAdd(pack);
            case ExchangeSub::GC::ITEM_DEL: return HandleExchangeSub_ItemDel(pack);
            case ExchangeSub::GC::ELK_ADD:  return HandleExchangeSub_ElkAdd(pack);
            case ExchangeSub::GC::ACCEPT:   return HandleExchangeSub_Accept(pack);
            case ExchangeSub::GC::END:      return HandleExchangeSub_End(pack);
            case ExchangeSub::GC::ALREADY:  return HandleExchangeSub_Already(pack);
            case ExchangeSub::GC::LESS_ELK: return HandleExchangeSub_LessElk(pack);
            default: return true;
        }
    }

    bool PhaseGameExchangeBridge::HandleExchangeSub_Start(const TPacketGCExchange& pack) {
        m_pythonRuntime.Invoke("exchange", "StartExchange", std::to_string(pack.arg1));
        return true;
    }

    bool PhaseGameExchangeBridge::HandleExchangeSub_ItemAdd(const TPacketGCExchange& pack) {
        if (pack.is_me) {
            m_pythonRuntime.Invoke("exchange", "SetItemToSelf", std::to_string(pack.arg2.cell), std::to_string(pack.arg1), std::to_string(pack.arg3));
            for (int i = 0; i < ITEM_SOCKET_SLOT_MAX_NUM; ++i) {
                m_pythonRuntime.Invoke("exchange", "SetItemMetinSocketToSelf", std::to_string(pack.arg2.cell), std::to_string(i), std::to_string(pack.alValues[i]));
            }
            for (int j = 0; j < ITEM_ATTRIBUTE_SLOT_MAX_NUM; ++j) {
                m_pythonRuntime.Invoke("exchange", "SetItemAttributeToSelf", std::to_string(pack.arg2.cell), std::to_string(j), std::to_string(pack.aAttr[j].bType), std::to_string(pack.aAttr[j].sValue));
            }
        } else {
            m_pythonRuntime.Invoke("exchange", "SetItemToTarget", std::to_string(pack.arg2.cell), std::to_string(pack.arg1), std::to_string(pack.arg3));
            for (int i = 0; i < ITEM_SOCKET_SLOT_MAX_NUM; ++i) {
                m_pythonRuntime.Invoke("exchange", "SetItemMetinSocketToTarget", std::to_string(pack.arg2.cell), std::to_string(i), std::to_string(pack.alValues[i]));
            }
            for (int j = 0; j < ITEM_ATTRIBUTE_SLOT_MAX_NUM; ++j) {
                m_pythonRuntime.Invoke("exchange", "SetItemAttributeToTarget", std::to_string(pack.arg2.cell), std::to_string(j), std::to_string(pack.aAttr[j].bType), std::to_string(pack.aAttr[j].sValue));
            }
        }
        m_pythonRuntime.Invoke("exchange", "RefreshExchangeWindow");
        return true;
    }

    bool PhaseGameExchangeBridge::HandleExchangeSub_ItemDel(const TPacketGCExchange& pack) {
        if (pack.is_me) {
            m_pythonRuntime.Invoke("exchange", "DelItemOfSelf", std::to_string(pack.arg1));
        } else {
            m_pythonRuntime.Invoke("exchange", "DelItemOfTarget", std::to_string(pack.arg1));
        }
        m_pythonRuntime.Invoke("exchange", "RefreshExchangeWindow");
        return true;
    }

    bool PhaseGameExchangeBridge::HandleExchangeSub_ElkAdd(const TPacketGCExchange& pack) {
        if (pack.is_me) {
            m_pythonRuntime.Invoke("exchange", "SetElkToSelf", std::to_string(pack.arg1));
        } else {
            m_pythonRuntime.Invoke("exchange", "SetElkToTarget", std::to_string(pack.arg1));
        }
        m_pythonRuntime.Invoke("exchange", "RefreshExchangeWindow");
        return true;
    }

    bool PhaseGameExchangeBridge::HandleExchangeSub_Accept(const TPacketGCExchange& pack) {
        if (pack.is_me) {
            m_pythonRuntime.Invoke("exchange", "SetAcceptToSelf", std::to_string(pack.arg1));
        } else {
            m_pythonRuntime.Invoke("exchange", "SetAcceptToTarget", std::to_string(pack.arg1));
        }
        m_pythonRuntime.Invoke("exchange", "RefreshExchangeWindow");
        return true;
    }

    bool PhaseGameExchangeBridge::HandleExchangeSub_End(const TPacketGCExchange& /*pack*/) {
        m_pythonRuntime.Invoke("exchange", "EndExchange");
        return true;
    }

    bool PhaseGameExchangeBridge::HandleExchangeSub_Already(const TPacketGCExchange& /*pack*/) {
        m_pythonRuntime.Invoke("exchange", "TradeAlready");
        return true;
    }

    bool PhaseGameExchangeBridge::HandleExchangeSub_LessElk(const TPacketGCExchange& /*pack*/) {
        m_pythonRuntime.Invoke("exchange", "TradeLessElk");
        return true;
    }

} // namespace Client::Bridge
