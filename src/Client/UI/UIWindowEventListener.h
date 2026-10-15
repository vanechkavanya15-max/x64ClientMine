#pragma once

#include <functional>
#include <unordered_map>
#include <string_view>
#include <format>
#include <EterBase/Result.h>
#include "WindowHierarchy.h"

namespace Client::UI {

enum class UIListenerError : uint8_t {
    None = 0,
    WindowNullptr,
    AlreadyRegistered,
    NotFound
};

[[nodiscard]] constexpr std::string_view ToString(UIListenerError err) noexcept {
    switch (err) {
        case UIListenerError::None: return "None";
        case UIListenerError::WindowNullptr: return "WindowNullptr";
        case UIListenerError::AlreadyRegistered: return "AlreadyRegistered";
        case UIListenerError::NotFound: return "NotFound";
    }
    return "UnknownUIListenerError";
}

class UIWindowEventListener {
public:
    using MouseEventCallback = std::function<bool(Window*, const MouseEvent&)>;
    using DragDropCallback = std::function<bool(Window*, Window*, const DragDropPayload&)>;

    UIWindowEventListener() = default;
    ~UIWindowEventListener() = default;

    // Nie kopiujemy ani nie przenosimy (Singleton / Manager pattern)
    UIWindowEventListener(const UIWindowEventListener&) = delete;
    UIWindowEventListener& operator=(const UIWindowEventListener&) = delete;
    UIWindowEventListener(UIWindowEventListener&&) = delete;
    UIWindowEventListener& operator=(UIWindowEventListener&&) = delete;

    EterBase::Result<void, UIListenerError> RegisterMouseListener(Window* window, MouseEventCallback callback);
    EterBase::Result<void, UIListenerError> RegisterDragDropListener(Window* window, DragDropCallback callback);

    EterBase::Result<void, UIListenerError> UnregisterMouseListener(Window* window);
    EterBase::Result<void, UIListenerError> UnregisterDragDropListener(Window* window);

    bool DispatchMouseEvent(Window* window, const MouseEvent& event) const;
    bool DispatchDragDropEvent(Window* target, Window* source, const DragDropPayload& payload) const;

    EterBase::Result<void, UIListenerError> ClearListeners(Window* window);
    void ClearAll();

private:
    std::unordered_map<Window*, MouseEventCallback> m_mouseListeners;
    std::unordered_map<Window*, DragDropCallback> m_dragDropListeners;
};

} // namespace Client::UI

template <>
struct std::formatter<Client::UI::UIListenerError> : std::formatter<std::string_view> {
    auto format(Client::UI::UIListenerError err, std::format_context& ctx) const {
        return std::formatter<std::string_view>::format(Client::UI::ToString(err), ctx);
    }
};

