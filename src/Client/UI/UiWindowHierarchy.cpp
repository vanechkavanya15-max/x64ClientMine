#include "UiWindowHierarchy.hpp"
#include <cassert>
#include <format>
#include <sstream>

namespace Client::UI {

UiWindowHierarchy::UiWindowHierarchy(std::pmr::memory_resource* mr)
    : m_mr(mr ? mr : std::pmr::get_default_resource())
    , m_nodes(m_mr)
    , m_activeHistory(m_mr)
    , m_lockStack(m_mr)
{
}

WindowId UiWindowHierarchy::CreateNode(std::string_view name, WindowFlags flags, int32_t zIndex, void* userData) {
    WindowId id{m_nextId++};
    auto [it, inserted] = m_nodes.try_emplace(id, id, name, flags, zIndex, userData, m_mr);
    assert(inserted && "WindowId must be unique");
    return id;
}

WindowId UiWindowHierarchy::CreateChildNode(WindowId parentId, std::string_view name, WindowFlags flags, int32_t zIndex, void* userData) {
    WindowId id = CreateNode(name, flags, zIndex, userData);
    if (parentId) {
        SetParent(id, parentId);
    }
    return id;
}

bool UiWindowHierarchy::DestroyNode(WindowId id, bool recursive) {
    auto it = m_nodes.find(id);
    if (it == m_nodes.end()) {
        return false;
    }

    if (m_activeWindow == id) {
        m_activeWindow = WindowId{0};
    }
    std::erase(m_activeHistory, id);

    if (m_lockWindow == id) {
        m_lockWindow = WindowId{0};
    }
    std::erase(m_lockStack, id);

    if (recursive) {
        std::pmr::vector<WindowId> childrenCopy(it->second.children, m_mr);
        for (WindowId childId : childrenCopy) {
            DestroyNode(childId, true);
        }
    }

    // Detach from parent
    if (it->second.parentId) {
        auto* parent = GetNode(it->second.parentId);
        if (parent) {
            std::erase(parent->children, id);
        }
    }

    m_nodes.erase(it);
    return true;
}

void UiWindowHierarchy::Clear() {
    m_activeWindow = WindowId{0};
    m_activeHistory.clear();
    m_lockWindow = WindowId{0};
    m_lockStack.clear();
    m_nodes.clear();
    m_nextId = 1;
}

bool UiWindowHierarchy::ContainsNode(WindowId id) const noexcept {
    return m_nodes.contains(id);
}

WindowNode* UiWindowHierarchy::GetNode(WindowId id) noexcept {
    auto it = m_nodes.find(id);
    return (it != m_nodes.end()) ? &it->second : nullptr;
}

const WindowNode* UiWindowHierarchy::GetNode(WindowId id) const noexcept {
    auto it = m_nodes.find(id);
    return (it != m_nodes.end()) ? &it->second : nullptr;
}

size_t UiWindowHierarchy::GetNodeCount() const noexcept {
    return m_nodes.size();
}

bool UiWindowHierarchy::SetParent(WindowId childId, WindowId newParentId) {
    if (childId == newParentId) {
        return false;
    }
    auto* child = GetNode(childId);
    if (!child) {
        return false;
    }

    if (newParentId) {
        auto* newParent = GetNode(newParentId);
        if (!newParent) {
            return false;
        }

        // Cycle check: newParent cannot be a descendant of child
        if (IsDescendantOf(newParentId, childId)) {
            return false;
        }
    }

    // Detach from current parent
    if (child->parentId) {
        auto* oldParent = GetNode(child->parentId);
        if (oldParent) {
            std::erase(oldParent->children, childId);
        }
    }

    child->parentId = newParentId;

    if (newParentId) {
        auto* newParent = GetNode(newParentId);
        if (newParent) {
            newParent->children.push_back(childId);
        }
    }

    UpdateRect(childId);
    return true;
}

bool UiWindowHierarchy::DetachFromParent(WindowId childId) {
    return SetParent(childId, WindowId{0});
}

WindowId UiWindowHierarchy::GetParent(WindowId id) const noexcept {
    const auto* node = GetNode(id);
    return node ? node->parentId : WindowId{0};
}

WindowId UiWindowHierarchy::GetRoot(WindowId id) const noexcept {
    const auto* node = GetNode(id);
    if (!node) return WindowId{0};

    WindowId currentId = id;
    while (true) {
        const auto* cur = GetNode(currentId);
        if (!cur || !cur->parentId) {
            return currentId;
        }
        currentId = cur->parentId;
    }
}

std::span<const WindowId> UiWindowHierarchy::GetChildren(WindowId id) const noexcept {
    const auto* node = GetNode(id);
    if (!node) return {};
    return node->children;
}

size_t UiWindowHierarchy::GetChildCount(WindowId id) const noexcept {
    const auto* node = GetNode(id);
    return node ? node->children.size() : 0;
}

bool UiWindowHierarchy::HasParent(WindowId id) const noexcept {
    const auto* node = GetNode(id);
    return node && node->parentId != WindowId{0};
}

bool UiWindowHierarchy::HasChildren(WindowId id) const noexcept {
    const auto* node = GetNode(id);
    return node && !node->children.empty();
}

bool UiWindowHierarchy::IsChildOf(WindowId childId, WindowId potentialParentId) const noexcept {
    const auto* node = GetNode(childId);
    return node && (node->parentId == potentialParentId);
}

bool UiWindowHierarchy::IsDescendantOf(WindowId childId, WindowId ancestorId) const noexcept {
    if (!childId || !ancestorId) return false;
    WindowId cur = GetParent(childId);
    while (cur) {
        if (cur == ancestorId) return true;
        cur = GetParent(cur);
    }
    return false;
}

bool UiWindowHierarchy::IsAncestorOf(WindowId ancestorId, WindowId childId) const noexcept {
    return IsDescendantOf(childId, ancestorId);
}

bool UiWindowHierarchy::BringToFront(WindowId id) {
    auto* node = GetNode(id);
    if (!node || !node->parentId) return false;

    auto* parent = GetNode(node->parentId);
    if (!parent) return false;

    auto it = std::find(parent->children.begin(), parent->children.end(), id);
    if (it == parent->children.end()) return false;

    parent->children.erase(it);
    parent->children.push_back(id);
    return true;
}

bool UiWindowHierarchy::SendToBack(WindowId id) {
    auto* node = GetNode(id);
    if (!node || !node->parentId) return false;

    auto* parent = GetNode(node->parentId);
    if (!parent) return false;

    auto it = std::find(parent->children.begin(), parent->children.end(), id);
    if (it == parent->children.end()) return false;

    parent->children.erase(it);
    parent->children.insert(parent->children.begin(), id);
    return true;
}

bool UiWindowHierarchy::SetZIndex(WindowId id, int32_t zIndex) {
    auto* node = GetNode(id);
    if (!node) return false;
    node->zIndex = zIndex;
    if (node->parentId) {
        SortChildrenByZOrder(node->parentId);
    }
    return true;
}

void UiWindowHierarchy::SortChildrenByZOrder(WindowId parentId) {
    auto* parent = GetNode(parentId);
    if (!parent) return;

    std::ranges::stable_sort(parent->children, [this](WindowId a, WindowId b) {
        const auto* na = GetNode(a);
        const auto* nb = GetNode(b);
        int32_t za = na ? na->zIndex : 0;
        int32_t zb = nb ? nb->zIndex : 0;
        return za < zb;
    });
}

void UiWindowHierarchy::SortAllChildrenByZOrder() {
    for (auto& [id, node] : m_nodes) {
        if (!node.children.empty()) {
            SortChildrenByZOrder(id);
        }
    }
}

void UiWindowHierarchy::SetPosition(WindowId id, int32_t x, int32_t y) {
    auto* node = GetNode(id);
    if (!node) return;
    node->localX = x;
    node->localY = y;
    UpdateRect(id);
}

void UiWindowHierarchy::SetSize(WindowId id, int32_t width, int32_t height) {
    auto* node = GetNode(id);
    if (!node) return;
    node->width = width;
    node->height = height;
    UpdateRect(id);
}

void UiWindowHierarchy::SetAlign(WindowId id, HorizontalAlign hAlign, VerticalAlign vAlign) {
    auto* node = GetNode(id);
    if (!node) return;
    node->hAlign = hAlign;
    node->vAlign = vAlign;
    UpdateRect(id);
}

void UiWindowHierarchy::SetVisible(WindowId id, bool visible) {
    auto* node = GetNode(id);
    if (!node) return;
    node->visible = visible;
}

void UiWindowHierarchy::SetEnableScissor(WindowId id, bool enable) {
    auto* node = GetNode(id);
    if (!node) return;
    node->enableScissor = enable;
}

void UiWindowHierarchy::SetFlags(WindowId id, WindowFlags flags) {
    auto* node = GetNode(id);
    if (!node) return;
    node->flags = flags;
}

void UiWindowHierarchy::AddFlags(WindowId id, WindowFlags flags) {
    auto* node = GetNode(id);
    if (!node) return;
    node->flags = node->flags | flags;
}

void UiWindowHierarchy::RemoveFlags(WindowId id, WindowFlags flags) {
    auto* node = GetNode(id);
    if (!node) return;
    node->flags = static_cast<WindowFlags>(static_cast<uint32_t>(node->flags) & ~static_cast<uint32_t>(flags));
}

bool UiWindowHierarchy::HasFlag(WindowId id, WindowFlags flag) const noexcept {
    const auto* node = GetNode(id);
    return node && Client::UI::HasFlag(node->flags, flag);
}

void UiWindowHierarchy::UpdateRect(WindowId id) {
    auto* node = GetNode(id);
    if (!node) return;

    int32_t top = node->localY;
    int32_t left = node->localX;

    const WindowNode* parent = node->parentId ? GetNode(node->parentId) : nullptr;
    if (parent) {
        // Vertical alignment
        switch (node->vAlign) {
            case VerticalAlign::Bottom:
                top = parent->height - top;
                break;
            case VerticalAlign::Center:
                top = (parent->height - node->height) / 2 + top;
                break;
            case VerticalAlign::Top:
            default:
                break;
        }
        top += parent->globalRect.top;

        // Horizontal alignment
        if (Client::UI::HasFlag(parent->flags, WindowFlags::Rtl)) {
            left = parent->width - node->width - node->localX;
            switch (node->hAlign) {
                case HorizontalAlign::Right:
                    left = -node->localX;
                    break;
                case HorizontalAlign::Center:
                    left = parent->width / 2 - node->width - node->localX;
                    break;
                case HorizontalAlign::Left:
                default:
                    break;
            }
            left += parent->globalRect.left;
        } else {
            switch (node->hAlign) {
                case HorizontalAlign::Right:
                    left = parent->width - left;
                    break;
                case HorizontalAlign::Center:
                    left = (parent->width - node->width) / 2 + left;
                    break;
                case HorizontalAlign::Left:
                default:
                    break;
            }
            left += parent->globalRect.left;
        }
    }

    node->globalRect.top = top;
    node->globalRect.bottom = top + node->height;
    node->globalRect.left = left;
    node->globalRect.right = left + node->width;

    for (WindowId childId : node->children) {
        UpdateRect(childId);
    }
}

void UiWindowHierarchy::UpdateAllRects() {
    for (const auto& [id, node] : m_nodes) {
        if (!node.parentId) {
            UpdateRect(id);
        }
    }
}

std::optional<UiRect> UiWindowHierarchy::ComputeActiveScissor(WindowId id) const noexcept {
    const WindowNode* node = GetNode(id);
    if (!node) return std::nullopt;

    std::optional<UiRect> currentScissor = std::nullopt;
    WindowId curParentId = node->parentId;

    while (curParentId) {
        const WindowNode* ancestor = GetNode(curParentId);
        if (!ancestor) break;

        if (ancestor->enableScissor) {
            UiRect ancestorRect{
                ancestor->globalRect.left,
                ancestor->globalRect.top,
                ancestor->globalRect.left + ancestor->width,
                ancestor->globalRect.top + ancestor->height
            };

            if (!currentScissor) {
                currentScissor = ancestorRect;
            } else {
                currentScissor = currentScissor->Intersect(ancestorRect);
                if (!currentScissor) {
                    return std::nullopt;
                }
            }
        }
        curParentId = ancestor->parentId;
    }
    return currentScissor;
}

bool UiWindowHierarchy::IsEffectivelyVisible(WindowId id) const noexcept {
    const WindowNode* cur = GetNode(id);
    while (cur) {
        if (!cur->visible) return false;
        cur = cur->parentId ? GetNode(cur->parentId) : nullptr;
    }
    return true;
}

std::optional<WindowId> UiWindowHierarchy::PickWindow(int32_t x, int32_t y, WindowId rootId) const noexcept {
    if (m_lockWindow && rootId == WindowId{0}) {
        rootId = m_lockWindow;
    }

    const WindowNode* root = GetNode(rootId);
    if (!root || !root->visible) return std::nullopt;

    // Traverse children in reverse order (highest z-order / top-most first)
    for (auto it = root->children.rbegin(); it != root->children.rend(); ++it) {
        WindowId childId = *it;
        const WindowNode* child = GetNode(childId);
        if (!child || !child->visible) continue;

        if (!Client::UI::HasFlag(child->flags, WindowFlags::IgnoreSize)) {
            if (!child->globalRect.Contains(x, y)) {
                if (child->width >= 0) continue;
            }
        }

        auto scissor = ComputeActiveScissor(childId);
        if (scissor && !scissor->Contains(x, y)) {
            continue;
        }

        auto picked = PickWindow(x, y, childId);
        if (picked) return picked;
    }

    if (Client::UI::HasFlag(root->flags, WindowFlags::NotPick)) {
        return std::nullopt;
    }

    if (!Client::UI::HasFlag(root->flags, WindowFlags::IgnoreSize)) {
        if (!root->globalRect.Contains(x, y)) {
            return std::nullopt;
        }
    }

    return rootId;
}

std::optional<WindowId> UiWindowHierarchy::PickTopWindow(int32_t x, int32_t y, WindowId rootId) const noexcept {
    const WindowNode* root = GetNode(rootId);
    if (!root || !root->visible) return std::nullopt;

    for (auto it = root->children.rbegin(); it != root->children.rend(); ++it) {
        WindowId childId = *it;
        const WindowNode* child = GetNode(childId);
        if (!child || !child->visible) continue;

        if (child->globalRect.Contains(x, y) && !Client::UI::HasFlag(child->flags, WindowFlags::NotPick)) {
            return childId;
        }
    }
    return std::nullopt;
}

WindowId UiWindowHierarchy::GetActiveWindow() const noexcept {
    return m_activeWindow;
}

bool UiWindowHierarchy::SetActiveWindow(WindowId id) {
    if (id && !ContainsNode(id)) {
        return false;
    }

    if (id == m_activeWindow) {
        return true;
    }

    std::erase(m_activeHistory, id);

    if (m_activeWindow) {
        m_activeHistory.push_back(m_activeWindow);
    }

    m_activeWindow = id;
    return true;
}

void UiWindowHierarchy::ClearActiveWindow() {
    m_activeWindow = WindowId{0};
    m_activeHistory.clear();
}

bool UiWindowHierarchy::DeactivateWindow() {
    if (!m_activeWindow) {
        return false;
    }

    if (m_activeHistory.empty()) {
        m_activeWindow = WindowId{0};
        return true;
    }

    m_activeWindow = m_activeHistory.back();
    m_activeHistory.pop_back();
    return true;
}

std::span<const WindowId> UiWindowHierarchy::GetActiveWindowHistory() const noexcept {
    return m_activeHistory;
}

bool UiWindowHierarchy::LockWindow(WindowId id) {
    if (!ContainsNode(id)) {
        return false;
    }

    if (m_lockWindow == id) {
        return true;
    }

    std::erase(m_lockStack, id);

    if (m_lockWindow) {
        m_lockStack.push_back(m_lockWindow);
    }

    m_lockWindow = id;
    return true;
}

bool UiWindowHierarchy::UnlockWindow() {
    if (!m_lockWindow) {
        return false;
    }

    if (m_lockStack.empty()) {
        m_lockWindow = WindowId{0};
        return true;
    }

    m_lockWindow = m_lockStack.back();
    m_lockStack.pop_back();
    return true;
}

WindowId UiWindowHierarchy::GetLockWindow() const noexcept {
    return m_lockWindow;
}

bool UiWindowHierarchy::IsModalLocked() const noexcept {
    return m_lockWindow != WindowId{0};
}

std::optional<WindowId> UiWindowHierarchy::GetNextFocusable(WindowId currentId) const noexcept {
    std::pmr::vector<WindowId> focusableList(m_mr);

    for (const auto& [id, node] : m_nodes) {
        if (!node.parentId) {
            TraversePreOrder(id, [&](const WindowNode& n) {
                if (n.visible && Client::UI::HasFlag(n.flags, WindowFlags::Focusable)) {
                    focusableList.push_back(n.id);
                }
                return true;
            });
        }
    }

    if (focusableList.empty()) return std::nullopt;

    auto it = std::find(focusableList.begin(), focusableList.end(), currentId);
    if (it == focusableList.end() || std::next(it) == focusableList.end()) {
        return focusableList.front();
    }
    return *std::next(it);
}

std::optional<WindowId> UiWindowHierarchy::GetPrevFocusable(WindowId currentId) const noexcept {
    std::pmr::vector<WindowId> focusableList(m_mr);

    for (const auto& [id, node] : m_nodes) {
        if (!node.parentId) {
            TraversePreOrder(id, [&](const WindowNode& n) {
                if (n.visible && Client::UI::HasFlag(n.flags, WindowFlags::Focusable)) {
                    focusableList.push_back(n.id);
                }
                return true;
            });
        }
    }

    if (focusableList.empty()) return std::nullopt;

    auto it = std::find(focusableList.begin(), focusableList.end(), currentId);
    if (it == focusableList.end() || it == focusableList.begin()) {
        return focusableList.back();
    }
    return *std::prev(it);
}

void UiWindowHierarchy::TraversePreOrder(WindowId rootId, const std::function<bool(const WindowNode&)>& visitor) const {
    const auto* root = GetNode(rootId);
    if (!root) return;

    if (!visitor(*root)) {
        return;
    }

    for (WindowId childId : root->children) {
        TraversePreOrder(childId, visitor);
    }
}

void UiWindowHierarchy::TraversePostOrder(WindowId rootId, const std::function<void(const WindowNode&)>& visitor) const {
    const auto* root = GetNode(rootId);
    if (!root) return;

    for (WindowId childId : root->children) {
        TraversePostOrder(childId, visitor);
    }
    visitor(*root);
}

std::string UiWindowHierarchy::DumpTree(WindowId rootId) const {
    std::ostringstream ss;
    auto dumpNode = [&](auto& self, WindowId id, int indent) -> void {
        const auto* node = GetNode(id);
        if (!node) return;

        for (int i = 0; i < indent; ++i) ss << "  ";
        ss << std::format("[{}] \"{}\" z={} pos=({}, {}) size=({}, {}) rect=({}, {}, {}, {}) vis={} flags=0x{:X}\n",
            node->id.get(),
            std::string_view(node->name),
            node->zIndex,
            node->localX, node->localY,
            node->width, node->height,
            node->globalRect.left, node->globalRect.top, node->globalRect.right, node->globalRect.bottom,
            node->visible ? "true" : "false",
            static_cast<uint32_t>(node->flags)
        );

        for (WindowId childId : node->children) {
            self(self, childId, indent + 1);
        }
    };

    if (rootId) {
        dumpNode(dumpNode, rootId, 0);
    } else {
        for (const auto& [id, node] : m_nodes) {
            if (!node.parentId) {
                dumpNode(dumpNode, id, 0);
            }
        }
    }
    return ss.str();
}

} // namespace Client::UI
