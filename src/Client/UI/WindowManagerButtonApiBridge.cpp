#include "WindowManagerButtonApiBridge.h"

namespace Client::UI {

bool WindowManagerButtonApiBridge::RegisterButton(uint32_t buttonId, ButtonType type, uint32_t group) {
    auto& btn = m_buttons[buttonId];
    btn.buttonId = buttonId;
    btn.type = type;
    btn.groupIndex = group;
    btn.state = ButtonVisualState::Normal;
    btn.isEnabled = true;
    return true;
}

bool WindowManagerButtonApiBridge::SetButtonType(uint32_t buttonId, ButtonType type) {
    auto it = m_buttons.find(buttonId);
    if (it == m_buttons.end()) return false;
    it->second.type = type;
    return true;
}

bool WindowManagerButtonApiBridge::SetButtonState(uint32_t buttonId, ButtonVisualState state) {
    auto it = m_buttons.find(buttonId);
    if (it == m_buttons.end()) return false;
    it->second.state = state;
    return true;
}

bool WindowManagerButtonApiBridge::SetButtonDown(uint32_t buttonId, bool down) {
    auto it = m_buttons.find(buttonId);
    if (it == m_buttons.end()) return false;
    it->second.isDown = down;
    it->second.state = down ? ButtonVisualState::Down : ButtonVisualState::Normal;
    return true;
}

bool WindowManagerButtonApiBridge::IsButtonDown(uint32_t buttonId) const {
    auto it = m_buttons.find(buttonId);
    return it != m_buttons.end() ? it->second.isDown : false;
}

bool WindowManagerButtonApiBridge::SetButtonEnable(uint32_t buttonId, bool enable) {
    auto it = m_buttons.find(buttonId);
    if (it == m_buttons.end()) return false;
    it->second.isEnabled = enable;
    it->second.state = enable ? ButtonVisualState::Normal : ButtonVisualState::Disable;
    return true;
}

bool WindowManagerButtonApiBridge::IsButtonEnabled(uint32_t buttonId) const {
    auto it = m_buttons.find(buttonId);
    return it != m_buttons.end() ? it->second.isEnabled : false;
}

bool WindowManagerButtonApiBridge::SetButtonFlash(uint32_t buttonId, bool flash) {
    auto it = m_buttons.find(buttonId);
    if (it == m_buttons.end()) return false;
    it->second.isFlashing = flash;
    return true;
}

bool WindowManagerButtonApiBridge::IsButtonFlashing(uint32_t buttonId) const {
    auto it = m_buttons.find(buttonId);
    return it != m_buttons.end() ? it->second.isFlashing : false;
}

bool WindowManagerButtonApiBridge::SetButtonTooltip(uint32_t buttonId, const std::string& tooltip) {
    auto it = m_buttons.find(buttonId);
    if (it == m_buttons.end()) return false;
    it->second.tooltipText = tooltip;
    return true;
}

std::string WindowManagerButtonApiBridge::GetButtonTooltip(uint32_t buttonId) const {
    auto it = m_buttons.find(buttonId);
    return it != m_buttons.end() ? it->second.tooltipText : "";
}

void WindowManagerButtonApiBridge::PressRadioButton(uint32_t buttonId) {
    auto it = m_buttons.find(buttonId);
    if (it == m_buttons.end() || it->second.type != ButtonType::Radio) return;
    uint32_t group = it->second.groupIndex;
    for (auto& [id, btn] : m_buttons) {
        if (btn.type == ButtonType::Radio && btn.groupIndex == group) {
            btn.isDown = (id == buttonId);
            btn.state = btn.isDown ? ButtonVisualState::Down : ButtonVisualState::Normal;
        }
    }
}

} // namespace Client::UI
