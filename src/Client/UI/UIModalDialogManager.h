#pragma once

#include <cstdint>
#include <vector>
#include <algorithm>
#include <expected>
#include <optional>
#include <string_view>
#include <format>
#include "EterBase/StrongTypes.h"
#include "EterBase/EventBus.h"
#include "Client/Core/Result.h"

namespace Client::UI {

// Silny typ dla identyfikatora okna dialogowego
struct DialogIdTag {};
using DialogId = EterBase::StrongType<DialogIdTag, uint32_t, 0>;

// Błędy domenowe dla okien modalnych
enum class UIError : uint8_t {
    DialogAlreadyActive,
    DialogNotFound,
    NoActiveDialog,
    InvalidDialogId
};

// Pomocnicza funkcja formatowania błędu
[[nodiscard]] constexpr std::string_view to_string(UIError error) noexcept {
    switch (error) {
        case UIError::DialogAlreadyActive: return "DialogAlreadyActive";
        case UIError::DialogNotFound: return "DialogNotFound";
        case UIError::NoActiveDialog: return "NoActiveDialog";
        case UIError::InvalidDialogId: return "InvalidDialogId";
        default: return "UnknownUIError";
    }
}

// Zdarzenie wysyłane przy otwarciu okna dialogowego
struct ModalDialogPushedEvent : public EterBase::IEvent {
    DialogId dialogId;
    explicit ModalDialogPushedEvent(DialogId id = DialogId{0}) : dialogId(id) {}
};

// Zdarzenie wysyłane przy zamknięciu okna dialogowego
struct ModalDialogPoppedEvent : public EterBase::IEvent {
    DialogId dialogId;
    explicit ModalDialogPoppedEvent(DialogId id = DialogId{0}) : dialogId(id) {}
};

/**
 * @brief Menedżer okien dialogowych (Headless, C++23)
 * Zarządza stosem aktywnych dialogów i nadaje zdarzenia do EventBus.
 */
class UIModalDialogManager {
public:
    UIModalDialogManager() = default;
    ~UIModalDialogManager() = default;

    // Blokada kopiowania/przenoszenia dla menedżera
    UIModalDialogManager(const UIModalDialogManager&) = delete;
    UIModalDialogManager& operator=(const UIModalDialogManager&) = delete;
    UIModalDialogManager(UIModalDialogManager&&) = delete;
    UIModalDialogManager& operator=(UIModalDialogManager&&) = delete;

    /**
     * @brief Otwiera nowe okno dialogowe i wrzuca je na stos
     */
    Client::Core::Result<void, UIError> PushDialog(DialogId id);

    /**
     * @brief Zamyka okno dialogowe. Jeśli podano ID, sprawdza czy jest na szczycie.
     */
    Client::Core::Result<void, UIError> PopDialog(std::optional<DialogId> expectedId = std::nullopt);

    /**
     * @brief Zwraca aktualnie aktywne okno dialogowe (szczyt stosu)
     */
    Client::Core::Result<DialogId, UIError> GetActiveDialog() const noexcept;

    /**
     * @brief Sprawdza czy okno o danym ID jest aktualnie aktywne (gdziekolwiek na stosie)
     */
    [[nodiscard]] bool IsDialogActive(DialogId id) const noexcept;

    /**
     * @brief Zwraca liczbę otwartych okien dialogowych
     */
    [[nodiscard]] size_t GetActiveDialogCount() const noexcept { return m_activeDialogs.size(); }

    /**
     * @brief Zamyka wszystkie otwarte okna
     */
    void ClearAll() noexcept;

private:
    std::vector<DialogId> m_activeDialogs;
};

} // namespace Client::UI

// Specjalizacja std::formatter dla UIError
template <>
struct std::formatter<Client::UI::UIError> : std::formatter<std::string_view> {
    auto format(Client::UI::UIError err, std::format_context& ctx) const {
        return std::formatter<std::string_view>::format(Client::UI::to_string(err), ctx);
    }
};
