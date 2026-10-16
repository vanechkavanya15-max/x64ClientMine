#pragma once

#include <cstdint>
#include <string_view>
#include <string>
#include <expected>
#include <format>

namespace Client::Terrain {

enum class EnvConfigError : uint8_t {
    Success = 0,
    InvalidSignature,
    MissingValue,
    InvalidFloatFormat,
    InvalidIntFormat,
    UnknownProperty,
    EmptyConfiguration,
};

constexpr std::string_view to_string(EnvConfigError error) noexcept {
    switch (error) {
        case EnvConfigError::Success: return "Success";
        case EnvConfigError::InvalidSignature: return "InvalidSignature";
        case EnvConfigError::MissingValue: return "MissingValue";
        case EnvConfigError::InvalidFloatFormat: return "InvalidFloatFormat";
        case EnvConfigError::InvalidIntFormat: return "InvalidIntFormat";
        case EnvConfigError::UnknownProperty: return "UnknownProperty";
        case EnvConfigError::EmptyConfiguration: return "EmptyConfiguration";
        default: return "UnknownError";
    }
}

struct FogConfig {
    bool enabled{false};
    float start{5000.0f};
    float end{20000.0f};
    float density{0.001f};
    float colorR{0.5f};
    float colorG{0.5f};
    float colorB{0.5f};
};

struct SunConfig {
    bool enabled{false};
    float dirX{0.0f};
    float dirY{0.0f};
    float dirZ{1.0f};
    float colorR{1.0f};
    float colorG{1.0f};
    float colorB{1.0f};
};

struct AmbientLightConfig {
    float colorR{0.2f};
    float colorG{0.2f};
    float colorB{0.2f};
};

struct DayCycleConfig {
    bool nightMode{false};
    float cycleSpeed{1.0f};
};

struct EnvironmentParams {
    FogConfig fog{};
    SunConfig sun{};
    AmbientLightConfig ambient{};
    DayCycleConfig dayCycle{};
};

class TerrainEnvironmentConfigManager {
public:
    TerrainEnvironmentConfigManager() = default;
    ~TerrainEnvironmentConfigManager() = default;

    void SetParams(const EnvironmentParams& params) noexcept { m_params = params; }
    [[nodiscard]] EnvironmentParams GetParams() const noexcept { return m_params; }

    [[nodiscard]] std::expected<void, EnvConfigError> LoadFromMemory(std::string_view content) noexcept;

private:
    EnvironmentParams m_params{};
};

} // namespace Client::Terrain

template <>
struct std::formatter<Client::Terrain::EnvConfigError> : std::formatter<std::string_view> {
    auto format(Client::Terrain::EnvConfigError err, format_context& ctx) const {
        return std::formatter<std::string_view>::format(Client::Terrain::to_string(err), ctx);
    }
};
