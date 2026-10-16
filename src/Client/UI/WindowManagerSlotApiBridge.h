#pragma once
#include <cstdint>
#include <string>
#include <unordered_map>
#include <optional>

namespace Client::UI {

struct SlotCoolTime {
    float duration{0.0f};
    float elapsed{0.0f};
    bool active{false};
};

struct SlotData {
    uint32_t slotIndex{0};
    uint32_t itemVnum{0};
    uint32_t count{0};
    bool locked{false};
    bool enabled{true};
    uint32_t style{0};
    SlotCoolTime coolTime{};
};

class WindowManagerSlotApiBridge {
public:
    WindowManagerSlotApiBridge() = default;
    ~WindowManagerSlotApiBridge() = default;

    bool SetSlot(uint32_t slotIndex, uint32_t itemVnum, uint32_t count);
    bool ClearSlot(uint32_t slotIndex);
    void ClearAllSlots();
    [[nodiscard]] std::optional<SlotData> GetSlot(uint32_t slotIndex) const;
    [[nodiscard]] size_t GetSlotCount() const noexcept { return m_slots.size(); }
    
    bool SetSlotCoolTime(uint32_t slotIndex, float duration, float elapsed = 0.0f);
    bool StoreSlotCoolTime(uint32_t slotIndex, const std::string& key);
    bool RestoreSlotCoolTime(uint32_t slotIndex, const std::string& key);
    bool TransferSlotCoolTime(uint32_t fromSlot, uint32_t toSlot);
    
    bool LockSlot(uint32_t slotIndex);
    bool UnlockSlot(uint32_t slotIndex);
    [[nodiscard]] bool IsSlotLocked(uint32_t slotIndex) const;
    
    bool SetSlotStyle(uint32_t slotIndex, uint32_t style);
    bool EnableSlot(uint32_t slotIndex, bool enable);
    [[nodiscard]] bool IsSlotEnabled(uint32_t slotIndex) const;
    void Update(float deltaTime);

private:
    std::unordered_map<uint32_t, SlotData> m_slots;
    std::unordered_map<std::string, SlotCoolTime> m_storedCoolTimes;
};

} // namespace Client::UI
