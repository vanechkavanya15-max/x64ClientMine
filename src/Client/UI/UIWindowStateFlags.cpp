#include "UIWindowStateFlags.h"

namespace Client::UI {

UIWindowStateFlags::UIWindowStateFlags() 
    : m_flags(WindowFlag::Show | WindowFlag::Enabled), m_alpha(1.0f) {}

StateResult<void> UIWindowStateFlags::SetFlag(WindowFlag flag) noexcept {
    return InternalUpdateFlags(m_flags | flag);
}

StateResult<void> UIWindowStateFlags::RemoveFlag(WindowFlag flag) noexcept {
    return InternalUpdateFlags(m_flags & ~flag);
}

StateResult<void> UIWindowStateFlags::ToggleFlag(WindowFlag flag) noexcept {
    return InternalUpdateFlags(m_flags ^ flag);
}

StateResult<void> UIWindowStateFlags::SetFlags(WindowFlag flags) noexcept {
    return InternalUpdateFlags(flags);
}

bool UIWindowStateFlags::HasFlag(WindowFlag flag) const noexcept {
    return (m_flags & flag) == flag;
}

WindowFlag UIWindowStateFlags::GetFlags() const noexcept {
    return m_flags;
}

bool UIWindowStateFlags::IsVisible() const noexcept {
    return HasFlag(WindowFlag::Show);
}

bool UIWindowStateFlags::IsEnabled() const noexcept {
    return HasFlag(WindowFlag::Enabled);
}

StateResult<void> UIWindowStateFlags::SetAlpha(float alpha) noexcept {
    if (HasFlag(WindowFlag::Locked)) {
        return std::unexpected(WindowStateError::Locked);
    }
    
    float newAlpha = alpha;
    if (newAlpha < 0.0f) newAlpha = 0.0f;
    if (newAlpha > 1.0f) newAlpha = 1.0f;

    m_alpha = newAlpha;
    if (m_alpha < 0.01f && HasFlag(WindowFlag::Show)) {
         return InternalUpdateFlags(m_flags & ~WindowFlag::Show);
    }
    return {};
}

float UIWindowStateFlags::GetAlpha() const noexcept {
    return m_alpha;
}

void UIWindowStateFlags::AddObserver(uint32_t id, WindowObserverCallback callback) noexcept {
    m_observers[id] = std::move(callback);
}

void UIWindowStateFlags::RemoveObserver(uint32_t id) noexcept {
    m_observers.erase(id);
}

void UIWindowStateFlags::SaveSnapshot(uint32_t snapshotId, uint64_t timestamp) noexcept {
    m_snapshots[snapshotId] = {m_flags, m_alpha, timestamp};
}

StateResult<void> UIWindowStateFlags::RestoreSnapshot(uint32_t snapshotId) noexcept {
    if (HasFlag(WindowFlag::Locked)) {
        return std::unexpected(WindowStateError::Locked);
    }

    auto it = m_snapshots.find(snapshotId);
    if (it == m_snapshots.end()) {
        return std::unexpected(WindowStateError::SnapshotNotFound);
    }

    WindowFlag oldFlags = m_flags;
    m_flags = it->second.Flags;
    m_alpha = it->second.Alpha;

    NotifyObservers(oldFlags, m_flags);
    return {};
}

StateResult<void> UIWindowStateFlags::InternalUpdateFlags(WindowFlag newFlags) noexcept {
    if (HasFlag(WindowFlag::Locked)) {
        // If it's already locked, we can only unlock it.
        // If the new flags don't have locked, it's an unlock attempt.
        if ((newFlags & WindowFlag::Locked) == WindowFlag::Locked && m_flags != newFlags) {
             return std::unexpected(WindowStateError::Locked);
        }
    }

    if (!ValidateTransition(m_flags, newFlags)) {
        return std::unexpected(WindowStateError::TransitionConstraintViolated);
    }

    WindowFlag oldFlags = m_flags;
    m_flags = newFlags;
    
    if (m_flags != oldFlags) {
        NotifyObservers(oldFlags, m_flags);
    }

    return {};
}

bool UIWindowStateFlags::ValidateTransition(WindowFlag currentFlags, WindowFlag newFlags) const noexcept {
    // Cannot be Hovered and NotPick at the same time
    if ((newFlags & WindowFlag::Hovered) == WindowFlag::Hovered && 
        (newFlags & WindowFlag::NotPick) == WindowFlag::NotPick) {
        return false;
    }

    // Cannot be Dragable if Restricted
    if ((newFlags & WindowFlag::Dragable) == WindowFlag::Dragable && 
        (newFlags & WindowFlag::Restricted) == WindowFlag::Restricted) {
        return false;
    }

    // Cannot be Pressed if NotEnabled or NotShow
    if ((newFlags & WindowFlag::Pressed) == WindowFlag::Pressed) {
        if ((newFlags & WindowFlag::Enabled) != WindowFlag::Enabled || 
            (newFlags & WindowFlag::Show) != WindowFlag::Show) {
            return false;
        }
    }
    
    // Cannot be Focused if NotEnabled or NotShow
    if ((newFlags & WindowFlag::Focused) == WindowFlag::Focused) {
        if ((newFlags & WindowFlag::Enabled) != WindowFlag::Enabled || 
            (newFlags & WindowFlag::Show) != WindowFlag::Show) {
            return false;
        }
    }

    return true;
}

void UIWindowStateFlags::NotifyObservers(WindowFlag oldFlags, WindowFlag newFlags) const noexcept {
    for (const auto& [id, callback] : m_observers) {
        if (callback) {
            callback(oldFlags, newFlags);
        }
    }
}


void UIWindowStateManager::RegisterWindow(WindowId id) noexcept {
    if (m_windows.find(id) == m_windows.end()) {
        m_windows[id] = std::make_unique<UIWindowStateFlags>();
    }
}

void UIWindowStateManager::UnregisterWindow(WindowId id) noexcept {
    m_windows.erase(id);
    
    // Clean up modal stack
    std::erase(m_modalStack, id);
    std::erase(m_bulkHiddenWindows, id);
}

UIWindowStateFlags* UIWindowStateManager::GetWindowFlags(WindowId id) noexcept {
    auto it = m_windows.find(id);
    return it != m_windows.end() ? it->second.get() : nullptr;
}

const UIWindowStateFlags* UIWindowStateManager::GetWindowFlags(WindowId id) const noexcept {
    auto it = m_windows.find(id);
    return it != m_windows.end() ? it->second.get() : nullptr;
}

StateResult<void> UIWindowStateManager::PushGlobalModal(WindowId id) noexcept {
    auto* flags = GetWindowFlags(id);
    if (!flags) {
        return std::unexpected(WindowStateError::WindowNotFound);
    }

    flags->SetFlag(WindowFlag::Modal | WindowFlag::Show | WindowFlag::TopMost);
    m_modalStack.push_back(id);
    
    // Disable input on all other windows
    for (auto& [winId, winFlags] : m_windows) {
        if (winId != id) {
             // We don't modify their actual Enabled flag if possible, 
             // but here we simply remove Enabled. A real implementation might use a separate "input blocked" state.
             winFlags->RemoveFlag(WindowFlag::Enabled);
        }
    }
    
    return {};
}

StateResult<void> UIWindowStateManager::PopGlobalModal() noexcept {
    if (m_modalStack.empty()) {
        return std::unexpected(WindowStateError::WindowNotFound);
    }
    
    WindowId poppedId = m_modalStack.back();
    m_modalStack.pop_back();
    
    auto* flags = GetWindowFlags(poppedId);
    if (flags) {
        flags->RemoveFlag(WindowFlag::Modal | WindowFlag::TopMost);
    }
    
    // Re-enable input if no other modals
    if (m_modalStack.empty()) {
        for (auto& [winId, winFlags] : m_windows) {
             winFlags->SetFlag(WindowFlag::Enabled);
        }
    } else {
        WindowId currentModal = m_modalStack.back();
        auto* currentModalFlags = GetWindowFlags(currentModal);
        if (currentModalFlags) {
            currentModalFlags->SetFlag(WindowFlag::Enabled);
        }
    }
    
    return {};
}

WindowId UIWindowStateManager::GetActiveGlobalModal() const noexcept {
    if (m_modalStack.empty()) {
        return 0; // Assuming 0 is an invalid WindowId
    }
    return m_modalStack.back();
}

void UIWindowStateManager::BulkHideAll() noexcept {
    m_bulkHiddenWindows.clear();
    for (auto& [winId, winFlags] : m_windows) {
        if (winFlags->IsVisible()) {
            winFlags->RemoveFlag(WindowFlag::Show);
            m_bulkHiddenWindows.push_back(winId);
        }
    }
}

void UIWindowStateManager::BulkShowAll() noexcept {
    for (WindowId winId : m_bulkHiddenWindows) {
        auto* flags = GetWindowFlags(winId);
        if (flags) {
            flags->SetFlag(WindowFlag::Show);
        }
    }
    m_bulkHiddenWindows.clear();
}

} // namespace Client::UI