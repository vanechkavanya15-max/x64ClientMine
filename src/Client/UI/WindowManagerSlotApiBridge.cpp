#include "WindowManagerSlotApiBridge.h"

namespace Client::UI {

bool WindowManagerSlotApiBridge::SetSlot(uint32_t slotIndex, uint32_t itemVnum, uint32_t count) {
    auto& slot = m_slots[slotIndex];
    slot.slotIndex = slotIndex;
    slot.itemVnum = itemVnum;
    slot.count = count;
    slot.enabled = true;
    return true;
}

bool WindowManagerSlotApiBridge::ClearSlot(uint32_t slotIndex) {
    auto it = m_slots.find(slotIndex);
    if (it != m_slots.end()) {
        m_slots.erase(it);
        return true;
    }
    return false;
}

void WindowManagerSlotApiBridge::ClearAllSlots() {
    m_slots.clear();
}

std::optional<SlotData> WindowManagerSlotApiBridge::GetSlot(uint32_t slotIndex) const {
    auto it = m_slots.find(slotIndex);
    if (it != m_slots.end()) return it->second;
    return std::nullopt;
}

bool WindowManagerSlotApiBridge::SetSlotCoolTime(uint32_t slotIndex, float duration, float elapsed) {
    auto it = m_slots.find(slotIndex);
    if (it == m_slots.end()) return false;
    it->second.coolTime.duration = duration;
    it->second.coolTime.elapsed = elapsed;
    it->second.coolTime.active = (duration > 0.0f && elapsed < duration);
    return true;
}

bool WindowManagerSlotApiBridge::StoreSlotCoolTime(uint32_t slotIndex, const std::string& key) {
    auto it = m_slots.find(slotIndex);
    if (it == m_slots.end()) return false;
    m_storedCoolTimes[key] = it->second.coolTime;
    return true;
}

bool WindowManagerSlotApiBridge::RestoreSlotCoolTime(uint32_t slotIndex, const std::string& key) {
    auto storedIt = m_storedCoolTimes.find(key);
    if (storedIt == m_storedCoolTimes.end()) return false;
    auto it = m_slots.find(slotIndex);
    if (it == m_slots.end()) return false;
    it->second.coolTime = storedIt->second;
    return true;
}

bool WindowManagerSlotApiBridge::TransferSlotCoolTime(uint32_t fromSlot, uint32_t toSlot) {
    auto fromIt = m_slots.find(fromSlot);
    if (fromIt == m_slots.end()) return false;
    auto toIt = m_slots.find(toSlot);
    if (toIt == m_slots.end()) return false;
    toIt->second.coolTime = fromIt->second.coolTime;
    fromIt->second.coolTime = {};
    return true;
}

bool WindowManagerSlotApiBridge::LockSlot(uint32_t slotIndex) {
    auto it = m_slots.find(slotIndex);
    if (it == m_slots.end()) return false;
    it->second.locked = true;
    return true;
}

bool WindowManagerSlotApiBridge::UnlockSlot(uint32_t slotIndex) {
    auto it = m_slots.find(slotIndex);
    if (it == m_slots.end()) return false;
    it->second.locked = false;
    return true;
}

bool WindowManagerSlotApiBridge::IsSlotLocked(uint32_t slotIndex) const {
    auto it = m_slots.find(slotIndex);
    return it != m_slots.end() ? it->second.locked : false;
}

bool WindowManagerSlotApiBridge::SetSlotStyle(uint32_t slotIndex, uint32_t style) {
    auto it = m_slots.find(slotIndex);
    if (it == m_slots.end()) return false;
    it->second.style = style;
    return true;
}

bool WindowManagerSlotApiBridge::EnableSlot(uint32_t slotIndex, bool enable) {
    auto it = m_slots.find(slotIndex);
    if (it == m_slots.end()) return false;
    it->second.enabled = enable;
    return true;
}

bool WindowManagerSlotApiBridge::IsSlotEnabled(uint32_t slotIndex) const {
    auto it = m_slots.find(slotIndex);
    return it != m_slots.end() ? it->second.enabled : false;
}

void WindowManagerSlotApiBridge::Update(float deltaTime) {
    for (auto& [_, slot] : m_slots) {
        if (slot.coolTime.active) {
            slot.coolTime.elapsed += deltaTime;
            if (slot.coolTime.elapsed >= slot.coolTime.duration) {
                slot.coolTime.active = false;
                slot.coolTime.elapsed = slot.coolTime.duration;
            }
        }
    }
}

} // namespace Client::UI
