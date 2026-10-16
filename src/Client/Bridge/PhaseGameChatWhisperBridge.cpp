#include "PhaseGameChatWhisperBridge.h"

namespace Client::Bridge {

void PhaseGameChatWhisperBridge::AppendMessage(ChatType type, const std::string& sender, const std::string& text) {
    std::string sanitized = SanitizeMessage(text);
    m_messages.push_back({type, sender, sanitized, 0});
    if (type == ChatType::Whisper && !sender.empty()) {
        m_lastWhisperTarget = sender;
    }
}

bool PhaseGameChatWhisperBridge::ContainsBlockedWord(const std::string& text) const {
    for (const auto& w : m_blockedWords) {
        if (!w.empty() && text.find(w) != std::string::npos) {
            return true;
        }
    }
    return false;
}

std::string PhaseGameChatWhisperBridge::SanitizeMessage(const std::string& text) const {
    std::string result = text;
    for (const auto& w : m_blockedWords) {
        if (w.empty()) continue;
        size_t pos = 0;
        while ((pos = result.find(w, pos)) != std::string::npos) {
            result.replace(pos, w.length(), std::string(w.length(), '*'));
            pos += w.length();
        }
    }
    return result;
}

} // namespace Client::Bridge
