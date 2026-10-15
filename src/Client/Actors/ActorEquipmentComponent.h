#pragma once

#include <expected>
#include <optional>
#include <unordered_map>
#include "Client/Core/StrongTypes.h"

namespace Client::Actors {

enum class EquipmentError {
    InvalidSlot,
    SlotAlreadyOccupied,
    SlotEmpty
};

class ActorEquipmentComponent {
public:
    ActorEquipmentComponent() = default;
    ~ActorEquipmentComponent() = default;

    ActorEquipmentComponent(const ActorEquipmentComponent&) = delete;
    ActorEquipmentComponent& operator=(const ActorEquipmentComponent&) = delete;

    ActorEquipmentComponent(ActorEquipmentComponent&&) noexcept = default;
    ActorEquipmentComponent& operator=(ActorEquipmentComponent&&) noexcept = default;

    std::expected<void, EquipmentError> EquipItem(Core::SlotIndex slot, Core::ItemVnum vnum);
    std::expected<void, EquipmentError> UnequipItem(Core::SlotIndex slot);

    std::optional<Core::ItemVnum> GetItem(Core::SlotIndex slot) const;
    bool HasItem(Core::SlotIndex slot) const;

private:
    std::unordered_map<Core::SlotIndex, Core::ItemVnum> m_equipment;
};

} // namespace Client::Actors
