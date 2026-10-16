#pragma once
#include <cstdint>
#include <string>
#include <vector>

namespace Client::IPC {

struct EntityInfo {
    uint32_t vid{0};
    uint32_t race{0};
    float x{0.0f};
    float y{0.0f};
    int64_t hp{0};
    int64_t maxHp{0};
};

class HarnessWorldQueryRouter {
public:
    HarnessWorldQueryRouter() = default;
    ~HarnessWorldQueryRouter() = default;

    void AddEntity(const EntityInfo& e) { m_entities.push_back(e); }
    void ClearEntities() noexcept { m_entities.clear(); }

    [[nodiscard]] std::string SerializeWorldSnapshot(uint32_t playerVid) const;

private:
    std::vector<EntityInfo> m_entities;
};

} // namespace Client::IPC
