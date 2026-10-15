#pragma once

#include <cstdint>
#include <expected>
#include <vector>
#include <unordered_map>
#include <optional>
#include <algorithm>

#include "Client/Core/StrongTypes.h"

namespace Client::UI {

struct WindowIdTag {};
using WindowId = Client::Core::StrongType<WindowIdTag, uint32_t, 0>;

enum class HierarchyError {
    None,
    WindowAlreadyExists,
    WindowNotFound,
    ParentNotFound,
    CycleDetected,
    InvalidWindowId,
    AlreadyChild,
    NotAChild
};

class UIWindowHierarchyTree {
public:
    UIWindowHierarchyTree() = default;
    ~UIWindowHierarchyTree() = default;

    // Zero copy operations
    UIWindowHierarchyTree(const UIWindowHierarchyTree&) = delete;
    UIWindowHierarchyTree& operator=(const UIWindowHierarchyTree&) = delete;
    UIWindowHierarchyTree(UIWindowHierarchyTree&&) = default;
    UIWindowHierarchyTree& operator=(UIWindowHierarchyTree&&) = default;

    std::expected<void, HierarchyError> AddWindow(WindowId id);
    std::expected<void, HierarchyError> RemoveWindow(WindowId id);

    std::expected<void, HierarchyError> SetParent(WindowId childId, WindowId parentId);
    std::expected<void, HierarchyError> RemoveParent(WindowId childId);

    std::expected<void, HierarchyError> BringToFront(WindowId id);
    std::expected<void, HierarchyError> SendToBack(WindowId id);

    std::expected<bool, HierarchyError> IsAncestor(WindowId ancestorId, WindowId childId) const;

    std::expected<std::vector<WindowId>, HierarchyError> GetChildren(WindowId id) const;
    std::optional<WindowId> GetParent(WindowId id) const;

    void Clear();

private:
    struct WindowNode {
        WindowId id;
        std::optional<WindowId> parent;
        std::vector<WindowId> children;
    };

    std::unordered_map<WindowId, WindowNode> m_nodes;

    bool CheckCycle(WindowId childId, WindowId parentId) const;
};

} // namespace Client::UI
