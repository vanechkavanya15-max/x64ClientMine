#pragma once

#include "Client/UI/UISlotGridController.h"
#include "Client/Core/Result.h"
#include "Client/Core/StrongTypes.h"
#include <optional>
#include <string_view>
#include <format>
#include <cstdint>

namespace Client::UI {

enum class SlotDragDropError : uint8_t {
    NotDragging,
    AlreadyDragging,
    SourceItemNotFound,
    TargetOccupied,
    InvalidTarget,
    GridError
};

[[nodiscard]] constexpr std::string_view to_string(SlotDragDropError error) noexcept {
    switch (error) {
        case SlotDragDropError::NotDragging: return "NotDragging";
        case SlotDragDropError::AlreadyDragging: return "AlreadyDragging";
        case SlotDragDropError::SourceItemNotFound: return "SourceItemNotFound";
        case SlotDragDropError::TargetOccupied: return "TargetOccupied";
        case SlotDragDropError::InvalidTarget: return "InvalidTarget";
        case SlotDragDropError::GridError: return "GridError";
        default: return "Unknown";
    }
}

class SlotItemDragDropManager {
public:
    SlotItemDragDropManager() = default;
    
    // Prevent copying
    SlotItemDragDropManager(const SlotItemDragDropManager&) = delete;
    SlotItemDragDropManager& operator=(const SlotItemDragDropManager&) = delete;

    [[nodiscard]] Core::Result<void, SlotDragDropError> StartDrag(UISlotGridController* sourceGrid, Core::SlotIndex sourceSlot, uint8_t itemWidth, uint8_t itemHeight);
    [[nodiscard]] Core::Result<void, SlotDragDropError> Drop(UISlotGridController* targetGrid, Core::SlotIndex targetSlot);
    [[nodiscard]] Core::Result<void, SlotDragDropError> CancelDrag();

    [[nodiscard]] bool IsDragging() const noexcept { return drag_state_.has_value(); }

private:
    struct DragState {
        UISlotGridController* sourceGrid;
        Core::SlotIndex sourceSlot;
        Core::ItemVnum itemVnum;
        uint8_t itemWidth;
        uint8_t itemHeight;
    };

    std::optional<DragState> drag_state_;
};

} // namespace Client::UI

template <>
struct std::formatter<Client::UI::SlotDragDropError> : std::formatter<std::string_view> {
    auto format(Client::UI::SlotDragDropError err, std::format_context& ctx) const {
        return std::formatter<std::string_view>::format(Client::UI::to_string(err), ctx);
    }
};
