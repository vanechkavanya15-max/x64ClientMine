#include "WindowManagerSlotApiBridge.h"
#include <algorithm>

namespace Client::UI {

Core::Result<void, SlotApiError> WindowManagerSlotApiBridge::SetSlot(Core::SlotIndex slotIndex, Core::ItemVnum itemVnum, uint32_t count) {
    auto& slot = m_slots[slotIndex];
    slot.slotIndex = slotIndex;
    slot.itemVnum = itemVnum;
    slot.count = count;
    slot.enabled = true;
    return {};
}

Core::Result<void, SlotApiError> WindowManagerSlotApiBridge::ClearSlot(Core::SlotIndex slotIndex) {
    auto it = m_slots.find(slotIndex);
    if (it != m_slots.end()) {
        m_slots.erase(it);
        return {};
    }
    return std::unexpected(SlotApiError::SlotNotFound);
}

void WindowManagerSlotApiBridge::ClearAllSlots() {
    m_slots.clear();
}

std::optional<SlotData> WindowManagerSlotApiBridge::GetSlot(Core::SlotIndex slotIndex) const {
    auto it = m_slots.find(slotIndex);
    if (it != m_slots.end()) {
        return it->second;
    }
    return std::nullopt;
}

Core::Result<void, SlotApiError> WindowManagerSlotApiBridge::SetSlotCoolTime(Core::SlotIndex slotIndex, uint32_t layerId, float duration, float elapsed) {
    auto it = m_slots.find(slotIndex);
    if (it == m_slots.end()) {
        return std::unexpected(SlotApiError::SlotNotFound);
    }
    if (duration < 0.0f || elapsed < 0.0f) {
        return std::unexpected(SlotApiError::InvalidCoolTime);
    }
    
    auto& coolTime = it->second.coolTimes[layerId];
    coolTime.duration = duration;
    coolTime.elapsed = elapsed;
    coolTime.active = (duration > 0.0f && elapsed < duration);
    coolTime.paused = false;
    
    return {};
}

Core::Result<void, SlotApiError> WindowManagerSlotApiBridge::StoreSlotCoolTime(Core::SlotIndex slotIndex, uint32_t layerId, const std::string& key) {
    auto it = m_slots.find(slotIndex);
    if (it == m_slots.end()) {
        return std::unexpected(SlotApiError::SlotNotFound);
    }
    auto coolIt = it->second.coolTimes.find(layerId);
    if (coolIt == it->second.coolTimes.end()) {
        return std::unexpected(SlotApiError::CoolTimeNotFound);
    }
    m_storedCoolTimes[key] = coolIt->second;
    return {};
}

Core::Result<void, SlotApiError> WindowManagerSlotApiBridge::RestoreSlotCoolTime(Core::SlotIndex slotIndex, uint32_t layerId, const std::string& key) {
    auto storedIt = m_storedCoolTimes.find(key);
    if (storedIt == m_storedCoolTimes.end()) {
        return std::unexpected(SlotApiError::CoolTimeNotFound);
    }
    auto it = m_slots.find(slotIndex);
    if (it == m_slots.end()) {
        return std::unexpected(SlotApiError::SlotNotFound);
    }
    it->second.coolTimes[layerId] = storedIt->second;
    return {};
}

Core::Result<void, SlotApiError> WindowManagerSlotApiBridge::TransferSlotCoolTime(Core::SlotIndex fromSlot, Core::SlotIndex toSlot, uint32_t layerId) {
    auto fromIt = m_slots.find(fromSlot);
    if (fromIt == m_slots.end()) {
        return std::unexpected(SlotApiError::SlotNotFound);
    }
    auto toIt = m_slots.find(toSlot);
    if (toIt == m_slots.end()) {
        return std::unexpected(SlotApiError::SlotNotFound);
    }
    
    auto coolIt = fromIt->second.coolTimes.find(layerId);
    if (coolIt != fromIt->second.coolTimes.end()) {
        toIt->second.coolTimes[layerId] = coolIt->second;
        fromIt->second.coolTimes.erase(coolIt);
        return {};
    }
    return std::unexpected(SlotApiError::CoolTimeNotFound);
}

Core::Result<void, SlotApiError> WindowManagerSlotApiBridge::PauseSlotCoolTime(Core::SlotIndex slotIndex, uint32_t layerId) {
    auto it = m_slots.find(slotIndex);
    if (it == m_slots.end()) {
        return std::unexpected(SlotApiError::SlotNotFound);
    }
    auto coolIt = it->second.coolTimes.find(layerId);
    if (coolIt == it->second.coolTimes.end()) {
        return std::unexpected(SlotApiError::CoolTimeNotFound);
    }
    coolIt->second.paused = true;
    return {};
}

Core::Result<void, SlotApiError> WindowManagerSlotApiBridge::ResumeSlotCoolTime(Core::SlotIndex slotIndex, uint32_t layerId) {
    auto it = m_slots.find(slotIndex);
    if (it == m_slots.end()) {
        return std::unexpected(SlotApiError::SlotNotFound);
    }
    auto coolIt = it->second.coolTimes.find(layerId);
    if (coolIt == it->second.coolTimes.end()) {
        return std::unexpected(SlotApiError::CoolTimeNotFound);
    }
    coolIt->second.paused = false;
    return {};
}

Core::Result<void, SlotApiError> WindowManagerSlotApiBridge::LockSlot(Core::SlotIndex slotIndex) {
    auto it = m_slots.find(slotIndex);
    if (it == m_slots.end()) {
        return std::unexpected(SlotApiError::SlotNotFound);
    }
    it->second.locked = true;
    return {};
}

Core::Result<void, SlotApiError> WindowManagerSlotApiBridge::UnlockSlot(Core::SlotIndex slotIndex) {
    auto it = m_slots.find(slotIndex);
    if (it == m_slots.end()) {
        return std::unexpected(SlotApiError::SlotNotFound);
    }
    it->second.locked = false;
    return {};
}

bool WindowManagerSlotApiBridge::IsSlotLocked(Core::SlotIndex slotIndex) const {
    auto it = m_slots.find(slotIndex);
    return it != m_slots.end() ? it->second.locked : false;
}

Core::Result<void, SlotApiError> WindowManagerSlotApiBridge::SetSlotStyle(Core::SlotIndex slotIndex, uint32_t style) {
    auto it = m_slots.find(slotIndex);
    if (it == m_slots.end()) {
        return std::unexpected(SlotApiError::SlotNotFound);
    }
    it->second.style = style;
    return {};
}

Core::Result<void, SlotApiError> WindowManagerSlotApiBridge::EnableSlot(Core::SlotIndex slotIndex, bool enable) {
    auto it = m_slots.find(slotIndex);
    if (it == m_slots.end()) {
        return std::unexpected(SlotApiError::SlotNotFound);
    }
    it->second.enabled = enable;
    return {};
}

bool WindowManagerSlotApiBridge::IsSlotEnabled(Core::SlotIndex slotIndex) const {
    auto it = m_slots.find(slotIndex);
    return it != m_slots.end() ? it->second.enabled : false;
}

Core::Result<void, SlotApiError> WindowManagerSlotApiBridge::SetSlotCover(Core::SlotIndex slotIndex, uint32_t coverType) {
    auto it = m_slots.find(slotIndex);
    if (it == m_slots.end()) {
        return std::unexpected(SlotApiError::SlotNotFound);
    }
    it->second.cover.active = true;
    it->second.cover.coverType = coverType;
    return {};
}

Core::Result<void, SlotApiError> WindowManagerSlotApiBridge::ClearSlotCover(Core::SlotIndex slotIndex) {
    auto it = m_slots.find(slotIndex);
    if (it == m_slots.end()) {
        return std::unexpected(SlotApiError::SlotNotFound);
    }
    it->second.cover.active = false;
    it->second.cover.coverType = 0;
    return {};
}

Core::Result<void, SlotApiError> WindowManagerSlotApiBridge::SwapSlots(Core::SlotIndex slotA, Core::SlotIndex slotB) {
    if (slotA == slotB) return {};

    auto itA = m_slots.find(slotA);
    auto itB = m_slots.find(slotB);
    
    bool aExists = (itA != m_slots.end());
    bool bExists = (itB != m_slots.end());
    
    if (!aExists && !bExists) {
        return std::unexpected(SlotApiError::SlotNotFound);
    }
    
    if (aExists && itA->second.locked) return std::unexpected(SlotApiError::SlotLocked);
    if (bExists && itB->second.locked) return std::unexpected(SlotApiError::SlotLocked);
    
    SlotData tempA;
    if (aExists) tempA = itA->second;
    
    SlotData tempB;
    if (bExists) tempB = itB->second;
    
    if (bExists) {
        m_slots[slotA] = tempB;
        m_slots[slotA].slotIndex = slotA;
    } else {
        m_slots.erase(slotA);
    }
    
    if (aExists) {
        m_slots[slotB] = tempA;
        m_slots[slotB].slotIndex = slotB;
    } else {
        m_slots.erase(slotB);
    }
    
    return {};
}

void WindowManagerSlotApiBridge::Update(float deltaTime) {
    for (auto& [_, slot] : m_slots) {
        for (auto& [layerId, coolTime] : slot.coolTimes) {
            if (coolTime.active && !coolTime.paused) {
                coolTime.elapsed += deltaTime;
                if (coolTime.elapsed >= coolTime.duration) {
                    coolTime.active = false;
                    coolTime.elapsed = coolTime.duration;
                }
            }
        }
    }
}

} // namespace Client::UI
