#include "SlotItemDragDropManager.h"

namespace Client::UI {

Core::Result<void, SlotDragDropError> SlotItemDragDropManager::StartDrag(UISlotGridController* sourceGrid, Core::SlotIndex sourceSlot, uint8_t itemWidth, uint8_t itemHeight) {
    if (drag_state_.has_value()) {
        return std::unexpected(SlotDragDropError::AlreadyDragging);
    }
    
    if (!sourceGrid) {
        return std::unexpected(SlotDragDropError::InvalidTarget);
    }

    auto vnumRes = sourceGrid->GetItemVnum(sourceSlot);
    if (!vnumRes) {
        return std::unexpected(SlotDragDropError::SourceItemNotFound);
    }

    Core::ItemVnum vnum = vnumRes.value();

    auto rmRes = sourceGrid->RemoveItem(sourceSlot);
    if (!rmRes) {
        return std::unexpected(SlotDragDropError::GridError);
    }

    drag_state_ = DragState{sourceGrid, sourceSlot, vnum, itemWidth, itemHeight};
    return {};
}

Core::Result<void, SlotDragDropError> SlotItemDragDropManager::Drop(UISlotGridController* targetGrid, Core::SlotIndex targetSlot) {
    if (!drag_state_.has_value()) {
        return std::unexpected(SlotDragDropError::NotDragging);
    }

    if (!targetGrid) {
        (void)CancelDrag();
        return std::unexpected(SlotDragDropError::InvalidTarget);
    }

    auto state = drag_state_.value();

    auto putRes = targetGrid->PutItem(targetSlot, state.itemVnum, state.itemWidth, state.itemHeight);
    if (!putRes) {
        if (putRes.error() == SlotGridError::OutOfBounds || putRes.error() == SlotGridError::SlotOccupied) {
            (void)CancelDrag();
            return std::unexpected(SlotDragDropError::TargetOccupied);
        }
        (void)CancelDrag();
        return std::unexpected(SlotDragDropError::GridError);
    }

    drag_state_.reset();
    return {};
}

Core::Result<void, SlotDragDropError> SlotItemDragDropManager::CancelDrag() {
    if (!drag_state_.has_value()) {
        return std::unexpected(SlotDragDropError::NotDragging);
    }

    auto state = drag_state_.value();
    
    // Attempt rollback
    auto putRes = state.sourceGrid->PutItem(state.sourceSlot, state.itemVnum, state.itemWidth, state.itemHeight);
    if (!putRes) {
        // We tried to rollback and failed. It implies grid structure error or bug
        drag_state_.reset();
        return std::unexpected(SlotDragDropError::GridError);
    }

    drag_state_.reset();
    return {};
}

} // namespace Client::UI
