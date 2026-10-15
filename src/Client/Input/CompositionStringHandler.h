#pragma once

#include <string>
#include <string_view>
#include <vector>
#include <expected>
#include <optional>
#include <cstdint>

namespace Client::Input {

enum class CompositionError {
    InvalidCursorPosition,
    InvalidStringLength,
    InvalidAttributeLength
};

struct CompositionState {
    std::wstring current_composition;
    std::wstring result_string;
    std::vector<uint8_t> attributes;
    std::optional<uint32_t> cursor_position;
};

class CompositionStringHandler {
public:
    CompositionStringHandler() = default;

    std::expected<void, CompositionError> UpdateComposition(
        std::wstring_view composition_str,
        std::vector<uint8_t> attributes,
        std::optional<uint32_t> cursor_position
    );

    std::expected<void, CompositionError> FinalizeResult(std::wstring_view result_str);

    void Clear();

    [[nodiscard]] const CompositionState& GetState() const noexcept;
    
private:
    CompositionState state_;
};

} // namespace Client::Input
