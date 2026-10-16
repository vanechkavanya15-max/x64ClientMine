#pragma once

#include "WindowHierarchy.h"
#include <EterBase/Result.h>
#include <cstdint>
#include <string_view>

namespace Client::UI {

enum class LayoutError : uint8_t {
    None = 0,
    NullContainer,
    NoChildren,
    InvalidGridColumns
};

[[nodiscard]] constexpr std::string_view ToString(LayoutError err) noexcept {
    switch (err) {
        case LayoutError::None: return "None";
        case LayoutError::NullContainer: return "NullContainer";
        case LayoutError::NoChildren: return "NoChildren";
        case LayoutError::InvalidGridColumns: return "InvalidGridColumns";
    }
    return "UnknownLayoutError";
}

enum class LayoutDirection : uint8_t {
    Horizontal,
    Vertical
};

enum class LayoutAlignment : uint8_t {
    Start,
    Center,
    End
};

struct LayoutConfig {
    int spacing = 0;
    int padding = 0;
    LayoutAlignment alignment = LayoutAlignment::Start;
};

/**
 * @brief Menedzer ukladu okien UI.
 * Klasa odpowiada za liniowe i siatkowe ukladanie elementow podrzednych.
 * Zapewnia brak bezposrednich zaleznosci od Windows/DirectX API (C++23 Zero-Conflict).
 */
class UIWindowLayoutManager {
public:
    static EterBase::Result<void, LayoutError> ArrangeLinear(Window* container, LayoutDirection direction, const LayoutConfig& config = {});
    static EterBase::Result<void, LayoutError> ArrangeGrid(Window* container, uint32_t columns, const LayoutConfig& config = {});
    static EterBase::Result<void, LayoutError> CenterInParent(Window* child);
    static EterBase::Result<void, LayoutError> FillParent(Window* child, int padding = 0);
};

} // namespace Client::UI

template <>
struct std::formatter<Client::UI::LayoutError> : std::formatter<std::string_view> {
    auto format(Client::UI::LayoutError err, std::format_context& ctx) const {
        return std::formatter<std::string_view>::format(Client::UI::ToString(err), ctx);
    }
};
