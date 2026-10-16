#include "UIWindowHierarchyController.h"
#include <queue>
#include <algorithm>

namespace Client::UI {

UIWindowHierarchyController::UIWindowHierarchyController(WindowId id)
    : m_id(id), m_zIndex(0)
{
}

void UIWindowHierarchyController::SetZIndex(int zIndex) noexcept {
    m_zIndex = zIndex;
    if (auto parent = m_parent.lock()) {
        parent->SortZOrder();
    }
}

std::expected<void, HierarchyError> UIWindowHierarchyController::AddChild(std::shared_ptr<UIWindowHierarchyController> child) {
    if (!child) {
        return std::unexpected(HierarchyError::InvalidNode);
    }
    
    if (child->GetId() == this->GetId()) {
        return std::unexpected(HierarchyError::CycleDetected);
    }

    if (HasCycle(child)) {
        return std::unexpected(HierarchyError::CycleDetected);
    }

    // Detach from current parent if any
    if (auto oldParent = child->GetParent()) {
        if (oldParent->GetId() == this->GetId()) {
            return {}; // Already a child
        }
        oldParent->RemoveChild(child->GetId());
    }

    child->m_parent = shared_from_this();
    m_children.push_back(child);
    SortZOrder();
    
    return {};
}

std::expected<void, HierarchyError> UIWindowHierarchyController::RemoveChild(WindowId childId) {
    auto it = std::find_if(m_children.begin(), m_children.end(),
        [childId](const std::shared_ptr<UIWindowHierarchyController>& c) {
            return c->GetId() == childId;
        });

    if (it == m_children.end()) {
        return std::unexpected(HierarchyError::NodeNotFound);
    }

    (*it)->m_parent.reset();
    m_children.erase(it);
    
    return {};
}

std::expected<void, HierarchyError> UIWindowHierarchyController::SetParent(std::shared_ptr<UIWindowHierarchyController> parent) {
    if (!parent) {
        DetachFromParent();
        return {};
    }

    if (parent->GetId() == this->GetId()) {
        return std::unexpected(HierarchyError::CycleDetected);
    }

    // If parent is a descendant of this, it's a cycle
    if (this->IsAncestorOf(parent->GetId())) {
        return std::unexpected(HierarchyError::CycleDetected);
    }

    return parent->AddChild(shared_from_this());
}

void UIWindowHierarchyController::DetachFromParent() {
    if (auto parent = m_parent.lock()) {
        parent->RemoveChild(m_id);
    }
}

std::shared_ptr<UIWindowHierarchyController> UIWindowHierarchyController::GetParent() const noexcept {
    return m_parent.lock();
}

std::shared_ptr<UIWindowHierarchyController> UIWindowHierarchyController::FindDescendant(WindowId id) const {
    if (m_id == id) {
        return nullptr; // Exclude self
    }

    std::queue<std::shared_ptr<UIWindowHierarchyController>> q;
    for (const auto& child : m_children) {
        q.push(child);
    }

    while (!q.empty()) {
        auto current = q.front();
        q.pop();

        if (current->GetId() == id) {
            return current;
        }

        for (const auto& child : current->GetChildren()) {
            q.push(child);
        }
    }

    return nullptr;
}

std::expected<void, HierarchyError> UIWindowHierarchyController::RemoveDescendantsIf(const std::function<bool(const UIWindowHierarchyController&)>& predicate) {
    if (!predicate) {
        return std::unexpected(HierarchyError::InvalidNode);
    }

    // Process children
    for (auto it = m_children.begin(); it != m_children.end(); ) {
        if (predicate(**it)) {
            (*it)->m_parent.reset();
            it = m_children.erase(it);
        } else {
            // Recursively check descendants
            (void)(*it)->RemoveDescendantsIf(predicate);
            ++it;
        }
    }

    return {};
}

std::expected<void, HierarchyError> UIWindowHierarchyController::ReserveAddChild(std::shared_ptr<UIWindowHierarchyController> child) {
    if (!child) return std::unexpected(HierarchyError::InvalidNode);
    
    // Check if a reservation already exists
    for (const auto& res : m_reservations) {
        if (res.targetId == child->GetId()) {
            return std::unexpected(HierarchyError::AlreadyReserved);
        }
    }

    m_reservations.push_back({ReservationAction::Add, child->GetId(), child});
    return {};
}

std::expected<void, HierarchyError> UIWindowHierarchyController::ReserveRemoveChild(WindowId childId) {
    for (const auto& res : m_reservations) {
        if (res.targetId == childId && res.action == ReservationAction::Remove) {
            return std::unexpected(HierarchyError::AlreadyReserved);
        }
    }

    m_reservations.push_back({ReservationAction::Remove, childId, nullptr});
    return {};
}

std::expected<void, HierarchyError> UIWindowHierarchyController::ReserveSetParent(std::shared_ptr<UIWindowHierarchyController> parent) {
    if (!parent) return std::unexpected(HierarchyError::InvalidParent);
    
    for (const auto& res : m_reservations) {
        if (res.targetId == parent->GetId() && res.action == ReservationAction::ChangeParent) {
             return std::unexpected(HierarchyError::AlreadyReserved);
        }
    }

    m_reservations.push_back({ReservationAction::ChangeParent, parent->GetId(), parent});
    return {};
}

std::expected<void, HierarchyError> UIWindowHierarchyController::CommitReservations() {
    auto currentReservations = std::move(m_reservations);
    m_reservations.clear(); // Clear so it can queue new reservations safely

    for (const auto& res : currentReservations) {
        switch (res.action) {
            case ReservationAction::Add:
                if (auto r = AddChild(res.node); !r.has_value()) {
                    // Log or handle individual failure? We return first failure
                    return std::unexpected(r.error());
                }
                break;
            case ReservationAction::Remove:
                if (auto r = RemoveChild(res.targetId); !r.has_value()) {
                    return std::unexpected(r.error());
                }
                break;
            case ReservationAction::ChangeParent:
                if (auto r = SetParent(res.node); !r.has_value()) {
                    return std::unexpected(r.error());
                }
                break;
        }
    }

    return {};
}

void UIWindowHierarchyController::ClearReservations() noexcept {
    m_reservations.clear();
}

void UIWindowHierarchyController::SortZOrder() {
    // Stable sort preserves insertion order if z-index is identical
    std::stable_sort(m_children.begin(), m_children.end(), 
        [](const std::shared_ptr<UIWindowHierarchyController>& a, const std::shared_ptr<UIWindowHierarchyController>& b) {
            return a->GetZIndex() < b->GetZIndex();
        });
}

void UIWindowHierarchyController::BringToFront(WindowId childId) {
    auto it = std::find_if(m_children.begin(), m_children.end(),
        [childId](const std::shared_ptr<UIWindowHierarchyController>& c) {
            return c->GetId() == childId;
        });

    if (it != m_children.end()) {
        auto node = *it;
        m_children.erase(it);
        
        // Find max Z-index
        int maxZ = 0;
        if (!m_children.empty()) {
            maxZ = m_children.back()->GetZIndex();
        }
        
        node->m_zIndex = maxZ + 1;
        m_children.push_back(node);
        SortZOrder();
    }
}

void UIWindowHierarchyController::SendToBack(WindowId childId) {
     auto it = std::find_if(m_children.begin(), m_children.end(),
        [childId](const std::shared_ptr<UIWindowHierarchyController>& c) {
            return c->GetId() == childId;
        });

    if (it != m_children.end()) {
        auto node = *it;
        m_children.erase(it);
        
        int minZ = 0;
        if (!m_children.empty()) {
            minZ = m_children.front()->GetZIndex();
        }

        node->m_zIndex = minZ - 1;
        m_children.insert(m_children.begin(), node);
        SortZOrder();
    }
}

void UIWindowHierarchyController::CollectRenderOrder(std::vector<std::shared_ptr<UIWindowHierarchyController>>& outList) const {
    outList.push_back(std::const_pointer_cast<UIWindowHierarchyController>(shared_from_this()));
    for (const auto& child : m_children) {
        child->CollectRenderOrder(outList);
    }
}

std::vector<std::shared_ptr<UIWindowHierarchyController>> UIWindowHierarchyController::GetRenderOrder() const {
    std::vector<std::shared_ptr<UIWindowHierarchyController>> order;
    // Optionally reserve space if tree size is known, but recursive nature makes it dynamic
    CollectRenderOrder(order);
    return order;
}

bool UIWindowHierarchyController::HasCycle(std::shared_ptr<UIWindowHierarchyController> potentialChild) const {
    if (!potentialChild) return false;
    
    // Traverse upwards from *this* to see if potentialChild is an ancestor
    auto current = m_parent.lock();
    while (current) {
        if (current->GetId() == potentialChild->GetId()) {
            return true;
        }
        current = current->GetParent();
    }
    
    return false;
}

bool UIWindowHierarchyController::IsAncestorOf(WindowId potentialDescendantId) const {
    std::queue<std::shared_ptr<UIWindowHierarchyController>> q;
    for (const auto& child : m_children) {
        q.push(child);
    }

    while (!q.empty()) {
        auto current = q.front();
        q.pop();

        if (current->GetId() == potentialDescendantId) {
            return true;
        }

        for (const auto& child : current->GetChildren()) {
            q.push(child);
        }
    }

    return false;
}

} // namespace Client::UI
