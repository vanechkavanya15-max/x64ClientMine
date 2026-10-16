#include "EventDialogBoxController.h"

namespace Client::Events {

EventDialogBoxController::EventDialogBoxController()
    : currentDialogId_(std::nullopt) {
}

Client::Core::ExpectedResult<void, EventDialogError> EventDialogBoxController::OpenDialog(DialogEventId eventId, std::string_view eventName) {
    if (currentDialogId_.has_value()) {
        return Client::Core::MakeUnexpected(EventDialogError::AlreadyOpen);
    }

    if (eventId.get() == 0) {
        return Client::Core::MakeUnexpected(EventDialogError::InvalidDialogId);
    }
    
    if (eventName.empty()) {
        return Client::Core::MakeUnexpected(EventDialogError::ValidationFailed);
    }

    currentDialogId_ = eventId;

    // Publikacja zdarzenia na EventBus
    EterBase::EventBus::Instance().Publish(EventDialogBoxOpenedEvent{eventId, eventName});

    return {};
}

Client::Core::ExpectedResult<void, EventDialogError> EventDialogBoxController::CloseDialog() {
    if (!currentDialogId_.has_value()) {
        return Client::Core::MakeUnexpected(EventDialogError::NotOpen);
    }

    DialogEventId closedId = currentDialogId_.value();
    currentDialogId_.reset();

    // Publikacja zdarzenia na EventBus
    EterBase::EventBus::Instance().Publish(EventDialogBoxClosedEvent{closedId});

    return {};
}

Client::Core::ExpectedResult<void, EventDialogError> EventDialogBoxController::Update() {
    // Miejsce na ewentualna logike aktualizacji okna co klatke, np. odliczanie czasu
    // lub sprawdzanie warunkow zewnetrznych.
    if (!currentDialogId_.has_value()) {
        return Client::Core::MakeUnexpected(EventDialogError::NotOpen);
    }
    return {};
}

bool EventDialogBoxController::IsDialogOpen() const noexcept {
    return currentDialogId_.has_value();
}

std::optional<DialogEventId> EventDialogBoxController::GetCurrentDialogId() const noexcept {
    return currentDialogId_;
}

} // namespace Client::Events
