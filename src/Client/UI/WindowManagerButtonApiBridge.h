#pragma once
#include <cstdint>
#include <string>
#include <unordered_map>

namespace Client::UI {

enum class ButtonType : uint8_t { Push = 0, Radio = 1, Toggle = 2 };
enum class ButtonVisualState : uint8_t { Normal = 0, Over = 1, Down = 2, Disable = 3 };

struct ButtonData {
    uint32_t buttonId{0};
    ButtonType type{ButtonType::Push};
    ButtonVisualState state{ButtonVisualState::Normal};
    bool isDown{false};
    bool isFlashing{false};
    bool isEnabled{true};
    uint32_t groupIndex{0};
    std::string tooltipText{};
};

class WindowManagerButtonApiBridge {
public:
    WindowManagerButtonApiBridge() = default;
    ~WindowManagerButtonApiBridge() = default;

    bool RegisterButton(uint32_t buttonId, ButtonType type = ButtonType::Push, uint32_t group = 0);
    bool SetButtonType(uint32_t buttonId, ButtonType type);
    bool SetButtonState(uint32_t buttonId, ButtonVisualState state);
    bool SetButtonDown(uint32_t buttonId, bool down);
    [[nodiscard]] bool IsButtonDown(uint32_t buttonId) const;
    bool SetButtonEnable(uint32_t buttonId, bool enable);
    [[nodiscard]] bool IsButtonEnabled(uint32_t buttonId) const;
    bool SetButtonFlash(uint32_t buttonId, bool flash);
    [[nodiscard]] bool IsButtonFlashing(uint32_t buttonId) const;
    bool SetButtonTooltip(uint32_t buttonId, const std::string& tooltip);
    [[nodiscard]] std::string GetButtonTooltip(uint32_t buttonId) const;
    void PressRadioButton(uint32_t buttonId);

private:
    std::unordered_map<uint32_t, ButtonData> m_buttons;
};

} // namespace Client::UI
