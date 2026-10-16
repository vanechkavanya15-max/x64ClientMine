#pragma once
#include <string>
#include <vector>

namespace Client::Bridge {

struct ChatMessage {
    uint8_t type{0};
    std::string sender{};
    std::string text{};
};

class PhaseGameChatWhisperBridge {
public:
    PhaseGameChatWhisperBridge() = default;
    ~PhaseGameChatWhisperBridge() = default;

    void AppendMessage(uint8_t type, const std::string& sender, const std::string& text);
    [[nodiscard]] size_t GetMessageCount() const noexcept { return m_messages.size(); }
    [[nodiscard]] const std::vector<ChatMessage>& GetMessages() const noexcept { return m_messages; }

private:
    std::vector<ChatMessage> m_messages;
};

} // namespace Client::Bridge
