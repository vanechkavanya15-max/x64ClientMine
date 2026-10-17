#pragma once

#include <expected>
#include <optional>
#include <unordered_map>
#include "Client/Core/StrongTypes.h"

namespace Client::Actor {

enum class EquipmentError {
    InvalidSlot,
    SlotOccupied,
    SlotEmpty,
    InvalidItem
};

class ActorEquipmentComponent {
public:
    ActorEquipmentComponent() = default;
    ~ActorEquipmentComponent() = default;

    ActorEquipmentComponent(const ActorEquipmentComponent&) = delete;
    ActorEquipmentComponent& operator=(const ActorEquipmentComponent&) = delete;
    ActorEquipmentComponent(ActorEquipmentComponent&&) noexcept = default;
    ActorEquipmentComponent& operator=(ActorEquipmentComponent&&) noexcept = default;

    std::expected<void, EquipmentError> Equip(Client::Core::SlotIndex slot, Client::Core::ItemVnum item);
    std::expected<Client::Core::ItemVnum, EquipmentError> Unequip(Client::Core::SlotIndex slot);
    std::optional<Client::Core::ItemVnum> GetItem(Client::Core::SlotIndex slot) const;
    void Clear();

private:
    std::unordered_map<Client::Core::SlotIndex, Client::Core::ItemVnum> m_equipment;
};

} // namespace Client::Actor
