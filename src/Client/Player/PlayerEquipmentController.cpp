#include "PlayerEquipmentController.h"

namespace Client::Player {

std::expected<void, EquipmentError> PlayerEquipmentController::EquipItem(EquipmentSlotType slot, Client::Core::ItemVnum vnum) {
    if (!IsSlotEmpty(slot)) {
        return std::unexpected(EquipmentError::SlotOccupied);
    }
    equippedItems_[slot] = vnum;
    return {};
}

std::expected<void, EquipmentError> PlayerEquipmentController::UnequipItem(EquipmentSlotType slot) {
    if (IsSlotEmpty(slot)) {
        return std::unexpected(EquipmentError::SlotEmpty);
    }
    equippedItems_.erase(slot);
    return {};
}

std::expected<Client::Core::ItemVnum, EquipmentError> PlayerEquipmentController::GetItemInSlot(EquipmentSlotType slot) const {
    auto it = equippedItems_.find(slot);
    if (it == equippedItems_.end()) {
        return std::unexpected(EquipmentError::SlotEmpty);
    }
    return it->second;
}

bool PlayerEquipmentController::IsSlotEmpty(EquipmentSlotType slot) const {
    return equippedItems_.find(slot) == equippedItems_.end();
}

void PlayerEquipmentController::ClearAll() {
    equippedItems_.clear();
}

} // namespace Client::Player
