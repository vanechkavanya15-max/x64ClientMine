#include "TerrainEnvironmentConfigManager.h"
#include <charconv>
#include <system_error>

namespace Client::Terrain {

namespace {

std::string_view TrimWhitespace(std::string_view sv) noexcept {
    const auto first = sv.find_first_not_of(" \t\r\n\v\f");
    if (first == std::string_view::npos) return {};
    const auto last = sv.find_last_not_of(" \t\r\n\v\f");
    return sv.substr(first, last - first + 1);
}

std::string_view GetNextToken(std::string_view& content) noexcept {
    content = TrimWhitespace(content);
    if (content.empty()) return {};

    if (content[0] == '"') {
        const auto endQuote = content.find('"', 1);
        if (endQuote == std::string_view::npos) {
            std::string_view token = content;
            content = {};
            return token;
        }
        std::string_view token = content.substr(1, endQuote - 1);
        content.remove_prefix(endQuote + 1);
        return token;
    }

    const auto nextSpace = content.find_first_of(" \t\r\n\v\f");
    std::string_view token = content.substr(0, nextSpace);
    if (nextSpace == std::string_view::npos) {
        content = {};
    } else {
        content.remove_prefix(nextSpace);
    }
    return token;
}

std::expected<float, EnvConfigError> ParseFloat(std::string_view token) noexcept {
    float result = 0.0f;
    const auto [ptr, ec] = std::from_chars(token.data(), token.data() + token.size(), result);
    if (ec != std::errc{} || ptr != token.data() + token.size()) {
        return std::unexpected(EnvConfigError::InvalidFloatFormat);
    }
    return result;
}

std::expected<int, EnvConfigError> ParseInt(std::string_view token) noexcept {
    int result = 0;
    const auto [ptr, ec] = std::from_chars(token.data(), token.data() + token.size(), result);
    if (ec != std::errc{} || ptr != token.data() + token.size()) {
        return std::unexpected(EnvConfigError::InvalidIntFormat);
    }
    return result;
}

} // namespace

std::expected<void, EnvConfigError> TerrainEnvironmentConfigManager::LoadFromMemory(std::string_view content) noexcept {
    content = TrimWhitespace(content);
    if (content.empty()) {
        return std::unexpected(EnvConfigError::EmptyConfiguration);
    }

    EnvironmentParams params{};

    auto getNextValidToken = [&]() -> std::string_view {
        while (!content.empty()) {
            content = TrimWhitespace(content);
            if (content.empty()) return {};
            if (content.starts_with('#') || content.starts_with("//")) {
                const auto nextLine = content.find_first_of("\n");
                if (nextLine == std::string_view::npos) {
                    content = {};
                    return {};
                }
                content.remove_prefix(nextLine + 1);
                continue;
            }
            return GetNextToken(content);
        }
        return {};
    };

    const auto signatureToken = getNextValidToken();
    if (signatureToken != "ScriptType") {
        return std::unexpected(EnvConfigError::InvalidSignature);
    }
    const auto envDataToken = getNextValidToken();
    if (envDataToken != "EnvironmentData") {
        return std::unexpected(EnvConfigError::InvalidSignature);
    }

    std::string_view token;
    while (!(token = getNextValidToken()).empty()) {
        if (token == "FogEnable") {
            const auto valToken = getNextValidToken();
            if (valToken.empty()) return std::unexpected(EnvConfigError::MissingValue);
            auto val = ParseInt(valToken);
            if (!val) return std::unexpected(val.error());
            params.fog.enabled = (*val != 0);
        } else if (token == "FogStart") {
            const auto valToken = getNextValidToken();
            if (valToken.empty()) return std::unexpected(EnvConfigError::MissingValue);
            auto val = ParseFloat(valToken);
            if (!val) return std::unexpected(val.error());
            params.fog.start = *val;
        } else if (token == "FogEnd") {
            const auto valToken = getNextValidToken();
            if (valToken.empty()) return std::unexpected(EnvConfigError::MissingValue);
            auto val = ParseFloat(valToken);
            if (!val) return std::unexpected(val.error());
            params.fog.end = *val;
        } else if (token == "FogDensity") {
            const auto valToken = getNextValidToken();
            if (valToken.empty()) return std::unexpected(EnvConfigError::MissingValue);
            auto val = ParseFloat(valToken);
            if (!val) return std::unexpected(val.error());
            params.fog.density = *val;
        } else if (token == "FogColor") {
            const auto rToken = getNextValidToken();
            if (rToken.empty()) return std::unexpected(EnvConfigError::MissingValue);
            auto r = ParseFloat(rToken);
            if (!r) return std::unexpected(r.error());

            const auto gToken = getNextValidToken();
            if (gToken.empty()) return std::unexpected(EnvConfigError::MissingValue);
            auto g = ParseFloat(gToken);
            if (!g) return std::unexpected(g.error());

            const auto bToken = getNextValidToken();
            if (bToken.empty()) return std::unexpected(EnvConfigError::MissingValue);
            auto b = ParseFloat(bToken);
            if (!b) return std::unexpected(b.error());

            params.fog.colorR = *r;
            params.fog.colorG = *g;
            params.fog.colorB = *b;
        } else if (token == "SunEnable") {
            const auto valToken = getNextValidToken();
            if (valToken.empty()) return std::unexpected(EnvConfigError::MissingValue);
            auto val = ParseInt(valToken);
            if (!val) return std::unexpected(val.error());
            params.sun.enabled = (*val != 0);
        } else if (token == "SunDir") {
            const auto xToken = getNextValidToken();
            if (xToken.empty()) return std::unexpected(EnvConfigError::MissingValue);
            auto x = ParseFloat(xToken);
            if (!x) return std::unexpected(x.error());

            const auto yToken = getNextValidToken();
            if (yToken.empty()) return std::unexpected(EnvConfigError::MissingValue);
            auto y = ParseFloat(yToken);
            if (!y) return std::unexpected(y.error());

            const auto zToken = getNextValidToken();
            if (zToken.empty()) return std::unexpected(EnvConfigError::MissingValue);
            auto z = ParseFloat(zToken);
            if (!z) return std::unexpected(z.error());

            params.sun.dirX = *x;
            params.sun.dirY = *y;
            params.sun.dirZ = *z;
        } else if (token == "SunColor") {
            const auto rToken = getNextValidToken();
            if (rToken.empty()) return std::unexpected(EnvConfigError::MissingValue);
            auto r = ParseFloat(rToken);
            if (!r) return std::unexpected(r.error());

            const auto gToken = getNextValidToken();
            if (gToken.empty()) return std::unexpected(EnvConfigError::MissingValue);
            auto g = ParseFloat(gToken);
            if (!g) return std::unexpected(g.error());

            const auto bToken = getNextValidToken();
            if (bToken.empty()) return std::unexpected(EnvConfigError::MissingValue);
            auto b = ParseFloat(bToken);
            if (!b) return std::unexpected(b.error());

            params.sun.colorR = *r;
            params.sun.colorG = *g;
            params.sun.colorB = *b;
        } else if (token == "AmbientColor") {
            const auto rToken = getNextValidToken();
            if (rToken.empty()) return std::unexpected(EnvConfigError::MissingValue);
            auto r = ParseFloat(rToken);
            if (!r) return std::unexpected(r.error());

            const auto gToken = getNextValidToken();
            if (gToken.empty()) return std::unexpected(EnvConfigError::MissingValue);
            auto g = ParseFloat(gToken);
            if (!g) return std::unexpected(g.error());

            const auto bToken = getNextValidToken();
            if (bToken.empty()) return std::unexpected(EnvConfigError::MissingValue);
            auto b = ParseFloat(bToken);
            if (!b) return std::unexpected(b.error());

            params.ambient.colorR = *r;
            params.ambient.colorG = *g;
            params.ambient.colorB = *b;
        } else if (token == "NightMode") {
            const auto valToken = getNextValidToken();
            if (valToken.empty()) return std::unexpected(EnvConfigError::MissingValue);
            auto val = ParseInt(valToken);
            if (!val) return std::unexpected(val.error());
            params.dayCycle.nightMode = (*val != 0);
        } else if (token == "DayCycleSpeed") {
            const auto valToken = getNextValidToken();
            if (valToken.empty()) return std::unexpected(EnvConfigError::MissingValue);
            auto val = ParseFloat(valToken);
            if (!val) return std::unexpected(val.error());
            params.dayCycle.cycleSpeed = *val;
        } else {
            return std::unexpected(EnvConfigError::UnknownProperty);
        }
    }

    m_params = params;
    return {};
}

} // namespace Client::Terrain
