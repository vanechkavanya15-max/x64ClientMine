#include "UIWindowEventListenerDispatch.h"

namespace Client::UI {

std::expected<uint64_t, DispatchError> UIWindowEventListenerDispatch::RegisterHandler(WindowId windowId, UIEventType evt, EventHandler handler, int32_t priority) {
    if (!windowId) return std::unexpected(DispatchError::InvalidWindowId);
    if (!handler) return std::unexpected(DispatchError::HandlerNotFound);

    uint64_t id = m_nextHandlerId++;
    auto& eventList = m_windowHandlers[windowId].eventHandlers[evt];
    eventList.push_back(EventRegistration{id, std::move(handler), priority});
    
    // Sort descending by priority
    std::stable_sort(eventList.begin(), eventList.end(), 
        [](const EventRegistration& a, const EventRegistration& b) {
            return a.priority > b.priority;
        });

    return id;
}

std::expected<void, DispatchError> UIWindowEventListenerDispatch::UnregisterHandler(WindowId windowId, UIEventType evt, uint64_t handlerId) {
    if (!windowId) return std::unexpected(DispatchError::InvalidWindowId);
    
    auto windowIt = m_windowHandlers.find(windowId);
    if (windowIt != m_windowHandlers.end()) {
        auto evtIt = windowIt->second.eventHandlers.find(evt);
        if (evtIt != windowIt->second.eventHandlers.end()) {
            std::erase_if(evtIt->second, [handlerId](const EventRegistration& reg) { return reg.handlerId == handlerId; });
            if (evtIt->second.empty()) {
                windowIt->second.eventHandlers.erase(evtIt);
            }
        }
        if (windowIt->second.eventHandlers.empty()) {
            m_windowHandlers.erase(windowIt);
        }
    }
    return {};
}

void UIWindowEventListenerDispatch::ClearAllHandlers(WindowId windowId) {
    if (windowId) {
        m_windowHandlers.erase(windowId);
    }
}

bool UIWindowEventListenerDispatch::ValidateEventPayload(UIEventType evt, const EventPayload& payload) const {
    switch (evt) {
        case UIEventType::OnClick:
        case UIEventType::OnDoubleClick:
        case UIEventType::OnMouseOver:
        case UIEventType::OnMouseLeave:
            return std::holds_alternative<MouseEventData>(payload) || std::holds_alternative<std::monostate>(payload);
        case UIEventType::OnKeyDown:
        case UIEventType::OnKeyUp:
            return std::holds_alternative<KeyEventData>(payload) || std::holds_alternative<std::monostate>(payload);
        case UIEventType::OnChar:
            return std::holds_alternative<CharEventData>(payload) || std::holds_alternative<std::monostate>(payload);
    }
    return false;
}

std::expected<bool, DispatchError> UIWindowEventListenerDispatch::DispatchEvent(WindowId windowId, UIEventType evt, const EventPayload& payload) {
    if (!windowId) return std::unexpected(DispatchError::InvalidWindowId);
    if (!ValidateEventPayload(evt, payload)) return std::unexpected(DispatchError::PayloadMismatch);

    auto windowIt = m_windowHandlers.find(windowId);
    if (windowIt == m_windowHandlers.end()) return false;

    auto eventIt = windowIt->second.eventHandlers.find(evt);
    if (eventIt == windowIt->second.eventHandlers.end() || eventIt->second.empty()) return false;

    // Execute handlers in priority order until one consumes the event
    for (const auto& reg : eventIt->second) {
        if (reg.handler(windowId, payload)) {
            return true; // Consumed
        }
    }

    return false;
}

std::expected<bool, DispatchError> UIWindowEventListenerDispatch::BubbleEvent(WindowId windowId, UIEventType evt, const UIWindowHierarchyTree& hierarchy, const EventPayload& payload) {
    if (!windowId) return std::unexpected(DispatchError::InvalidWindowId);
    if (!ValidateEventPayload(evt, payload)) return std::unexpected(DispatchError::PayloadMismatch);

    WindowId currentId = windowId;
    std::vector<WindowId> visited;

    while (currentId) {
        if (std::find(visited.begin(), visited.end(), currentId) != visited.end()) {
            return std::unexpected(DispatchError::CycleDetected);
        }
        visited.push_back(currentId);

        auto result = DispatchEvent(currentId, evt, payload);
        if (!result.has_value()) {
            return std::unexpected(result.error());
        }
        
        if (result.value()) {
            return true; // Event consumed during bubbling
        }

        auto parentOpt = hierarchy.GetParent(currentId);
        if (parentOpt.has_value()) {
            currentId = parentOpt.value();
        } else {
            break;
        }
    }

    return false; // Event not consumed
}

} // namespace Client::UI
