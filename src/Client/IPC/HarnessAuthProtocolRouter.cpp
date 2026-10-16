#include "HarnessAuthProtocolRouter.h"

namespace Client::IPC {

bool HarnessAuthProtocolRouter::SetCredentials(const std::string& user, const std::string& pass) {
    if (user.empty()) return false;
    m_user = user;
    m_pass = pass;
    return true;
}

bool HarnessAuthProtocolRouter::SelectSlot(uint8_t slot) {
    if (slot > 4) return false;
    m_slot = slot;
    return true;
}

} // namespace Client::IPC
