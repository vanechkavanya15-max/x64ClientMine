#pragma once

#include "EterBase/EventBus.h"
#include "Client/Core/ExpectedResult.h"
#include "Client/Core/StrongTypes.h"
#include <cstdint>
#include <optional>
#include <string>
#include <string_view>

namespace Client::Events {

/**
 * @brief Identyfikator eventu (dialogu eventowego).
 */
struct DialogEventIdTag {};
using DialogEventId = Client::Core::StrongType<DialogEventIdTag, uint32_t, 0>;

/**
 * @brief Blad obslugi dialogow eventowych.
 */
enum class EventDialogError : uint8_t {
    AlreadyOpen,
    NotOpen,
    InvalidDialogId,
    ValidationFailed
};

/**
 * @brief Zdarzenie domenowe wysylane po pomyslnym otwarciu okna dialogowego eventu.
 */
struct EventDialogBoxOpenedEvent final : public EterBase::IEvent {
    DialogEventId eventId;
    std::string eventName;

    EventDialogBoxOpenedEvent(DialogEventId id, std::string_view name)
        : eventId(id), eventName(name) {}
};

/**
 * @brief Zdarzenie domenowe wysylane po zamknieciu okna dialogowego eventu.
 */
struct EventDialogBoxClosedEvent final : public EterBase::IEvent {
    DialogEventId eventId;

    explicit EventDialogBoxClosedEvent(DialogEventId id)
        : eventId(id) {}
};

/**
 * @brief Kontroler zarzadzajacy stanem UI dla okienek dialogowych zwiazanych ze zdarzeniami/eventami.
 * W architekturze 2026 jest 100% odizolowany od bezposredniego DirectX/Windows API.
 * Komunikuje sie z reszta systemu uzywajac EventBus.
 */
class EventDialogBoxController {
public:
    EventDialogBoxController();
    ~EventDialogBoxController() = default;

    // Zakaz kopiowania i przenoszenia (controller)
    EventDialogBoxController(const EventDialogBoxController&) = delete;
    EventDialogBoxController& operator=(const EventDialogBoxController&) = delete;
    EventDialogBoxController(EventDialogBoxController&&) = delete;
    EventDialogBoxController& operator=(EventDialogBoxController&&) = delete;

    /**
     * @brief Proba otwarcia okna dialogowego dla danego eventu.
     * Zwraca blad, jesli inne okno jest juz otwarte lub ID jest nieprawidlowe.
     */
    Client::Core::ExpectedResult<void, EventDialogError> OpenDialog(DialogEventId eventId, std::string_view eventName);

    /**
     * @brief Zamyka aktualnie otwarte okno dialogowe eventu.
     * Zwraca blad jesli nie ma otwartego okna.
     */
    Client::Core::ExpectedResult<void, EventDialogError> CloseDialog();

    /**
     * @brief Aktualizuje stan w czasie rzeczywistym.
     */
    Client::Core::ExpectedResult<void, EventDialogError> Update();

    [[nodiscard]] bool IsDialogOpen() const noexcept;
    [[nodiscard]] std::optional<DialogEventId> GetCurrentDialogId() const noexcept;

private:
    std::optional<DialogEventId> currentDialogId_;
};

} // namespace Client::Events
