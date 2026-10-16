#pragma once
#include <string>

namespace Client::Terrain {

struct EnvironmentParams {
    float fogStart{5000.0f};
    float fogEnd{20000.0f};
    float fogDensity{0.001f};
    bool nightMode{false};
};

class TerrainEnvironmentConfigManager {
public:
    TerrainEnvironmentConfigManager() = default;
    ~TerrainEnvironmentConfigManager() = default;

    void SetParams(const EnvironmentParams& params) noexcept { m_params = params; }
    [[nodiscard]] EnvironmentParams GetParams() const noexcept { return m_params; }

private:
    EnvironmentParams m_params{};
};

} // namespace Client::Terrain
