#include "PhaseGameChatWhisperBridge.h"

namespace Client::Bridge {

void PhaseGameChatWhisperBridge::AppendMessage(uint8_t type, const std::string& sender, const std::string& text) {
    m_messages.push_back({type, sender, text});
}

} // namespace Client::Bridge
