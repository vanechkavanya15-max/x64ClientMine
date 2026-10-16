#pragma once
#include <cstdint>
#include <vector>
#include <memory>
#include <algorithm>

namespace Client::UI {

class UIWindowHierarchyController {
public:
    UIWindowHierarchyController(uint32_t id = 0) : m_id(id) {}
    ~UIWindowHierarchyController() = default;

    [[nodiscard]] uint32_t GetId() const noexcept { return m_id; }
    void AddChild(std::shared_ptr<UIWindowHierarchyController> child) {
        if (child) m_children.push_back(child);
    }
    void RemoveChild(uint32_t childId) {
        m_children.erase(
            std::remove_if(m_children.begin(), m_children.end(), [childId](const auto& c) { return c->GetId() == childId; }),
            m_children.end()
        );
    }
    [[nodiscard]] size_t GetChildCount() const noexcept { return m_children.size(); }

private:
    uint32_t m_id{0};
    std::vector<std::shared_ptr<UIWindowHierarchyController>> m_children;
};

} // namespace Client::UI
