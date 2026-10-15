#include "UIWindowHierarchyTree.h"

namespace Client::UI {

std::expected<void, HierarchyError> UIWindowHierarchyTree::AddWindow(WindowId id) {
    if (!id) {
        return std::unexpected(HierarchyError::InvalidWindowId);
    }
    
    if (m_nodes.contains(id)) {
        return std::unexpected(HierarchyError::WindowAlreadyExists);
    }

    m_nodes.emplace(id, WindowNode{id, std::nullopt, {}});
    return {};
}

std::expected<void, HierarchyError> UIWindowHierarchyTree::RemoveWindow(WindowId id) {
    auto it = m_nodes.find(id);
    if (it == m_nodes.end()) {
        return std::unexpected(HierarchyError::WindowNotFound);
    }

    // Remove from parent
    if (it->second.parent.has_value()) {
        auto parentIt = m_nodes.find(it->second.parent.value());
        if (parentIt != m_nodes.end()) {
            auto& siblings = parentIt->second.children;
            siblings.erase(std::remove(siblings.begin(), siblings.end(), id), siblings.end());
        }
    }

    // Disconnect children (make them root level)
    for (auto childId : it->second.children) {
        auto childIt = m_nodes.find(childId);
        if (childIt != m_nodes.end()) {
            childIt->second.parent = std::nullopt;
        }
    }

    m_nodes.erase(it);
    return {};
}

bool UIWindowHierarchyTree::CheckCycle(WindowId childId, WindowId parentId) const {
    // If the proposed parent is the same as the child, it's a cycle
    if (childId == parentId) return true;

    auto currentId = parentId;
    while (true) {
        auto it = m_nodes.find(currentId);
        if (it == m_nodes.end()) break;
        
        if (!it->second.parent.has_value()) break;
        
        currentId = it->second.parent.value();
        if (currentId == childId) {
            return true;
        }
    }
    
    return false;
}

std::expected<void, HierarchyError> UIWindowHierarchyTree::SetParent(WindowId childId, WindowId parentId) {
    if (!childId || !parentId) {
        return std::unexpected(HierarchyError::InvalidWindowId);
    }

    auto childIt = m_nodes.find(childId);
    if (childIt == m_nodes.end()) {
        return std::unexpected(HierarchyError::WindowNotFound);
    }

    auto parentIt = m_nodes.find(parentId);
    if (parentIt == m_nodes.end()) {
        return std::unexpected(HierarchyError::ParentNotFound);
    }

    if (childIt->second.parent.has_value() && childIt->second.parent.value() == parentId) {
        return std::unexpected(HierarchyError::AlreadyChild);
    }

    if (CheckCycle(childId, parentId)) {
        return std::unexpected(HierarchyError::CycleDetected);
    }

    // Remove from old parent if any
    if (childIt->second.parent.has_value()) {
        auto oldParentIt = m_nodes.find(childIt->second.parent.value());
        if (oldParentIt != m_nodes.end()) {
            auto& siblings = oldParentIt->second.children;
            siblings.erase(std::remove(siblings.begin(), siblings.end(), childId), siblings.end());
        }
    }

    // Set new parent
    childIt->second.parent = parentId;
    parentIt->second.children.push_back(childId);

    return {};
}

std::expected<void, HierarchyError> UIWindowHierarchyTree::RemoveParent(WindowId childId) {
    if (!childId) {
        return std::unexpected(HierarchyError::InvalidWindowId);
    }

    auto childIt = m_nodes.find(childId);
    if (childIt == m_nodes.end()) {
        return std::unexpected(HierarchyError::WindowNotFound);
    }

    if (!childIt->second.parent.has_value()) {
        return {}; // Already root, treat as success or no-op
    }

    auto parentId = childIt->second.parent.value();
    auto parentIt = m_nodes.find(parentId);
    if (parentIt != m_nodes.end()) {
        auto& siblings = parentIt->second.children;
        siblings.erase(std::remove(siblings.begin(), siblings.end(), childId), siblings.end());
    }

    childIt->second.parent = std::nullopt;
    return {};
}

std::expected<void, HierarchyError> UIWindowHierarchyTree::BringToFront(WindowId id) {
    if (!id) return std::unexpected(HierarchyError::InvalidWindowId);

    auto it = m_nodes.find(id);
    if (it == m_nodes.end()) return std::unexpected(HierarchyError::WindowNotFound);

    if (!it->second.parent.has_value()) {
        return {}; // No parent, root level conceptually
    }

    auto parentIt = m_nodes.find(it->second.parent.value());
    if (parentIt != m_nodes.end()) {
        auto& siblings = parentIt->second.children;
        auto siblingIt = std::find(siblings.begin(), siblings.end(), id);
        if (siblingIt != siblings.end() && siblingIt != siblings.end() - 1) {
            siblings.erase(siblingIt);
            siblings.push_back(id);
        }
    }

    return {};
}

std::expected<void, HierarchyError> UIWindowHierarchyTree::SendToBack(WindowId id) {
    if (!id) return std::unexpected(HierarchyError::InvalidWindowId);

    auto it = m_nodes.find(id);
    if (it == m_nodes.end()) return std::unexpected(HierarchyError::WindowNotFound);

    if (!it->second.parent.has_value()) {
        return {}; 
    }

    auto parentIt = m_nodes.find(it->second.parent.value());
    if (parentIt != m_nodes.end()) {
        auto& siblings = parentIt->second.children;
        auto siblingIt = std::find(siblings.begin(), siblings.end(), id);
        if (siblingIt != siblings.end() && siblingIt != siblings.begin()) {
            siblings.erase(siblingIt);
            siblings.insert(siblings.begin(), id);
        }
    }

    return {};
}

std::expected<bool, HierarchyError> UIWindowHierarchyTree::IsAncestor(WindowId ancestorId, WindowId childId) const {
    if (!ancestorId || !childId) return std::unexpected(HierarchyError::InvalidWindowId);
    
    if (!m_nodes.contains(ancestorId)) return std::unexpected(HierarchyError::WindowNotFound); // Could technically be parent not found, but it's checking window existence
    if (!m_nodes.contains(childId)) return std::unexpected(HierarchyError::WindowNotFound);

    auto currentId = childId;
    while (true) {
        auto it = m_nodes.find(currentId);
        if (it == m_nodes.end() || !it->second.parent.has_value()) {
            return false;
        }
        
        currentId = it->second.parent.value();
        if (currentId == ancestorId) {
            return true;
        }
    }
    
    return false;
}

std::expected<std::vector<WindowId>, HierarchyError> UIWindowHierarchyTree::GetChildren(WindowId id) const {
    if (!id) return std::unexpected(HierarchyError::InvalidWindowId);

    auto it = m_nodes.find(id);
    if (it == m_nodes.end()) return std::unexpected(HierarchyError::WindowNotFound);

    return it->second.children;
}

std::optional<WindowId> UIWindowHierarchyTree::GetParent(WindowId id) const {
    auto it = m_nodes.find(id);
    if (it != m_nodes.end()) {
        return it->second.parent;
    }
    return std::nullopt;
}

void UIWindowHierarchyTree::Clear() {
    m_nodes.clear();
}

} // namespace Client::UI
