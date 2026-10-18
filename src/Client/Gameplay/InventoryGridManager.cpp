#include "InventoryGridManager.h"
#include <bit>
#include <algorithm>

namespace Client::Gameplay {

InventoryGridManager::InventoryGridManager(uint16_t width, uint16_t height, uint16_t pages)
    : m_width(width)
    , m_height(height)
    , m_pages(pages)
    , m_pageSize(width * height)
    , m_totalSize(width * height * pages)
{
    if (m_pageSize <= 64) {
        m_pageMasks.resize(m_pages, 0ULL);
    } else {
        m_gridFallback.resize(m_totalSize, false);
    }
}

Client::Core::Result<void, Client::Core::CommandError> InventoryGridManager::CanPlaceItem(EterBase::ItemSlot slot, uint8_t itemHeight) const {
    uint16_t startIdx = slot.get();
    if (startIdx >= m_totalSize || itemHeight == 0) {
        return std::unexpected(Client::Core::CommandError::OutOfRange);
    }

    uint16_t startPage = startIdx / m_pageSize;
    uint16_t localIdx = startIdx % m_pageSize;
    uint16_t localRow = localIdx / m_width;

    if (localRow + itemHeight > m_height) {
        return std::unexpected(Client::Core::CommandError::OutOfRange);
    }

    if (m_pageSize <= 64) {
        uint64_t itemMask = 0ULL;
        for (uint8_t i = 0; i < itemHeight; ++i) {
            itemMask |= (1ULL << (localIdx + i * m_width));
        }

        if ((m_pageMasks[startPage] & itemMask) != 0ULL) {
            return std::unexpected(Client::Core::CommandError::InventoryFull);
        }
    } else {
        for (uint8_t i = 0; i < itemHeight; ++i) {
            uint16_t currentIdx = startIdx + i * m_width;
            if (currentIdx >= m_totalSize || (currentIdx / m_pageSize) != startPage) {
                return std::unexpected(Client::Core::CommandError::OutOfRange);
            }
            if (m_gridFallback[currentIdx]) {
                return std::unexpected(Client::Core::CommandError::InventoryFull);
            }
        }
    }

    return {};
}

Client::Core::Result<EterBase::ItemSlot, Client::Core::CommandError> InventoryGridManager::FindEmptySlot(uint8_t itemHeight) const {
    if (itemHeight == 0 || itemHeight > m_height) {
        return std::unexpected(Client::Core::CommandError::OutOfRange);
    }

    if (m_width == 5 && m_height == 9 && m_pageSize == 45) {
        // SWAR O(1) bitwise parallel check per page
        for (uint16_t page = 0; page < m_pages; ++page) {
            uint64_t B = m_pageMasks[page];
            uint64_t blocked = 0;
            if (itemHeight == 1) {
                blocked = B | 0xFFFFE00000000000ULL; // bits 45..63 blocked
            } else if (itemHeight == 2) {
                blocked = B | (B >> 5) | 0xFFFFFF0000000000ULL; // bits 40..63 blocked
            } else if (itemHeight == 3) {
                blocked = B | (B >> 5) | (B >> 10) | 0xFFFFFFF800000000ULL; // bits 35..63 blocked
            } else {
                for (uint8_t h = 0; h < itemHeight; ++h) {
                    blocked |= (B >> (h * 5));
                }
                uint64_t maxRow = 9 - itemHeight;
                uint64_t validSlots = (maxRow + 1) * 5;
                blocked |= (~((1ULL << validSlots) - 1));
            }

            uint64_t freeBits = ~blocked;
            if (freeBits != 0ULL) {
                int local = std::countr_zero(freeBits);
                if (local < 45) {
                    return EterBase::ItemSlot(page * 45 + local);
                }
            }
        }
        return std::unexpected(Client::Core::CommandError::InventoryFull);
    }

    // General fallback for other dimensions
    for (uint16_t i = 0; i < m_totalSize; ++i) {
        if (CanPlaceItem(EterBase::ItemSlot(i), itemHeight)) {
            return EterBase::ItemSlot(i);
        }
    }
    return std::unexpected(Client::Core::CommandError::InventoryFull);
}

Client::Core::Result<void, Client::Core::CommandError> InventoryGridManager::OccupySlot(EterBase::ItemSlot slot, uint8_t itemHeight) {
    auto res = CanPlaceItem(slot, itemHeight);
    if (!res) {
        return res;
    }

    uint16_t startIdx = slot.get();
    uint16_t startPage = startIdx / m_pageSize;
    uint16_t localIdx = startIdx % m_pageSize;

    if (m_pageSize <= 64) {
        uint64_t itemMask = 0ULL;
        for (uint8_t i = 0; i < itemHeight; ++i) {
            itemMask |= (1ULL << (localIdx + i * m_width));
        }
        m_pageMasks[startPage] |= itemMask;
    } else {
        for (uint8_t i = 0; i < itemHeight; ++i) {
            m_gridFallback[startIdx + i * m_width] = true;
        }
    }

    return {};
}

Client::Core::Result<void, Client::Core::CommandError> InventoryGridManager::FreeSlot(EterBase::ItemSlot slot, uint8_t itemHeight) {
    uint16_t startIdx = slot.get();
    if (startIdx >= m_totalSize || itemHeight == 0) {
        return std::unexpected(Client::Core::CommandError::OutOfRange);
    }

    uint16_t startPage = startIdx / m_pageSize;
    uint16_t localIdx = startIdx % m_pageSize;
    uint16_t localRow = localIdx / m_width;

    if (localRow + itemHeight > m_height) {
        return std::unexpected(Client::Core::CommandError::OutOfRange);
    }

    if (m_pageSize <= 64) {
        uint64_t itemMask = 0ULL;
        for (uint8_t i = 0; i < itemHeight; ++i) {
            itemMask |= (1ULL << (localIdx + i * m_width));
        }

        if ((m_pageMasks[startPage] & itemMask) != itemMask) {
            return std::unexpected(Client::Core::CommandError::SlotEmpty);
        }

        m_pageMasks[startPage] &= ~itemMask;
    } else {
        for (uint8_t i = 0; i < itemHeight; ++i) {
            uint16_t currentIdx = startIdx + i * m_width;
            if (currentIdx >= m_totalSize || (currentIdx / m_pageSize) != startPage) {
                return std::unexpected(Client::Core::CommandError::OutOfRange);
            }
            if (!m_gridFallback[currentIdx]) {
                return std::unexpected(Client::Core::CommandError::SlotEmpty);
            }
        }

        for (uint8_t i = 0; i < itemHeight; ++i) {
            m_gridFallback[startIdx + i * m_width] = false;
        }
    }

    return {};
}

uint64_t InventoryGridManager::GetPageMask(uint16_t page) const {
    if (page < m_pageMasks.size()) {
        return m_pageMasks[page];
    }
    return 0ULL;
}

void InventoryGridManager::Clear() {
    std::fill(m_pageMasks.begin(), m_pageMasks.end(), 0ULL);
    std::fill(m_gridFallback.begin(), m_gridFallback.end(), false);
}

} // namespace Client::Gameplay
