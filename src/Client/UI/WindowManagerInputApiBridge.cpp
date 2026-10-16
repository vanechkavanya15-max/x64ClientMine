#include "WindowManagerInputApiBridge.h"

namespace Client::UI {

std::expected<void, InputError> WindowManagerInputApiBridge::SetCapture(uint32_t windowId) noexcept {
    if (m_captureWindowId.has_value() && m_captureWindowId.value() != windowId) {
        return std::unexpected(InputError::AlreadyCaptured);
    }
    m_captureWindowId = windowId;
    return {};
}

std::expected<void, InputError> WindowManagerInputApiBridge::ReleaseCapture() noexcept {
    if (!m_captureWindowId.has_value()) {
        return std::unexpected(InputError::NotCaptured);
    }
    m_captureWindowId.reset();
    return {};
}

void WindowManagerInputApiBridge::SetFocus(uint32_t windowId) noexcept {
    if (!m_focusStack.empty() && m_focusStack.back() == windowId) {
        return; // Already focused
    }

    // Remove from stack if already exists to push to top
    auto it = std::find(m_focusStack.begin(), m_focusStack.end(), windowId);
    if (it != m_focusStack.end()) {
        m_focusStack.erase(it);
    }
    
    m_focusStack.push_back(windowId);
}

void WindowManagerInputApiBridge::KillFocus() noexcept {
    if (!m_focusStack.empty()) {
        m_focusStack.pop_back();
    }
}

void WindowManagerInputApiBridge::KillFocus(uint32_t windowId) noexcept {
    auto it = std::find(m_focusStack.begin(), m_focusStack.end(), windowId);
    if (it != m_focusStack.end()) {
        m_focusStack.erase(it);
    }
}

std::optional<uint32_t> WindowManagerInputApiBridge::GetFocusWindow() const noexcept {
    if (m_focusStack.empty()) {
        return std::nullopt;
    }
    return m_focusStack.back();
}

void WindowManagerInputApiBridge::AttachIcon(uint32_t type, uint32_t slotNumber, uint32_t itemVnum, uint32_t count, std::optional<uint32_t> sourceWindowId) {
    m_attachingIcon.type = type;
    m_attachingIcon.slotNumber = slotNumber;
    m_attachingIcon.itemVnum = itemVnum;
    m_attachingIcon.count = count;
    m_attachingIcon.sourceWindowId = sourceWindowId;
    m_attachingIcon.isAttaching = true;
}

void WindowManagerInputApiBridge::DeattachIcon() noexcept {
    m_attachingIcon.isAttaching = false;
    m_attachingIcon.type = 0;
    m_attachingIcon.slotNumber = 0;
    m_attachingIcon.itemVnum = 0;
    m_attachingIcon.count = 0;
    m_attachingIcon.sourceWindowId.reset();
}

void WindowManagerInputApiBridge::BeginDrag(uint32_t windowId, MouseButton button) noexcept {
    m_dragContext.isDragging = true;
    m_dragContext.sourceWindowId = windowId;
    m_dragContext.startPosition = m_mousePos;
    m_dragContext.button = button;
}

void WindowManagerInputApiBridge::EndDrag() noexcept {
    m_dragContext.isDragging = false;
    m_dragContext.sourceWindowId = 0;
    m_dragContext.startPosition = {0, 0};
    m_dragContext.button = MouseButton::Unknown;
}

double WindowManagerInputApiBridge::CalculateDragDistance() const noexcept {
    if (!m_dragContext.isDragging) {
        return 0.0;
    }
    double dx = static_cast<double>(m_mousePos.x - m_dragContext.startPosition.x);
    double dy = static_cast<double>(m_mousePos.y - m_dragContext.startPosition.y);
    return std::sqrt(dx * dx + dy * dy);
}

std::expected<void, InputError> WindowManagerInputApiBridge::OnButtonDown(uint32_t windowId, MouseButton button, uint64_t timestampMs) noexcept {
    if (IsInputLocked(timestampMs)) {
        return std::unexpected(InputError::InputLocked);
    }
    
    auto allowed = CheckInputAllowed(m_mousePos);
    if (!allowed) {
        return std::unexpected(allowed.error());
    }

    if (button == MouseButton::Left) {
        m_leftClickData.windowId = windowId;
    } else if (button == MouseButton::Right) {
        m_rightClickData.windowId = windowId;
    }

    return {};
}

std::expected<bool, InputError> WindowManagerInputApiBridge::OnButtonUp(uint32_t windowId, MouseButton button, uint64_t timestampMs) noexcept {
    if (IsInputLocked(timestampMs)) {
        return std::unexpected(InputError::InputLocked);
    }
    
    auto allowed = CheckInputAllowed(m_mousePos);
    if (!allowed) {
        return std::unexpected(allowed.error());
    }

    bool isDoubleClick = false;
    if (button == MouseButton::Left) {
        if (m_leftClickData.windowId == windowId && (timestampMs - m_leftClickData.lastClickTime) <= DOUBLE_CLICK_THRESHOLD_MS) {
            m_leftClickData.clickCount++;
            if (m_leftClickData.clickCount >= 2) {
                isDoubleClick = true;
                m_leftClickData.clickCount = 0;
            }
        } else {
            m_leftClickData.clickCount = 1;
        }
        m_leftClickData.lastClickTime = timestampMs;
        m_leftClickData.windowId = windowId;
    } else if (button == MouseButton::Right) {
        if (m_rightClickData.windowId == windowId && (timestampMs - m_rightClickData.lastClickTime) <= DOUBLE_CLICK_THRESHOLD_MS) {
            m_rightClickData.clickCount++;
            if (m_rightClickData.clickCount >= 2) {
                isDoubleClick = true;
                m_rightClickData.clickCount = 0;
            }
        } else {
            m_rightClickData.clickCount = 1;
        }
        m_rightClickData.lastClickTime = timestampMs;
        m_rightClickData.windowId = windowId;
    }

    return isDoubleClick;
}

void WindowManagerInputApiBridge::LockInput(uint64_t durationMs, uint64_t currentTimestampMs) noexcept {
    m_inputLockEndTime = currentTimestampMs + durationMs;
}

bool WindowManagerInputApiBridge::IsInputLocked(uint64_t currentTimestampMs) const noexcept {
    return currentTimestampMs < m_inputLockEndTime;
}

std::expected<void, InputError> WindowManagerInputApiBridge::CheckInputAllowed(const Point2D& point) const noexcept {
    if (m_restrictionRect.has_value() && !m_restrictionRect.value().Contains(point)) {
        return std::unexpected(InputError::OutsideRestriction);
    }
    return {};
}

void WindowManagerInputApiBridge::OnWindowDestroyed(uint32_t windowId) noexcept {
    KillFocus(windowId);
    
    if (m_captureWindowId.has_value() && m_captureWindowId.value() == windowId) {
        ReleaseCapture();
    }
    
    if (m_attachingIcon.isAttaching && m_attachingIcon.sourceWindowId.has_value() && m_attachingIcon.sourceWindowId.value() == windowId) {
        DeattachIcon();
    }

    if (m_dragContext.isDragging && m_dragContext.sourceWindowId == windowId) {
        EndDrag();
    }
}

} // namespace Client::UI
