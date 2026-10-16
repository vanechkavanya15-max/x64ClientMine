#pragma once
#include <cstdint>
#include <vector>

namespace Client::Bridge {

struct PointChangeData {
    uint8_t type{0};
    int64_t value{0};
    int64_t amount{0};
};

class PhaseGameCharacterStateBridge {
public:
    PhaseGameCharacterStateBridge() = default;
    ~PhaseGameCharacterStateBridge() = default;

    void OnPointChange(uint8_t type, int64_t value, int64_t amount);
    [[nodiscard]] const std::vector<PointChangeData>& GetHistory() const noexcept { return m_history; }

private:
    std::vector<PointChangeData> m_history;
};

} // namespace Client::Bridge
