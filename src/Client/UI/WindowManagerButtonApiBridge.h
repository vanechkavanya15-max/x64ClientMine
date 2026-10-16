#pragma once
#include <cstdint>
#include <string>
#include <unordered_map>
#include "EterBase/Result.h"
#include "EterBase/StrongTypes.h"

namespace Client::UI {

struct ButtonIdTag {};
using ButtonId = EterBase::StrongType<ButtonIdTag, uint32_t, 0>;

enum class ButtonType : uint8_t { Push = 0, Radio = 1, Toggle = 2 };
enum class ButtonVisualState : uint8_t { Normal = 0, Over = 1, Down = 2, Disable = 3 };

enum class ButtonError : uint8_t {
    None = 0,
    NotFound,
    AlreadyExists,
    IsDisabled,
    InvalidType
};

[[nodiscard]] constexpr std::string_view ToString(ButtonError err) noexcept {
    switch (err) {
        case ButtonError::None: return "None";
        case ButtonError::NotFound: return "NotFound";
        case ButtonError::AlreadyExists: return "AlreadyExists";
        case ButtonError::IsDisabled: return "IsDisabled";
        case ButtonError::InvalidType: return "InvalidType";
    }
    return "UnknownButtonError";
}

struct ButtonData {
    ButtonId buttonId{};
    ButtonType type{ButtonType::Push};
    ButtonVisualState state{ButtonVisualState::Normal};
    bool isDown{false};
    bool isFlashing{false};
    bool isEnabled{true};
    bool isHovered{false};
    bool isPressed{false}; // Zmienna pomocnicza do sledzenia wcisniecia w trakcie hovera
    uint32_t groupIndex{0};
    std::string tooltipText{};
};

class WindowManagerButtonApiBridge {
public:
    WindowManagerButtonApiBridge() = default;
    ~WindowManagerButtonApiBridge() = default;

    EterBase::Result<void, ButtonError> RegisterButton(ButtonId buttonId, ButtonType type = ButtonType::Push, uint32_t group = 0);
    EterBase::Result<void, ButtonError> UnregisterButton(ButtonId buttonId);
    
    EterBase::Result<void, ButtonError> SetButtonType(ButtonId buttonId, ButtonType type);
    EterBase::Result<void, ButtonError> SetButtonState(ButtonId buttonId, ButtonVisualState state);
    
    EterBase::Result<void, ButtonError> SetButtonDown(ButtonId buttonId, bool down);
    [[nodiscard]] EterBase::Result<bool, ButtonError> IsButtonDown(ButtonId buttonId) const;
    
    EterBase::Result<void, ButtonError> SetButtonEnable(ButtonId buttonId, bool enable);
    [[nodiscard]] EterBase::Result<bool, ButtonError> IsButtonEnabled(ButtonId buttonId) const;
    
    EterBase::Result<void, ButtonError> SetButtonFlash(ButtonId buttonId, bool flash);
    [[nodiscard]] EterBase::Result<bool, ButtonError> IsButtonFlashing(ButtonId buttonId) const;
    
    EterBase::Result<void, ButtonError> SetButtonTooltip(ButtonId buttonId, const std::string& tooltip);
    [[nodiscard]] EterBase::Result<std::string, ButtonError> GetButtonTooltip(ButtonId buttonId) const;
    
    EterBase::Result<void, ButtonError> PressRadioButton(ButtonId buttonId);

    // Nowe API do symulacji zdarzen myszy
    EterBase::Result<void, ButtonError> SimulateMouseEnter(ButtonId buttonId);
    EterBase::Result<void, ButtonError> SimulateMouseLeave(ButtonId buttonId);
    EterBase::Result<void, ButtonError> SimulateMousePress(ButtonId buttonId);
    EterBase::Result<void, ButtonError> SimulateMouseRelease(ButtonId buttonId);

    // API do pobrania stanu wizualnego (do testow i renderera)
    [[nodiscard]] EterBase::Result<ButtonVisualState, ButtonError> GetButtonVisualState(ButtonId buttonId) const;

private:
    void UpdateVisualState(ButtonData& btn);
    std::unordered_map<ButtonId, ButtonData> m_buttons;
};

} // namespace Client::UI

template <>
struct std::formatter<Client::UI::ButtonError> : std::formatter<std::string_view> {
    auto format(Client::UI::ButtonError err, std::format_context& ctx) const {
        return std::formatter<std::string_view>::format(Client::UI::ToString(err), ctx);
    }
};
