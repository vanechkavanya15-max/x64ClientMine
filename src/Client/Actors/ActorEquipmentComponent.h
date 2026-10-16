#pragma once

#include <expected>
#include <optional>
#include <unordered_map>
#include "Client/Core/StrongTypes.h"

namespace Client::Actors {

enum class EquipmentError {
    InvalidSlot,
    SlotAlreadyOccupied,
    SlotEmpty,
    InvalidItem
};

enum class ModelPart : uint8_t {
    Main = 0,
    Weapon,
    WeaponLeft,
    Hair,
    Sash,
    CostumeBody,
    CostumeHair,
    CostumeWeapon,
    MaxParts
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

    std::expected<void, EquipmentError> SetModelPart(ModelPart part, Core::ItemVnum vnum);
    std::expected<void, EquipmentError> RemoveModelPart(ModelPart part);
    std::optional<Core::ItemVnum> GetModelPart(ModelPart part) const;

    void ClearAll();

private:
    std::unordered_map<Core::SlotIndex, Core::ItemVnum> m_equipment;
    std::unordered_map<ModelPart, Core::ItemVnum> m_modelParts;
};

} // namespace Client::Actors
