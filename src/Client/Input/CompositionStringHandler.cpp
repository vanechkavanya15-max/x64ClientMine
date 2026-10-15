#include "CompositionStringHandler.h"

namespace Client::Input {

std::expected<void, CompositionError> CompositionStringHandler::UpdateComposition(
    std::wstring_view composition_str,
    std::vector<uint8_t> attributes,
    std::optional<uint32_t> cursor_position
) {
    if (attributes.size() != composition_str.length()) {
        return std::unexpected(CompositionError::InvalidAttributeLength);
    }
    
    if (cursor_position.has_value() && cursor_position.value() > composition_str.length()) {
        return std::unexpected(CompositionError::InvalidCursorPosition);
    }

    state_.current_composition = composition_str;
    state_.attributes = std::move(attributes);
    state_.cursor_position = cursor_position;

    return {};
}

std::expected<void, CompositionError> CompositionStringHandler::FinalizeResult(std::wstring_view result_str) {
    state_.result_string = result_str;
    state_.current_composition.clear();
    state_.attributes.clear();
    state_.cursor_position.reset();
    
    return {};
}

void CompositionStringHandler::Clear() {
    state_ = CompositionState{};
}

const CompositionState& CompositionStringHandler::GetState() const noexcept {
    return state_;
}

} // namespace Client::Input
