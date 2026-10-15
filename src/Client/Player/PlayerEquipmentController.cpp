#include "Client/Player/PlayerEquipmentController.h"

namespace Client::Player {

Core::ExpectedResult<void, EquipmentError> PlayerEquipmentController::EquipItem(Core::SlotIndex slot, Core::ItemVnum item) noexcept {
    if (!slot) {
        return Core::MakeUnexpected(EquipmentError::InvalidSlot);
    }
    if (!item) {
        return Core::MakeUnexpected(EquipmentError::InvalidItem);
    }
    
    if (equipment_.contains(slot)) {
        return Core::MakeUnexpected(EquipmentError::SlotOccupied);
    }

    try {
        equipment_.emplace(slot, item);
        return {};
    } catch (...) {
        return Core::MakeUnexpected(EquipmentError::InternalError);
    }
}

Core::ExpectedResult<Core::ItemVnum, EquipmentError> PlayerEquipmentController::UnequipItem(Core::SlotIndex slot) noexcept {
    if (!slot) {
        return Core::MakeUnexpected(EquipmentError::InvalidSlot);
    }

    auto it = equipment_.find(slot);
    if (it == equipment_.end()) {
        return Core::MakeUnexpected(EquipmentError::SlotEmpty);
    }

    Core::ItemVnum item = it->second;
    equipment_.erase(it);
    return item;
}

std::optional<Core::ItemVnum> PlayerEquipmentController::GetItem(Core::SlotIndex slot) const noexcept {
    if (!slot) {
        return std::nullopt;
    }

    auto it = equipment_.find(slot);
    if (it != equipment_.end()) {
        return it->second;
    }

    return std::nullopt;
}

bool PlayerEquipmentController::HasItem(Core::SlotIndex slot) const noexcept {
    if (!slot) {
        return false;
    }
    return equipment_.contains(slot);
}

void PlayerEquipmentController::Clear() noexcept {
    equipment_.clear();
}

} // namespace Client::Player
