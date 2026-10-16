#include "EventAnswerChoiceManager.h"
#include <algorithm>

namespace Client::Events {

std::expected<void, EventAnswerError> EventAnswerChoiceManager::SetActiveEvent(EventIndex eventIndex) {
    m_activeEventIndex = eventIndex;
    m_hasActiveEvent = true;
    m_choices.clear();
    return {};
}

void EventAnswerChoiceManager::ClearActiveEvent() {
    m_hasActiveEvent = false;
    m_activeEventIndex = EventIndex{};
    m_choices.clear();
}

std::expected<void, EventAnswerError> EventAnswerChoiceManager::EnsureActiveEvent() const {
    if (!m_hasActiveEvent) {
        return std::unexpected(EventAnswerError::EventNotActive);
    }
    return {};
}

std::expected<void, EventAnswerError> EventAnswerChoiceManager::AddChoice(AnswerIndex answerIndex, std::string_view text) {
    auto activeEventResult = EnsureActiveEvent();
    if (!activeEventResult.has_value()) {
        return std::unexpected(activeEventResult.error());
    }

    if (text.empty()) {
        return std::unexpected(EventAnswerError::EmptyChoiceText);
    }

    auto it = std::find_if(m_choices.begin(), m_choices.end(),
        [answerIndex](const AnswerChoice& choice) { return choice.index == answerIndex; });

    if (it != m_choices.end()) {
        return std::unexpected(EventAnswerError::ChoiceAlreadyExists);
    }

    m_choices.push_back({answerIndex, std::string(text)});
    return {};
}

std::expected<std::string_view, EventAnswerError> EventAnswerChoiceManager::GetChoiceText(AnswerIndex answerIndex) const {
    auto activeEventResult = EnsureActiveEvent();
    if (!activeEventResult.has_value()) {
        return std::unexpected(activeEventResult.error());
    }

    auto it = std::find_if(m_choices.begin(), m_choices.end(),
        [answerIndex](const AnswerChoice& choice) { return choice.index == answerIndex; });

    if (it == m_choices.end()) {
        return std::unexpected(EventAnswerError::InvalidAnswerIndex);
    }

    return it->text;
}

std::vector<AnswerChoice> EventAnswerChoiceManager::GetChoices() const {
    if (!m_hasActiveEvent) {
        return {};
    }
    return m_choices;
}

} // namespace Client::Events
