#pragma once

#include <cstdint>
#include <string>
#include <string_view>
#include <vector>
#include <expected>
#include <format>
#include "Client/Core/StrongTypes.h"

namespace Client::Events {

using EventIndex = Core::StrongType<struct EventIndexTag, uint32_t, 0>;
using AnswerIndex = Core::StrongType<struct AnswerIndexTag, uint8_t, 0>;

enum class EventAnswerError : uint8_t {
    EventNotActive,
    InvalidAnswerIndex,
    ChoiceAlreadyExists,
    EmptyChoiceText
};

[[nodiscard]] constexpr std::string_view to_string(EventAnswerError error) noexcept {
    switch (error) {
        case EventAnswerError::EventNotActive: return "EventNotActive";
        case EventAnswerError::InvalidAnswerIndex: return "InvalidAnswerIndex";
        case EventAnswerError::ChoiceAlreadyExists: return "ChoiceAlreadyExists";
        case EventAnswerError::EmptyChoiceText: return "EmptyChoiceText";
    }
    return "UnknownEventAnswerError";
}

struct AnswerChoice {
    AnswerIndex index;
    std::string text;
};

class EventAnswerChoiceManager {
public:
    EventAnswerChoiceManager() = default;

    std::expected<void, EventAnswerError> SetActiveEvent(EventIndex eventIndex);
    void ClearActiveEvent();

    std::expected<void, EventAnswerError> AddChoice(AnswerIndex answerIndex, std::string_view text);
    std::expected<std::string_view, EventAnswerError> GetChoiceText(AnswerIndex answerIndex) const;
    std::vector<AnswerChoice> GetChoices() const;

private:
    std::expected<void, EventAnswerError> EnsureActiveEvent() const;

    bool m_hasActiveEvent{false};
    EventIndex m_activeEventIndex;
    std::vector<AnswerChoice> m_choices;
};

} // namespace Client::Events

template <>
struct std::formatter<Client::Events::EventAnswerError> : std::formatter<std::string_view> {
    auto format(Client::Events::EventAnswerError err, std::format_context& ctx) const {
        return std::formatter<std::string_view>::format(Client::Events::to_string(err), ctx);
    }
};
