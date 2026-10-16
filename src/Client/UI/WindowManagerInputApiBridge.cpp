#include "WindowManagerInputApiBridge.h"

namespace Client::UI {

void WindowManagerInputApiBridge::AttachIcon(uint32_t type, uint32_t slotNumber, uint32_t itemVnum, uint32_t count) {
    m_attachingIcon.type = type;
    m_attachingIcon.slotNumber = slotNumber;
    m_attachingIcon.itemVnum = itemVnum;
    m_attachingIcon.count = count;
    m_attachingIcon.isAttaching = true;
}

} // namespace Client::UI
