#include "WindowManagerLayoutApiBridge.h"

namespace Client::UI {

std::expected<void, LayoutError> WindowManagerLayoutApiBridge::CreateWindow(uint32_t windowId) {
    if (m_layouts.contains(windowId)) {
        return std::unexpected(LayoutError::WindowAlreadyExists);
    }
    m_layouts[windowId] = WindowLayoutData{.windowId = windowId};
    return {};
}

std::expected<void, LayoutError> WindowManagerLayoutApiBridge::DestroyWindow(uint32_t windowId) {
    auto it = m_layouts.find(windowId);
    if (it == m_layouts.end()) {
        return std::unexpected(LayoutError::WindowNotFound);
    }

    // Detach from parent
    if (it->second.parentId != 0) {
        RemoveChildFromParent(windowId, it->second.parentId);
    }

    // Detach children (they become root windows)
    auto childrenCopy = it->second.children;
    for (uint32_t childId : childrenCopy) {
        if (auto childIt = m_layouts.find(childId); childIt != m_layouts.end()) {
            childIt->second.parentId = 0;
        }
    }

    m_layouts.erase(it);
    return {};
}

std::expected<void, LayoutError> WindowManagerLayoutApiBridge::SetWindowPosition(uint32_t windowId, int32_t x, int32_t y) {
    auto it = m_layouts.find(windowId);
    if (it == m_layouts.end()) return std::unexpected(LayoutError::WindowNotFound);
    it->second.x = x;
    it->second.y = y;
    return {};
}

std::expected<void, LayoutError> WindowManagerLayoutApiBridge::SetWindowSize(uint32_t windowId, int32_t width, int32_t height) {
    auto it = m_layouts.find(windowId);
    if (it == m_layouts.end()) return std::unexpected(LayoutError::WindowNotFound);
    it->second.width = width;
    it->second.height = height;
    return {};
}

std::expected<void, LayoutError> WindowManagerLayoutApiBridge::SetHorizontalAlign(uint32_t windowId, HorizontalAlign align) {
    auto it = m_layouts.find(windowId);
    if (it == m_layouts.end()) return std::unexpected(LayoutError::WindowNotFound);
    it->second.hAlign = align;
    return {};
}

std::expected<void, LayoutError> WindowManagerLayoutApiBridge::SetVerticalAlign(uint32_t windowId, VerticalAlign align) {
    auto it = m_layouts.find(windowId);
    if (it == m_layouts.end()) return std::unexpected(LayoutError::WindowNotFound);
    it->second.vAlign = align;
    return {};
}

std::expected<void, LayoutError> WindowManagerLayoutApiBridge::SetMargin(uint32_t windowId, const Margin& margin) {
    auto it = m_layouts.find(windowId);
    if (it == m_layouts.end()) return std::unexpected(LayoutError::WindowNotFound);
    it->second.margin = margin;
    return {};
}

std::expected<void, LayoutError> WindowManagerLayoutApiBridge::EnableScissorRect(uint32_t windowId, bool enable) {
    auto it = m_layouts.find(windowId);
    if (it == m_layouts.end()) return std::unexpected(LayoutError::WindowNotFound);
    it->second.scissorEnabled = enable;
    return {};
}

std::expected<void, LayoutError> WindowManagerLayoutApiBridge::SetScissorRect(uint32_t windowId, const Rect2D& rect) {
    auto it = m_layouts.find(windowId);
    if (it == m_layouts.end()) return std::unexpected(LayoutError::WindowNotFound);
    it->second.scissorRect = rect;
    return {};
}

std::expected<void, LayoutError> WindowManagerLayoutApiBridge::AttachWindow(uint32_t windowId, uint32_t parentId) {
    if (windowId == parentId) return std::unexpected(LayoutError::InvalidParent);
    auto itWindow = m_layouts.find(windowId);
    if (itWindow == m_layouts.end()) return std::unexpected(LayoutError::WindowNotFound);

    if (parentId != 0) {
        if (!m_layouts.contains(parentId)) return std::unexpected(LayoutError::InvalidParent);
        if (CheckCircularDependency(windowId, parentId)) {
            return std::unexpected(LayoutError::CircularDependencyDetected);
        }
    }

    if (itWindow->second.parentId != 0) {
        RemoveChildFromParent(windowId, itWindow->second.parentId);
    }

    itWindow->second.parentId = parentId;

    if (parentId != 0) {
        auto& parentChildren = m_layouts[parentId].children;
        if (std::find(parentChildren.begin(), parentChildren.end(), windowId) == parentChildren.end()) {
            parentChildren.push_back(windowId);
        }
    }

    return {};
}

std::expected<void, LayoutError> WindowManagerLayoutApiBridge::DetachWindow(uint32_t windowId) {
    auto it = m_layouts.find(windowId);
    if (it == m_layouts.end()) return std::unexpected(LayoutError::WindowNotFound);
    
    if (it->second.parentId != 0) {
        RemoveChildFromParent(windowId, it->second.parentId);
        it->second.parentId = 0;
    }
    return {};
}

std::expected<void, LayoutError> WindowManagerLayoutApiBridge::SetZOrder(uint32_t windowId, int32_t zOrder) {
    auto it = m_layouts.find(windowId);
    if (it == m_layouts.end()) return std::unexpected(LayoutError::WindowNotFound);
    it->second.zOrder = zOrder;
    return {};
}

std::expected<void, LayoutError> WindowManagerLayoutApiBridge::BringToFront(uint32_t windowId) {
    auto it = m_layouts.find(windowId);
    if (it == m_layouts.end()) return std::unexpected(LayoutError::WindowNotFound);

    int32_t maxZ = 0;
    if (it->second.parentId == 0) {
        for (const auto& [id, data] : m_layouts) {
            if (data.parentId == 0 && id != windowId) {
                maxZ = std::max(maxZ, data.zOrder);
            }
        }
    } else {
        auto parentIt = m_layouts.find(it->second.parentId);
        if (parentIt != m_layouts.end()) {
            for (uint32_t siblingId : parentIt->second.children) {
                if (siblingId != windowId) {
                    if (auto siblingIt = m_layouts.find(siblingId); siblingIt != m_layouts.end()) {
                        maxZ = std::max(maxZ, siblingIt->second.zOrder);
                    }
                }
            }
        }
    }
    it->second.zOrder = maxZ + 1;
    return {};
}

std::expected<void, LayoutError> WindowManagerLayoutApiBridge::SendToBack(uint32_t windowId) {
    auto it = m_layouts.find(windowId);
    if (it == m_layouts.end()) return std::unexpected(LayoutError::WindowNotFound);

    int32_t minZ = 0;
    if (it->second.parentId == 0) {
        for (const auto& [id, data] : m_layouts) {
            if (data.parentId == 0 && id != windowId) {
                minZ = std::min(minZ, data.zOrder);
            }
        }
    } else {
        auto parentIt = m_layouts.find(it->second.parentId);
        if (parentIt != m_layouts.end()) {
            for (uint32_t siblingId : parentIt->second.children) {
                if (siblingId != windowId) {
                    if (auto siblingIt = m_layouts.find(siblingId); siblingIt != m_layouts.end()) {
                        minZ = std::min(minZ, siblingIt->second.zOrder);
                    }
                }
            }
        }
    }
    it->second.zOrder = minZ - 1;
    return {};
}

std::expected<Rect2D, LayoutError> WindowManagerLayoutApiBridge::CalculateGlobalRect(uint32_t windowId, int32_t rootWidth, int32_t rootHeight) const {
    auto it = m_layouts.find(windowId);
    if (it == m_layouts.end()) return std::unexpected(LayoutError::WindowNotFound);

    const auto& l = it->second;
    
    int32_t parentX = 0;
    int32_t parentY = 0;
    int32_t parentW = rootWidth;
    int32_t parentH = rootHeight;

    if (l.parentId != 0) {
        auto parentRectRes = CalculateGlobalRect(l.parentId, rootWidth, rootHeight);
        if (!parentRectRes) return std::unexpected(LayoutError::HierarchyError);
        const auto& pr = *parentRectRes;
        parentX = pr.left;
        parentY = pr.top;
        parentW = pr.Width();
        parentH = pr.Height();
    }

    int32_t gx = parentX + l.x;
    int32_t gy = parentY + l.y;
    int32_t gw = l.width;
    int32_t gh = l.height;

    // Horizontal Alignment
    switch (l.hAlign) {
        case HorizontalAlign::Left:
            gx += l.margin.left;
            break;
        case HorizontalAlign::Center:
            gx = parentX + (parentW - gw) / 2 + l.x + l.margin.left - l.margin.right;
            break;
        case HorizontalAlign::Right:
            gx = parentX + parentW - gw - l.x - l.margin.right;
            break;
        case HorizontalAlign::Stretch:
            gx = parentX + l.margin.left + l.x;
            gw = parentW - l.margin.left - l.margin.right - l.x;
            break;
    }

    // Vertical Alignment
    switch (l.vAlign) {
        case VerticalAlign::Top:
            gy += l.margin.top;
            break;
        case VerticalAlign::Center:
            gy = parentY + (parentH - gh) / 2 + l.y + l.margin.top - l.margin.bottom;
            break;
        case VerticalAlign::Bottom:
            gy = parentY + parentH - gh - l.y - l.margin.bottom;
            break;
        case VerticalAlign::Stretch:
            gy = parentY + l.margin.top + l.y;
            gh = parentH - l.margin.top - l.margin.bottom - l.y;
            break;
    }

    return Rect2D{gx, gy, gx + gw, gy + gh};
}

std::expected<std::optional<Rect2D>, LayoutError> WindowManagerLayoutApiBridge::CalculateEffectiveScissorRect(uint32_t windowId, int32_t rootWidth, int32_t rootHeight) const {
    auto it = m_layouts.find(windowId);
    if (it == m_layouts.end()) return std::unexpected(LayoutError::WindowNotFound);

    std::optional<Rect2D> effectiveScissor = std::nullopt;

    if (it->second.scissorEnabled) {
        auto globalRectRes = CalculateGlobalRect(windowId, rootWidth, rootHeight);
        if (!globalRectRes) return std::unexpected(LayoutError::HierarchyError);
        
        // Convert local scissor to global space based on window's global rect
        const auto& lScissor = it->second.scissorRect;
        effectiveScissor = Rect2D{
            globalRectRes->left + lScissor.left,
            globalRectRes->top + lScissor.top,
            globalRectRes->left + lScissor.right,
            globalRectRes->top + lScissor.bottom
        };
    }

    if (it->second.parentId != 0) {
        auto parentScissorRes = CalculateEffectiveScissorRect(it->second.parentId, rootWidth, rootHeight);
        if (!parentScissorRes) return std::unexpected(LayoutError::HierarchyError);
        
        if (parentScissorRes->has_value()) {
            if (effectiveScissor.has_value()) {
                effectiveScissor = effectiveScissor->Intersect(**parentScissorRes);
            } else {
                effectiveScissor = *parentScissorRes;
            }
        }
    }

    return effectiveScissor;
}

std::vector<uint32_t> WindowManagerLayoutApiBridge::GetRenderOrder() const {
    std::vector<uint32_t> order;
    
    // Find roots
    std::vector<uint32_t> roots;
    for (const auto& [id, data] : m_layouts) {
        if (data.parentId == 0) {
            roots.push_back(id);
        }
    }
    
    // Sort roots by z-order
    std::stable_sort(roots.begin(), roots.end(), [this](uint32_t a, uint32_t b) {
        return m_layouts.at(a).zOrder < m_layouts.at(b).zOrder;
    });

    for (uint32_t rootId : roots) {
        BuildRenderOrderRecursive(rootId, order);
    }

    return order;
}

std::optional<uint32_t> WindowManagerLayoutApiBridge::HitTest(int32_t x, int32_t y, int32_t rootWidth, int32_t rootHeight) const {
    auto renderOrder = GetRenderOrder();
    
    // Reverse iterate to test topmost first
    for (auto it = renderOrder.rbegin(); it != renderOrder.rend(); ++it) {
        uint32_t windowId = *it;
        
        auto rectRes = CalculateGlobalRect(windowId, rootWidth, rootHeight);
        if (!rectRes || !rectRes->Contains(x, y)) continue;

        auto scissorRes = CalculateEffectiveScissorRect(windowId, rootWidth, rootHeight);
        if (scissorRes && scissorRes->has_value() && !scissorRes->value().Contains(x, y)) continue;
        
        return windowId;
    }
    return std::nullopt;
}

bool WindowManagerLayoutApiBridge::CheckCircularDependency(uint32_t windowId, uint32_t potentialParentId) const {
    uint32_t currentId = potentialParentId;
    while (currentId != 0) {
        if (currentId == windowId) return true;
        auto it = m_layouts.find(currentId);
        if (it == m_layouts.end()) break;
        currentId = it->second.parentId;
    }
    return false;
}

void WindowManagerLayoutApiBridge::BuildRenderOrderRecursive(uint32_t windowId, std::vector<uint32_t>& order) const {
    order.push_back(windowId);
    
    auto it = m_layouts.find(windowId);
    if (it != m_layouts.end() && !it->second.children.empty()) {
        auto children = it->second.children;
        std::stable_sort(children.begin(), children.end(), [this](uint32_t a, uint32_t b) {
            return m_layouts.at(a).zOrder < m_layouts.at(b).zOrder;
        });
        
        for (uint32_t childId : children) {
            BuildRenderOrderRecursive(childId, order);
        }
    }
}

void WindowManagerLayoutApiBridge::RemoveChildFromParent(uint32_t windowId, uint32_t parentId) {
    if (auto parentIt = m_layouts.find(parentId); parentIt != m_layouts.end()) {
        auto& children = parentIt->second.children;
        auto childIt = std::find(children.begin(), children.end(), windowId);
        if (childIt != children.end()) {
            children.erase(childIt);
        }
    }
}

} // namespace Client::UI
