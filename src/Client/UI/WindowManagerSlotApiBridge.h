#pragma once

#include <cstdint>
#include <string>
#include <string_view>
#include <unordered_map>
#include <optional>
#include <vector>
#include <format>
#include "Client/Core/StrongTypes.h"
#include "Client/Core/Result.h"

namespace Client::UI {

enum class SlotApiError : uint8_t {
    SlotNotFound,
    SlotAlreadyExists,
    CoolTimeNotFound,
    InvalidCoolTime,
    SlotLocked,
    NotEnoughItems
};

[[nodiscard]] constexpr std::string_view to_string(SlotApiError error) noexcept {
    switch (error) {
        case SlotApiError::SlotNotFound: return "SlotNotFound";
        case SlotApiError::SlotAlreadyExists: return "SlotAlreadyExists";
        case SlotApiError::CoolTimeNotFound: return "CoolTimeNotFound";
        case SlotApiError::InvalidCoolTime: return "InvalidCoolTime";
        case SlotApiError::SlotLocked: return "SlotLocked";
        case SlotApiError::NotEnoughItems: return "NotEnoughItems";
        default: return "Unknown";
    }
}

} // namespace Client::UI

template <>
struct std::formatter<Client::UI::SlotApiError> : std::formatter<std::string_view> {
    auto format(Client::UI::SlotApiError err, std::format_context& ctx) const {
        return std::formatter<std::string_view>::format(Client::UI::to_string(err), ctx);
    }
};

namespace Client::UI {

struct SlotCoolTime {
    float duration{0.0f};
    float elapsed{0.0f};
    bool active{false};
    bool paused{false};
};

struct SlotCover {
    bool active{false};
    uint32_t coverType{0};
};

struct SlotData {
    Core::SlotIndex slotIndex{0};
    Core::ItemVnum itemVnum{0};
    uint32_t count{0};
    bool locked{false};
    bool enabled{true};
    uint32_t style{0};
    
    // Layered CoolTimes (e.g., layer 0 for global cd, layer 1 for item-specific cd)
    std::unordered_map<uint32_t, SlotCoolTime> coolTimes;
    
    // Cover image state
    SlotCover cover{};
};

struct AttachedIconData {
    uint32_t type{0};
    uint32_t index{0};
    uint32_t slotNumber{0};
    uint8_t width{0};
    uint8_t height{0};
    bool attaching{false};
    bool dragging{false};
};

class WindowManagerSlotApiBridge {
public:
    WindowManagerSlotApiBridge() = default;
    ~WindowManagerSlotApiBridge() = default;

    static WindowManagerSlotApiBridge& Instance() noexcept;

    Core::Result<void, SlotApiError> SetSlot(Core::SlotIndex slotIndex, Core::ItemVnum itemVnum, uint32_t count);
    Core::Result<void, SlotApiError> SetSlotCount(Core::SlotIndex slotIndex, uint32_t count);
    Core::Result<void, SlotApiError> ClearSlot(Core::SlotIndex slotIndex);
    void ClearAllSlots();
    
    [[nodiscard]] std::optional<SlotData> GetSlot(Core::SlotIndex slotIndex) const;
    [[nodiscard]] size_t GetSlotCount() const noexcept { return m_slots.size(); }
    
    Core::Result<void, SlotApiError> SetSlotCoolTime(Core::SlotIndex slotIndex, uint32_t layerId, float duration, float elapsed = 0.0f);
    Core::Result<void, SlotApiError> StoreSlotCoolTime(Core::SlotIndex slotIndex, uint32_t layerId, const std::string& key);
    Core::Result<void, SlotApiError> RestoreSlotCoolTime(Core::SlotIndex slotIndex, uint32_t layerId, const std::string& key);
    Core::Result<void, SlotApiError> TransferSlotCoolTime(Core::SlotIndex fromSlot, Core::SlotIndex toSlot, uint32_t layerId);
    
    Core::Result<void, SlotApiError> PauseSlotCoolTime(Core::SlotIndex slotIndex, uint32_t layerId);
    Core::Result<void, SlotApiError> ResumeSlotCoolTime(Core::SlotIndex slotIndex, uint32_t layerId);
    
    Core::Result<void, SlotApiError> LockSlot(Core::SlotIndex slotIndex);
    Core::Result<void, SlotApiError> UnlockSlot(Core::SlotIndex slotIndex);
    [[nodiscard]] bool IsSlotLocked(Core::SlotIndex slotIndex) const;
    
    Core::Result<void, SlotApiError> SetSlotStyle(Core::SlotIndex slotIndex, uint32_t style);
    Core::Result<void, SlotApiError> EnableSlot(Core::SlotIndex slotIndex, bool enable);
    [[nodiscard]] bool IsSlotEnabled(Core::SlotIndex slotIndex) const;

    Core::Result<void, SlotApiError> SetSlotCover(Core::SlotIndex slotIndex, uint32_t coverType);
    Core::Result<void, SlotApiError> ClearSlotCover(Core::SlotIndex slotIndex);

    Core::Result<void, SlotApiError> SwapSlots(Core::SlotIndex slotA, Core::SlotIndex slotB);

    // Drag and drop / Attaching icon
    void AttachIcon(uint32_t type, uint32_t index, uint32_t slotNumber, uint8_t width, uint8_t height);
    void DeattachIcon();
    void SetAttachingFlag(bool flag);
    [[nodiscard]] bool IsAttaching() const noexcept;
    [[nodiscard]] bool IsDragging() const noexcept;
    void SetDragging(bool dragging) noexcept;
    [[nodiscard]] const AttachedIconData& GetAttachedIcon() const noexcept;

    void Update(float deltaTime);

private:
    std::unordered_map<Core::SlotIndex, SlotData> m_slots;
    std::unordered_map<std::string, SlotCoolTime> m_storedCoolTimes;
    AttachedIconData m_attachedIcon{};
};

} // namespace Client::UI
