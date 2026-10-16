#include "UIDragDropController.h"

namespace Client::UI {

Client::Core::Result<void, DragDropError> UIDragDropController::StartDrag(Window* source) {
    if (!source) {
        return std::unexpected(DragDropError::NotDraggable);
    }
    
    if (m_dragSource) {
        return std::unexpected(DragDropError::AlreadyDragging);
    }

    if (!source->IsDraggable()) {
        return std::unexpected(DragDropError::NotDraggable);
    }

    if (!source->GetDragPayload().has_value()) {
        return std::unexpected(DragDropError::NoPayload);
    }

    m_dragSource = source;
    return {};
}

Client::Core::Result<void, DragDropError> UIDragDropController::Drop(Window* target) {
    if (!m_dragSource) {
        return std::unexpected(DragDropError::NotDragging);
    }

    if (!target) {
        m_dragSource = nullptr;
        return std::unexpected(DragDropError::DropTargetNotFound);
    }

    const auto& payloadOpt = m_dragSource->GetDragPayload();
    if (!payloadOpt.has_value()) {
        m_dragSource = nullptr;
        return std::unexpected(DragDropError::NoPayload);
    }

    bool accepted = target->OnDragDrop(m_dragSource, payloadOpt.value());
    m_dragSource = nullptr;

    if (!accepted) {
        return std::unexpected(DragDropError::DropRejected);
    }

    return {};
}

Client::Core::Result<void, DragDropError> UIDragDropController::CancelDrag() {
    if (!m_dragSource) {
        return std::unexpected(DragDropError::NotDragging);
    }

    m_dragSource = nullptr;
    return {};
}

bool UIDragDropController::IsDragging() const {
    return m_dragSource != nullptr;
}

Window* UIDragDropController::GetDragSource() const {
    return m_dragSource;
}

} // namespace Client::UI
