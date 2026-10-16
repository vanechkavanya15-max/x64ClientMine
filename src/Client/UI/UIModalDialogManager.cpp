#include "UIModalDialogManager.h"
#include <algorithm>

namespace Client::UI {

Client::Core::Result<void, UIError> UIModalDialogManager::PushDialog(DialogId id) {
    if (!id) {
        return std::unexpected(UIError::InvalidDialogId);
    }

    if (IsDialogActive(id)) {
        return std::unexpected(UIError::DialogAlreadyActive);
    }

    m_activeDialogs.push_back(id);
    EterBase::EventBus::Instance().Publish(ModalDialogPushedEvent{id});

    return {};
}

Client::Core::Result<void, UIError> UIModalDialogManager::PopDialog(std::optional<DialogId> expectedId) {
    if (m_activeDialogs.empty()) {
        return std::unexpected(UIError::NoActiveDialog);
    }

    DialogId topDialog = m_activeDialogs.back();

    if (expectedId.has_value() && expectedId.value() != topDialog) {
        return std::unexpected(UIError::DialogNotFound);
    }

    m_activeDialogs.pop_back();
    EterBase::EventBus::Instance().Publish(ModalDialogPoppedEvent{topDialog});

    return {};
}

Client::Core::Result<DialogId, UIError> UIModalDialogManager::GetActiveDialog() const noexcept {
    if (m_activeDialogs.empty()) {
        return std::unexpected(UIError::NoActiveDialog);
    }
    
    return m_activeDialogs.back();
}

bool UIModalDialogManager::IsDialogActive(DialogId id) const noexcept {
    return std::ranges::find(m_activeDialogs, id) != m_activeDialogs.end();
}

void UIModalDialogManager::ClearAll() noexcept {
    while (!m_activeDialogs.empty()) {
        DialogId topDialog = m_activeDialogs.back();
        m_activeDialogs.pop_back();
        EterBase::EventBus::Instance().Publish(ModalDialogPoppedEvent{topDialog});
    }
}

} // namespace Client::UI
