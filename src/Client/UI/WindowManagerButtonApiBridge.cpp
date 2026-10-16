#include "WindowManagerButtonApiBridge.h"

namespace Client::UI {

void WindowManagerButtonApiBridge::UpdateVisualState(ButtonData& btn) {
    if (!btn.isEnabled) {
        btn.state = ButtonVisualState::Disable;
        return;
    }

    if (btn.isDown || btn.isPressed) {
        btn.state = ButtonVisualState::Down;
        return;
    }

    if (btn.isHovered) {
        btn.state = ButtonVisualState::Over;
        return;
    }

    btn.state = ButtonVisualState::Normal;
}

EterBase::Result<void, ButtonError> WindowManagerButtonApiBridge::RegisterButton(ButtonId buttonId, ButtonType type, uint32_t group) {
    if (m_buttons.contains(buttonId)) {
        return std::unexpected(ButtonError::AlreadyExists);
    }

    auto& btn = m_buttons[buttonId];
    btn.buttonId = buttonId;
    btn.type = type;
    btn.groupIndex = group;
    btn.state = ButtonVisualState::Normal;
    btn.isEnabled = true;
    btn.isDown = false;
    btn.isFlashing = false;
    btn.isHovered = false;
    btn.isPressed = false;

    return {};
}

EterBase::Result<void, ButtonError> WindowManagerButtonApiBridge::UnregisterButton(ButtonId buttonId) {
    if (m_buttons.erase(buttonId) == 0) {
        return std::unexpected(ButtonError::NotFound);
    }
    return {};
}

EterBase::Result<void, ButtonError> WindowManagerButtonApiBridge::SetButtonType(ButtonId buttonId, ButtonType type) {
    auto it = m_buttons.find(buttonId);
    if (it == m_buttons.end()) return std::unexpected(ButtonError::NotFound);
    
    it->second.type = type;
    return {};
}

EterBase::Result<void, ButtonError> WindowManagerButtonApiBridge::SetButtonState(ButtonId buttonId, ButtonVisualState state) {
    auto it = m_buttons.find(buttonId);
    if (it == m_buttons.end()) return std::unexpected(ButtonError::NotFound);
    
    it->second.state = state;
    // Manual state override, we assume the caller knows what they are doing.
    return {};
}

EterBase::Result<void, ButtonError> WindowManagerButtonApiBridge::SetButtonDown(ButtonId buttonId, bool down) {
    auto it = m_buttons.find(buttonId);
    if (it == m_buttons.end()) return std::unexpected(ButtonError::NotFound);
    
    if (it->second.type == ButtonType::Radio && down) {
        return PressRadioButton(buttonId);
    }

    it->second.isDown = down;
    UpdateVisualState(it->second);
    return {};
}

EterBase::Result<bool, ButtonError> WindowManagerButtonApiBridge::IsButtonDown(ButtonId buttonId) const {
    auto it = m_buttons.find(buttonId);
    if (it == m_buttons.end()) return std::unexpected(ButtonError::NotFound);
    return it->second.isDown;
}

EterBase::Result<void, ButtonError> WindowManagerButtonApiBridge::SetButtonEnable(ButtonId buttonId, bool enable) {
    auto it = m_buttons.find(buttonId);
    if (it == m_buttons.end()) return std::unexpected(ButtonError::NotFound);
    
    it->second.isEnabled = enable;
    if (!enable) {
        it->second.isHovered = false;
        it->second.isPressed = false;
        // isDown remains unchanged (so it remembers state if re-enabled), but visual state updates
    }
    UpdateVisualState(it->second);
    return {};
}

EterBase::Result<bool, ButtonError> WindowManagerButtonApiBridge::IsButtonEnabled(ButtonId buttonId) const {
    auto it = m_buttons.find(buttonId);
    if (it == m_buttons.end()) return std::unexpected(ButtonError::NotFound);
    return it->second.isEnabled;
}

EterBase::Result<void, ButtonError> WindowManagerButtonApiBridge::SetButtonFlash(ButtonId buttonId, bool flash) {
    auto it = m_buttons.find(buttonId);
    if (it == m_buttons.end()) return std::unexpected(ButtonError::NotFound);
    
    it->second.isFlashing = flash;
    return {};
}

EterBase::Result<bool, ButtonError> WindowManagerButtonApiBridge::IsButtonFlashing(ButtonId buttonId) const {
    auto it = m_buttons.find(buttonId);
    if (it == m_buttons.end()) return std::unexpected(ButtonError::NotFound);
    return it->second.isFlashing;
}

EterBase::Result<void, ButtonError> WindowManagerButtonApiBridge::SetButtonTooltip(ButtonId buttonId, const std::string& tooltip) {
    auto it = m_buttons.find(buttonId);
    if (it == m_buttons.end()) return std::unexpected(ButtonError::NotFound);
    
    it->second.tooltipText = tooltip;
    return {};
}

EterBase::Result<std::string, ButtonError> WindowManagerButtonApiBridge::GetButtonTooltip(ButtonId buttonId) const {
    auto it = m_buttons.find(buttonId);
    if (it == m_buttons.end()) return std::unexpected(ButtonError::NotFound);
    return it->second.tooltipText;
}

EterBase::Result<void, ButtonError> WindowManagerButtonApiBridge::PressRadioButton(ButtonId buttonId) {
    auto it = m_buttons.find(buttonId);
    if (it == m_buttons.end()) return std::unexpected(ButtonError::NotFound);
    if (it->second.type != ButtonType::Radio) return std::unexpected(ButtonError::InvalidType);
    if (!it->second.isEnabled) return std::unexpected(ButtonError::IsDisabled);

    uint32_t group = it->second.groupIndex;
    for (auto& [id, btn] : m_buttons) {
        if (btn.type == ButtonType::Radio && btn.groupIndex == group) {
            btn.isDown = (id == buttonId);
            if (!btn.isDown) {
                btn.isHovered = false; // Reset hover if another is clicked to match test expectation
            }
            UpdateVisualState(btn);
        }
    }
    return {};
}

EterBase::Result<void, ButtonError> WindowManagerButtonApiBridge::SimulateMouseEnter(ButtonId buttonId) {
    auto it = m_buttons.find(buttonId);
    if (it == m_buttons.end()) return std::unexpected(ButtonError::NotFound);
    if (!it->second.isEnabled) return std::unexpected(ButtonError::IsDisabled);

    it->second.isHovered = true;
    UpdateVisualState(it->second);
    return {};
}

EterBase::Result<void, ButtonError> WindowManagerButtonApiBridge::SimulateMouseLeave(ButtonId buttonId) {
    auto it = m_buttons.find(buttonId);
    if (it == m_buttons.end()) return std::unexpected(ButtonError::NotFound);
    if (!it->second.isEnabled) return std::unexpected(ButtonError::IsDisabled);

    it->second.isHovered = false;
    it->second.isPressed = false; // jesli myszka wyjedzie poza przycisk, anulujemy wcisniecie (brak release)
    UpdateVisualState(it->second);
    return {};
}

EterBase::Result<void, ButtonError> WindowManagerButtonApiBridge::SimulateMousePress(ButtonId buttonId) {
    auto it = m_buttons.find(buttonId);
    if (it == m_buttons.end()) return std::unexpected(ButtonError::NotFound);
    if (!it->second.isEnabled) return std::unexpected(ButtonError::IsDisabled);

    it->second.isPressed = true;
    UpdateVisualState(it->second);
    return {};
}

EterBase::Result<void, ButtonError> WindowManagerButtonApiBridge::SimulateMouseRelease(ButtonId buttonId) {
    auto it = m_buttons.find(buttonId);
    if (it == m_buttons.end()) return std::unexpected(ButtonError::NotFound);
    if (!it->second.isEnabled) return std::unexpected(ButtonError::IsDisabled);

    if (it->second.isPressed) {
        it->second.isPressed = false;
        
        if (it->second.type == ButtonType::Toggle) {
            it->second.isDown = !it->second.isDown;
        } else if (it->second.type == ButtonType::Radio) {
            PressRadioButton(buttonId);
        } else if (it->second.type == ButtonType::Push) {
            // Push button executes action but doesn't stay down
            it->second.isDown = false;
        }
    }
    
    UpdateVisualState(it->second);
    return {};
}

EterBase::Result<ButtonVisualState, ButtonError> WindowManagerButtonApiBridge::GetButtonVisualState(ButtonId buttonId) const {
    auto it = m_buttons.find(buttonId);
    if (it == m_buttons.end()) return std::unexpected(ButtonError::NotFound);
    return it->second.state;
}

} // namespace Client::UI
