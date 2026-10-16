#include "WindowManagerLayoutApiBridge.h"

namespace Client::UI {

bool WindowManagerLayoutApiBridge::SetWindowPosition(uint32_t windowId, int32_t x, int32_t y) {
    auto& l = m_layouts[windowId];
    l.windowId = windowId;
    l.x = x;
    l.y = y;
    return true;
}

bool WindowManagerLayoutApiBridge::SetWindowSize(uint32_t windowId, int32_t width, int32_t height) {
    auto& l = m_layouts[windowId];
    l.windowId = windowId;
    l.width = width;
    l.height = height;
    return true;
}

bool WindowManagerLayoutApiBridge::SetHorizontalAlign(uint32_t windowId, HorizontalAlign align) {
    auto it = m_layouts.find(windowId);
    if (it == m_layouts.end()) return false;
    it->second.hAlign = align;
    return true;
}

bool WindowManagerLayoutApiBridge::SetVerticalAlign(uint32_t windowId, VerticalAlign align) {
    auto it = m_layouts.find(windowId);
    if (it == m_layouts.end()) return false;
    it->second.vAlign = align;
    return true;
}

bool WindowManagerLayoutApiBridge::EnableScissorRect(uint32_t windowId, bool enable) {
    auto it = m_layouts.find(windowId);
    if (it == m_layouts.end()) return false;
    it->second.scissorEnabled = enable;
    return true;
}

bool WindowManagerLayoutApiBridge::SetScissorRect(uint32_t windowId, const Rect2D& rect) {
    auto it = m_layouts.find(windowId);
    if (it == m_layouts.end()) return false;
    it->second.scissorRect = rect;
    return true;
}

Rect2D WindowManagerLayoutApiBridge::CalculateGlobalRect(uint32_t windowId, int32_t parentWidth, int32_t parentHeight) const {
    auto it = m_layouts.find(windowId);
    if (it == m_layouts.end()) return Rect2D{};
    
    const auto& l = it->second;
    int32_t gx = l.x;
    int32_t gy = l.y;
    
    if (l.hAlign == HorizontalAlign::Center) {
        gx = (parentWidth - l.width) / 2 + l.x;
    } else if (l.hAlign == HorizontalAlign::Right) {
        gx = parentWidth - l.width - l.x;
    }
    
    if (l.vAlign == VerticalAlign::Center) {
        gy = (parentHeight - l.height) / 2 + l.y;
    } else if (l.vAlign == VerticalAlign::Bottom) {
        gy = parentHeight - l.height - l.y;
    }
    
    return Rect2D{ gx, gy, gx + l.width, gy + l.height };
}

} // namespace Client::UI
