#include "ActorEquipmentComponent.h"

namespace Client::Actors {

std::expected<void, EquipmentError> ActorEquipmentComponent::EquipItem(Core::SlotIndex slot, Core::ItemVnum vnum) {
    if (m_equipment.contains(slot)) {
        return std::unexpected(EquipmentError::SlotAlreadyOccupied);
    }
    m_equipment[slot] = vnum;
    return {};
}

std::expected<void, EquipmentError> ActorEquipmentComponent::UnequipItem(Core::SlotIndex slot) {
    if (!m_equipment.contains(slot)) {
        return std::unexpected(EquipmentError::SlotEmpty);
    }
    m_equipment.erase(slot);
    return {};
}

std::optional<Core::ItemVnum> ActorEquipmentComponent::GetItem(Core::SlotIndex slot) const {
    auto it = m_equipment.find(slot);
    if (it != m_equipment.end()) {
        return it->second;
    }
    return std::nullopt;
}

bool ActorEquipmentComponent::HasItem(Core::SlotIndex slot) const {
    return m_equipment.contains(slot);
}

} // namespace Client::Actors
