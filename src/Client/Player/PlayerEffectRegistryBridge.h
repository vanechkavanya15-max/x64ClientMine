#pragma once
#include <cstdint>
#include <string>
#include <unordered_map>

namespace Client::Player {

class PlayerEffectRegistryBridge {
public:
    PlayerEffectRegistryBridge() = default;
    ~PlayerEffectRegistryBridge() = default;

    bool RegisterEffect(uint32_t eid, const std::string& path) {
        m_effects[eid] = path;
        return true;
    }
    [[nodiscard]] std::string GetEffectPath(uint32_t eid) const {
        auto it = m_effects.find(eid);
        return it != m_effects.end() ? it->second : "";
    }

private:
    std::unordered_map<uint32_t, std::string> m_effects;
};

} // namespace Client::Player
