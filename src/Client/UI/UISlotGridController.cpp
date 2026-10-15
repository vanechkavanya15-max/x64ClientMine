#include "UISlotGridController.h"

namespace Client::UI {

UISlotGridController::UISlotGridController(uint8_t width, uint8_t height)
    : width_(width), height_(height) {
    grid_.resize(static_cast<size_t>(width_) * height_, std::nullopt);
    items_.resize(static_cast<size_t>(width_) * height_, std::nullopt);
}

bool UISlotGridController::IsValidCoordinates(uint8_t x, uint8_t y) const noexcept {
    return x < width_ && y < height_;
}

uint16_t UISlotGridController::CoordsToIndex(uint8_t x, uint8_t y) const noexcept {
    return static_cast<uint16_t>(y) * width_ + x;
}

void UISlotGridController::IndexToCoords(uint16_t index, uint8_t& x, uint8_t& y) const noexcept {
    x = static_cast<uint8_t>(index % width_);
    y = static_cast<uint8_t>(index / width_);
}

bool UISlotGridController::CanPlaceItem(uint8_t startX, uint8_t startY, uint8_t itemWidth, uint8_t itemHeight, std::optional<Core::SlotIndex> ignoreSlot) const noexcept {
    if (startX + itemWidth > width_ || startY + itemHeight > height_) {
        return false;
    }

    for (uint8_t y = startY; y < startY + itemHeight; ++y) {
        for (uint8_t x = startX; x < startX + itemWidth; ++x) {
            uint16_t idx = CoordsToIndex(x, y);
            if (grid_[idx].has_value() && (!ignoreSlot.has_value() || grid_[idx].value() != ignoreSlot.value())) {
                return false;
            }
        }
    }

    return true;
}

Core::Result<Core::SlotIndex, SlotGridError> UISlotGridController::PutItem(Core::SlotIndex slot, Core::ItemVnum vnum, uint8_t itemWidth, uint8_t itemHeight) {
    if (slot.get() >= grid_.size()) {
        return std::unexpected(SlotGridError::OutOfBounds);
    }

    uint8_t x, y;
    IndexToCoords(slot.get(), x, y);

    if (!IsValidCoordinates(x, y)) {
        return std::unexpected(SlotGridError::OutOfBounds);
    }

    if (!CanPlaceItem(x, y, itemWidth, itemHeight)) {
        return std::unexpected(SlotGridError::SlotOccupied);
    }

    GridItem item{vnum, slot, itemWidth, itemHeight};
    items_[slot.get()] = item;

    for (uint8_t currY = y; currY < y + itemHeight; ++currY) {
        for (uint8_t currX = x; currX < x + itemWidth; ++currX) {
            grid_[CoordsToIndex(currX, currY)] = slot;
        }
    }

    return slot;
}

Core::Result<void, SlotGridError> UISlotGridController::RemoveItem(Core::SlotIndex slot) {
    if (slot.get() >= grid_.size()) {
        return std::unexpected(SlotGridError::OutOfBounds);
    }
    
    if (!grid_[slot.get()].has_value()) {
        return std::unexpected(SlotGridError::ItemNotFound);
    }
    
    // Get actual top-left slot of the item
    Core::SlotIndex actualSlot = grid_[slot.get()].value();
    
    if (!items_[actualSlot.get()].has_value()) {
        return std::unexpected(SlotGridError::ItemNotFound);
    }

    const auto& item = items_[actualSlot.get()].value();
    uint8_t x, y;
    IndexToCoords(actualSlot.get(), x, y);

    for (uint8_t currY = y; currY < y + item.height; ++currY) {
        for (uint8_t currX = x; currX < x + item.width; ++currX) {
            grid_[CoordsToIndex(currX, currY)] = std::nullopt;
        }
    }

    items_[actualSlot.get()] = std::nullopt;
    return {};
}

Core::Result<Core::SlotIndex, SlotGridError> UISlotGridController::FindFreeSlot(uint8_t itemWidth, uint8_t itemHeight) const {
    for (uint8_t y = 0; y < height_; ++y) {
        for (uint8_t x = 0; x < width_; ++x) {
            if (CanPlaceItem(x, y, itemWidth, itemHeight)) {
                return Core::SlotIndex(CoordsToIndex(x, y));
            }
        }
    }
    return std::unexpected(SlotGridError::NotEnoughSpace);
}

Core::Result<void, SlotGridError> UISlotGridController::MoveItem(Core::SlotIndex from, Core::SlotIndex to) {
    if (from.get() >= grid_.size() || to.get() >= grid_.size()) {
        return std::unexpected(SlotGridError::OutOfBounds);
    }

    if (!grid_[from.get()].has_value()) {
        return std::unexpected(SlotGridError::ItemNotFound);
    }

    Core::SlotIndex actualFrom = grid_[from.get()].value();

    if (!items_[actualFrom.get()].has_value()) {
        return std::unexpected(SlotGridError::ItemNotFound);
    }

    const auto& item = items_[actualFrom.get()].value();
    uint8_t toX, toY;
    IndexToCoords(to.get(), toX, toY);
    
    if (toX + item.width > width_ || toY + item.height > height_) {
         return std::unexpected(SlotGridError::OutOfBounds);
    }

    if (!CanPlaceItem(toX, toY, item.width, item.height, actualFrom)) {
        return std::unexpected(SlotGridError::SlotOccupied);
    }
    
    // Copy item data
    Core::ItemVnum vnum = item.vnum;
    uint8_t w = item.width;
    uint8_t h = item.height;
    
    // Remove from old pos
    auto removeRes = RemoveItem(actualFrom);
    if (!removeRes) {
        return removeRes;
    }
    
    // Place at new pos
    auto putRes = PutItem(to, vnum, w, h);
    if (!putRes) {
        // Rollback? Should not fail due to checks, but just in case
        return std::unexpected(putRes.error());
    }

    return {};
}

Core::Result<Core::ItemVnum, SlotGridError> UISlotGridController::GetItemVnum(Core::SlotIndex slot) const {
    if (slot.get() >= grid_.size()) {
        return std::unexpected(SlotGridError::OutOfBounds);
    }
    
    if (!grid_[slot.get()].has_value()) {
        return std::unexpected(SlotGridError::ItemNotFound);
    }
    
    Core::SlotIndex actualSlot = grid_[slot.get()].value();
    if (!items_[actualSlot.get()].has_value()) {
        return std::unexpected(SlotGridError::ItemNotFound);
    }
    
    return items_[actualSlot.get()].value().vnum;
}

} // namespace Client::UI
