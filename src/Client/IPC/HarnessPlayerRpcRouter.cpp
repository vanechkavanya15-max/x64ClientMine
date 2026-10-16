#include "HarnessPlayerRpcRouter.h"

namespace Client::IPC {

bool HarnessPlayerRpcRouter::RouteMove(float x, float y) {
    m_history.push_back({"move", 0, 0, x, y, ""});
    return true;
}

bool HarnessPlayerRpcRouter::RouteAttack(uint32_t targetVid) {
    m_history.push_back({"attack", targetVid, 0, 0.0f, 0.0f, ""});
    return true;
}

bool HarnessPlayerRpcRouter::RouteUseItem(uint32_t slot) {
    m_history.push_back({"use_item", 0, slot, 0.0f, 0.0f, ""});
    return true;
}

bool HarnessPlayerRpcRouter::RouteSendChat(const std::string& msg) {
    m_history.push_back({"chat", 0, 0, 0.0f, 0.0f, msg});
    return true;
}

} // namespace Client::IPC
