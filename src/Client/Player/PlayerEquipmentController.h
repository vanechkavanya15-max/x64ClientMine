#pragma once

#include <optional>
#include <unordered_map>
#include "Client/Core/StrongTypes.h"
#include "Client/Core/ExpectedResult.h"

namespace Client::Player {

enum class EquipmentError {
    InvalidSlot,
    SlotOccupied,
    SlotEmpty,
    InvalidItem,
    InternalError
};

class PlayerEquipmentController {
public:
    PlayerEquipmentController() = default;
    ~PlayerEquipmentController() = default;

    PlayerEquipmentController(const PlayerEquipmentController&) = default;
    PlayerEquipmentController& operator=(const PlayerEquipmentController&) = default;
    PlayerEquipmentController(PlayerEquipmentController&&) noexcept = default;
    PlayerEquipmentController& operator=(PlayerEquipmentController&&) noexcept = default;

    Core::ExpectedResult<void, EquipmentError> EquipItem(Core::SlotIndex slot, Core::ItemVnum item) noexcept;
    Core::ExpectedResult<Core::ItemVnum, EquipmentError> UnequipItem(Core::SlotIndex slot) noexcept;
    
    [[nodiscard]] std::optional<Core::ItemVnum> GetItem(Core::SlotIndex slot) const noexcept;
    [[nodiscard]] bool HasItem(Core::SlotIndex slot) const noexcept;
    
    void Clear() noexcept;

private:
    std::unordered_map<Core::SlotIndex, Core::ItemVnum> equipment_;
};

} // namespace Client::Player
