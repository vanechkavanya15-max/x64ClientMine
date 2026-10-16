#pragma once
#include <cstdint>
#include <functional>
#include <unordered_map>
#include <vector>
#include <variant>
#include <expected>
#include <optional>
#include <string>
#include <algorithm>

#include "Client/Core/StrongTypes.h"
#include "UIWindowHierarchyTree.h"

namespace Client::UI {

enum class UIEventType : uint8_t { 
    OnClick = 0, 
    OnDoubleClick = 1, 
    OnMouseOver = 2, 
    OnMouseLeave = 3,
    OnKeyDown = 4,
    OnKeyUp = 5,
    OnChar = 6
};

enum class DispatchError : uint8_t {
    None = 0,
    InvalidWindowId,
    HandlerNotFound,
    PayloadMismatch,
    CycleDetected
};

struct MouseEventData {
    int32_t x = 0;
    int32_t y = 0;
    uint8_t button = 0;
    bool operator==(const MouseEventData&) const = default;
};

struct KeyEventData {
    uint32_t keyCode = 0;
    bool operator==(const KeyEventData&) const = default;
};

struct CharEventData {
    char32_t character = 0;
    bool operator==(const CharEventData&) const = default;
};

using EventPayload = std::variant<std::monostate, MouseEventData, KeyEventData, CharEventData>;

struct EventRegistration {
    uint64_t handlerId = 0;
    std::function<bool(WindowId, const EventPayload&)> handler;
    int32_t priority = 0;
};

class UIWindowEventListenerDispatch {
public:
    using EventHandler = std::function<bool(WindowId, const EventPayload&)>;

    UIWindowEventListenerDispatch() = default;
    ~UIWindowEventListenerDispatch() = default;

    // Zero copy
    UIWindowEventListenerDispatch(const UIWindowEventListenerDispatch&) = delete;
    UIWindowEventListenerDispatch& operator=(const UIWindowEventListenerDispatch&) = delete;
    UIWindowEventListenerDispatch(UIWindowEventListenerDispatch&&) = default;
    UIWindowEventListenerDispatch& operator=(UIWindowEventListenerDispatch&&) = default;

    // Register a handler for a specific event type on a specific window
    std::expected<uint64_t, DispatchError> RegisterHandler(WindowId windowId, UIEventType evt, EventHandler handler, int32_t priority = 0);
    
    // Unregister handlers
    std::expected<void, DispatchError> UnregisterHandler(WindowId windowId, UIEventType evt, uint64_t handlerId);
    void ClearAllHandlers(WindowId windowId);

    // Dispatch event to a specific window, returns true if event was consumed
    std::expected<bool, DispatchError> DispatchEvent(WindowId windowId, UIEventType evt, const EventPayload& payload = std::monostate{});

    // Bubbles event up the hierarchy until consumed
    std::expected<bool, DispatchError> BubbleEvent(WindowId windowId, UIEventType evt, const UIWindowHierarchyTree& hierarchy, const EventPayload& payload = std::monostate{});

    // Validates payload matches the event type
    bool ValidateEventPayload(UIEventType evt, const EventPayload& payload) const;

private:
    struct WindowEventMap {
        std::unordered_map<UIEventType, std::vector<EventRegistration>> eventHandlers;
    };

    std::unordered_map<WindowId, WindowEventMap> m_windowHandlers;
    uint64_t m_nextHandlerId = 1;
};

} // namespace Client::UI
