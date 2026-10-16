#pragma once
#include <cstdint>
#include <vector>
#include <memory>
#include <algorithm>
#include <expected>
#include <functional>

#include "Client/Core/StrongTypes.h"

namespace Client::UI {

struct WindowIdTag {};
using WindowId = Client::Core::StrongType<WindowIdTag, uint32_t, 0>;

enum class HierarchyError {
    None,
    InvalidNode,
    InvalidParent,
    CycleDetected,
    NodeNotFound,
    AlreadyReserved,
    NotReserved,
    NodeAlreadyHasParent
};

enum class ReservationAction {
    Add,
    Remove,
    ChangeParent
};

struct HierarchyReservation {
    ReservationAction action;
    WindowId targetId;
    std::shared_ptr<class UIWindowHierarchyController> node; // used for Add/ChangeParent
};

class UIWindowHierarchyController : public std::enable_shared_from_this<UIWindowHierarchyController> {
public:
    UIWindowHierarchyController(WindowId id);
    UIWindowHierarchyController(uint32_t id) : m_id(id) {} // Legacy fallback, to be removed or adapted
    ~UIWindowHierarchyController() = default;

    [[nodiscard]] WindowId GetId() const noexcept { return m_id; }
    [[nodiscard]] WindowId GetIdTyped() const noexcept { return m_id; }
    
    // Z-Order
    void SetZIndex(int zIndex) noexcept;
    [[nodiscard]] int GetZIndex() const noexcept { return m_zIndex; }

    // Hierarchy Management (Immediate)
    std::expected<void, HierarchyError> AddChild(std::shared_ptr<UIWindowHierarchyController> child);
    std::expected<void, HierarchyError> RemoveChild(WindowId childId);
    std::expected<void, HierarchyError> SetParent(std::shared_ptr<UIWindowHierarchyController> parent);
    void DetachFromParent();

    // Querying
    [[nodiscard]] size_t GetChildCount() const noexcept { return m_children.size(); }
    [[nodiscard]] std::shared_ptr<UIWindowHierarchyController> GetParent() const noexcept;
    [[nodiscard]] const std::vector<std::shared_ptr<UIWindowHierarchyController>>& GetChildren() const noexcept { return m_children; }
    [[nodiscard]] std::shared_ptr<UIWindowHierarchyController> FindDescendant(WindowId id) const;

    // Advanced Tree Operations
    std::expected<void, HierarchyError> RemoveDescendantsIf(const std::function<bool(const UIWindowHierarchyController&)>& predicate);
    
    // Reservation System (Deferred actions)
    std::expected<void, HierarchyError> ReserveAddChild(std::shared_ptr<UIWindowHierarchyController> child);
    std::expected<void, HierarchyError> ReserveRemoveChild(WindowId childId);
    std::expected<void, HierarchyError> ReserveSetParent(std::shared_ptr<UIWindowHierarchyController> parent);
    std::expected<void, HierarchyError> CommitReservations();
    void ClearReservations() noexcept;

    // Sorting and Ordering
    void SortZOrder();
    void BringToFront(WindowId childId);
    void SendToBack(WindowId childId);
    
    // Returns flattened list of all descendants in PreOrder (Parent, then Z-sorted Children)
    [[nodiscard]] std::vector<std::shared_ptr<UIWindowHierarchyController>> GetRenderOrder() const;
    
    // Cycle checking
    [[nodiscard]] bool HasCycle(std::shared_ptr<UIWindowHierarchyController> potentialChild) const;
    [[nodiscard]] bool IsAncestorOf(WindowId potentialDescendantId) const;

private:
    WindowId m_id;
    int m_zIndex{0};
    
    std::weak_ptr<UIWindowHierarchyController> m_parent;
    std::vector<std::shared_ptr<UIWindowHierarchyController>> m_children;
    std::vector<HierarchyReservation> m_reservations;

    void CollectRenderOrder(std::vector<std::shared_ptr<UIWindowHierarchyController>>& outList) const;
};

} // namespace Client::UI
