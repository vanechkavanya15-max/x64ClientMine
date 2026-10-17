#pragma once

#include <vector>
#include <optional>
#include <cstdint>
#include <string_view>
#include <format>
#include "Client/Core/StrongTypes.h"
#include "Client/Core/Result.h"

namespace Client::UI {

enum class SlotGridError : uint8_t {
    OutOfBounds,
    SlotOccupied,
    ItemNotFound,
    NotEnoughSpace
};

[[nodiscard]] constexpr std::string_view to_string(SlotGridError error) noexcept {
    switch (error) {
        case SlotGridError::OutOfBounds: return "OutOfBounds";
        case SlotGridError::SlotOccupied: return "SlotOccupied";
        case SlotGridError::ItemNotFound: return "ItemNotFound";
        case SlotGridError::NotEnoughSpace: return "NotEnoughSpace";
        default: return "Unknown";
    }
}

} // namespace Client::UI

template <>
struct std::formatter<Client::UI::SlotGridError> : std::formatter<std::string_view> {
    auto format(Client::UI::SlotGridError err, std::format_context& ctx) const {
        return std::formatter<std::string_view>::format(Client::UI::to_string(err), ctx);
    }
};

namespace Client::UI {

class UISlotGridController {
public:
    UISlotGridController(uint8_t width, uint8_t height);
    
    // Zapobiegamy kopiowaniu
    UISlotGridController(const UISlotGridController&) = delete;
    UISlotGridController& operator=(const UISlotGridController&) = delete;

    UISlotGridController(UISlotGridController&&) noexcept = default;
    UISlotGridController& operator=(UISlotGridController&&) noexcept = default;

    [[nodiscard]] Core::Result<Core::SlotIndex, SlotGridError> PutItem(Core::SlotIndex slot, Core::ItemVnum vnum, uint8_t itemWidth, uint8_t itemHeight);
    [[nodiscard]] Core::Result<void, SlotGridError> RemoveItem(Core::SlotIndex slot);
    [[nodiscard]] Core::Result<Core::SlotIndex, SlotGridError> FindFreeSlot(uint8_t itemWidth, uint8_t itemHeight) const;
    [[nodiscard]] Core::Result<void, SlotGridError> MoveItem(Core::SlotIndex from, Core::SlotIndex to);

    [[nodiscard]] uint8_t GetWidth() const noexcept { return width_; }
    [[nodiscard]] uint8_t GetHeight() const noexcept { return height_; }
    
    [[nodiscard]] Core::Result<Core::ItemVnum, SlotGridError> GetItemVnum(Core::SlotIndex slot) const;

private:
    struct GridItem {
        Core::ItemVnum vnum;
        Core::SlotIndex startSlot;
        uint8_t width;
        uint8_t height;
    };

    uint8_t width_;
    uint8_t height_;
    
    // Przechowujemy referencję (indeks startowy) do przedmiotu w każdej zajętej komórce.
    // Pusta komórka to std::nullopt.
    std::vector<std::optional<Core::SlotIndex>> grid_;
    
    // Przechowujemy informacje o przedmiotach wg indeksu startowego
    std::vector<std::optional<GridItem>> items_;

    [[nodiscard]] bool IsValidCoordinates(uint8_t x, uint8_t y) const noexcept;
    [[nodiscard]] bool CanPlaceItem(uint8_t startX, uint8_t startY, uint8_t itemWidth, uint8_t itemHeight, std::optional<Core::SlotIndex> ignoreSlot = std::nullopt) const noexcept;
    [[nodiscard]] uint16_t CoordsToIndex(uint8_t x, uint8_t y) const noexcept;
    void IndexToCoords(uint16_t index, uint8_t& x, uint8_t& y) const noexcept;
};

} // namespace Client::UI
