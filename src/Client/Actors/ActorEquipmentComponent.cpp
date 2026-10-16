#include "ActorEquipmentComponent.h"

namespace Client::Actors {

std::expected<void, EquipmentError> ActorEquipmentComponent::EquipItem(Core::SlotIndex slot, Core::ItemVnum vnum) {
    if (vnum.value() == 0) {
        return std::unexpected(EquipmentError::InvalidItem);
    }
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

std::expected<void, EquipmentError> ActorEquipmentComponent::SetModelPart(ModelPart part, Core::ItemVnum vnum) {
    if (vnum.value() == 0) {
        return std::unexpected(EquipmentError::InvalidItem);
    }
    if (m_modelParts.contains(part)) {
        return std::unexpected(EquipmentError::SlotAlreadyOccupied);
    }
    m_modelParts[part] = vnum;
    return {};
}

std::expected<void, EquipmentError> ActorEquipmentComponent::RemoveModelPart(ModelPart part) {
    if (!m_modelParts.contains(part)) {
        return std::unexpected(EquipmentError::SlotEmpty);
    }
    m_modelParts.erase(part);
    return {};
}

std::optional<Core::ItemVnum> ActorEquipmentComponent::GetModelPart(ModelPart part) const {
    auto it = m_modelParts.find(part);
    if (it != m_modelParts.end()) {
        return it->second;
    }
    return std::nullopt;
}

void ActorEquipmentComponent::ClearAll() {
    m_equipment.clear();
    m_modelParts.clear();
}

} // namespace Client::Actors
