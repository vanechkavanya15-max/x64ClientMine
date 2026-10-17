#include "ActorEquipmentComponent.h"

namespace Client::Actor {

std::expected<void, EquipmentError> ActorEquipmentComponent::Equip(Client::Core::SlotIndex slot, Client::Core::ItemVnum item) {
    if (item.value() == 0) {
        return std::unexpected(EquipmentError::InvalidItem);
    }
    
    // Używamy iteratora by uniknąć podwójnego szukania
    if (auto it = m_equipment.find(slot); it != m_equipment.end()) {
        return std::unexpected(EquipmentError::SlotOccupied);
    }

    m_equipment.emplace(slot, item);
    return {};
}

std::expected<Client::Core::ItemVnum, EquipmentError> ActorEquipmentComponent::Unequip(Client::Core::SlotIndex slot) {
    auto it = m_equipment.find(slot);
    if (it == m_equipment.end()) {
        return std::unexpected(EquipmentError::SlotEmpty);
    }

    Client::Core::ItemVnum unequippedItem = it->second;
    m_equipment.erase(it);
    
    return unequippedItem;
}

std::optional<Client::Core::ItemVnum> ActorEquipmentComponent::GetItem(Client::Core::SlotIndex slot) const {
    auto it = m_equipment.find(slot);
    if (it != m_equipment.end()) {
        return it->second;
    }
    
    return std::nullopt;
}

void ActorEquipmentComponent::Clear() {
    m_equipment.clear();
}

} // namespace Client::Actor
