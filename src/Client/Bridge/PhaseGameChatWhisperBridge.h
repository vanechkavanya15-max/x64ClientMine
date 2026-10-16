#pragma once
#include <cstdint>
#include <string>
#include <vector>
#include <unordered_set>
#include <algorithm>

namespace Client::Bridge {

enum class ChatType : uint8_t {
    Talk = 0,
    Info = 1,
    Notice = 2,
    Party = 3,
    Guild = 4,
    Shout = 5,
    Whisper = 6,
    Command = 7
};

struct ChatMessage {
    ChatType type{ChatType::Talk};
    std::string sender{};
    std::string text{};
    uint64_t timestamp{0};
};

class PhaseGameChatWhisperBridge {
public:
    PhaseGameChatWhisperBridge() = default;
    ~PhaseGameChatWhisperBridge() = default;

    void AppendMessage(ChatType type, const std::string& sender, const std::string& text);
    [[nodiscard]] size_t GetMessageCount() const noexcept { return m_messages.size(); }
    [[nodiscard]] const std::vector<ChatMessage>& GetMessages() const noexcept { return m_messages; }
    void ClearMessages() noexcept { m_messages.clear(); }

    // Word filter
    void AddBlockedWord(const std::string& word) { m_blockedWords.insert(word); }
    [[nodiscard]] bool ContainsBlockedWord(const std::string& text) const;
    [[nodiscard]] std::string SanitizeMessage(const std::string& text) const;

    // Whisper target tracking
    void SetLastWhisperTarget(const std::string& target) { m_lastWhisperTarget = target; }
    [[nodiscard]] const std::string& GetLastWhisperTarget() const noexcept { return m_lastWhisperTarget; }

private:
    std::vector<ChatMessage> m_messages;
    std::unordered_set<std::string> m_blockedWords;
    std::string m_lastWhisperTarget{};
};

} // namespace Client::Bridge
