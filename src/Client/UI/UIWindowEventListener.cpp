#include "UIWindowEventListener.h"

namespace Client::UI {

EterBase::Result<void, UIListenerError> UIWindowEventListener::RegisterMouseListener(Window* window, MouseEventCallback callback) {
    if (!window) {
        return EterBase::MakeError(UIListenerError::WindowNullptr);
    }
    if (m_mouseListeners.contains(window)) {
        return EterBase::MakeError(UIListenerError::AlreadyRegistered);
    }
    m_mouseListeners.emplace(window, std::move(callback));
    return {};
}

EterBase::Result<void, UIListenerError> UIWindowEventListener::RegisterDragDropListener(Window* window, DragDropCallback callback) {
    if (!window) {
        return EterBase::MakeError(UIListenerError::WindowNullptr);
    }
    if (m_dragDropListeners.contains(window)) {
        return EterBase::MakeError(UIListenerError::AlreadyRegistered);
    }
    m_dragDropListeners.emplace(window, std::move(callback));
    return {};
}

EterBase::Result<void, UIListenerError> UIWindowEventListener::UnregisterMouseListener(Window* window) {
    if (!window) {
        return EterBase::MakeError(UIListenerError::WindowNullptr);
    }
    if (!m_mouseListeners.erase(window)) {
        return EterBase::MakeError(UIListenerError::NotFound);
    }
    return {};
}

EterBase::Result<void, UIListenerError> UIWindowEventListener::UnregisterDragDropListener(Window* window) {
    if (!window) {
        return EterBase::MakeError(UIListenerError::WindowNullptr);
    }
    if (!m_dragDropListeners.erase(window)) {
        return EterBase::MakeError(UIListenerError::NotFound);
    }
    return {};
}

bool UIWindowEventListener::DispatchMouseEvent(Window* window, const MouseEvent& event) const {
    if (!window) return false;
    auto it = m_mouseListeners.find(window);
    if (it != m_mouseListeners.end() && it->second) {
        return it->second(window, event);
    }
    return false;
}

bool UIWindowEventListener::DispatchDragDropEvent(Window* target, Window* source, const DragDropPayload& payload) const {
    if (!target) return false;
    auto it = m_dragDropListeners.find(target);
    if (it != m_dragDropListeners.end() && it->second) {
        return it->second(target, source, payload);
    }
    return false;
}

EterBase::Result<void, UIListenerError> UIWindowEventListener::ClearListeners(Window* window) {
    if (!window) {
        return EterBase::MakeError(UIListenerError::WindowNullptr);
    }
    
    bool removedMouse = m_mouseListeners.erase(window) > 0;
    bool removedDragDrop = m_dragDropListeners.erase(window) > 0;

    if (!removedMouse && !removedDragDrop) {
        return EterBase::MakeError(UIListenerError::NotFound);
    }
    return {};
}

void UIWindowEventListener::ClearAll() {
    m_mouseListeners.clear();
    m_dragDropListeners.clear();
}

} // namespace Client::UI
