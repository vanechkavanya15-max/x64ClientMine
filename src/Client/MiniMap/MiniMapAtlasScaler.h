#pragma once

#include <cstdint>
#include <utility>
#include <expected>
#include <string_view>
#include <format>

namespace Client::MiniMap {

/**
 * @brief Enumy bledow domenowych dla skalowania atlasu minimapy.
 */
enum class ScalerError : uint8_t {
    None = 0,
    InvalidDimensions,
    InvalidBounds,
    OutOfBounds,
    NotInitialized
};

[[nodiscard]] constexpr std::string_view to_string(ScalerError error) noexcept {
    switch (error) {
        case ScalerError::None: return "ScalerError::None";
        case ScalerError::InvalidDimensions: return "ScalerError::InvalidDimensions - Wymiary atlasu musza byc wieksze od zera";
        case ScalerError::InvalidBounds: return "ScalerError::InvalidBounds - Nieprawidlowe granice mapy (min >= max)";
        case ScalerError::OutOfBounds: return "ScalerError::OutOfBounds - Pozycja znajduje sie poza zakresem";
        case ScalerError::NotInitialized: return "ScalerError::NotInitialized - Skaler nie zostal zainicjowany";
        default: return "ScalerError::Unknown";
    }
}

template <typename T>
using ScalerResult = std::expected<T, ScalerError>;

/**
 * @brief Pozycja w wirtualnym swiecie gry.
 */
struct WorldPosition {
    float x;
    float y;

    constexpr bool operator==(const WorldPosition&) const noexcept = default;
};

/**
 * @brief Pozycja w pikselach na teksturze atlasu minimapy.
 */
struct AtlasPosition {
    uint32_t x;
    uint32_t y;

    constexpr bool operator==(const AtlasPosition&) const noexcept = default;
};

/**
 * @brief Klasa odpowiedzialna za konwersje koordynatow miedzy swiatem a atlasem minimapy.
 * Zgodna z zasada SRP, operujaca bezposrednio na matematyce projekcji.
 */
class MiniMapAtlasScaler {
public:
    MiniMapAtlasScaler() noexcept = default;
    ~MiniMapAtlasScaler() = default;

    /**
     * @brief Inicjalizuje skaler wymiarami atlasu oraz granicami swiata.
     * @param atlasWidth Szerokosc tekstury atlasu w pikselach.
     * @param atlasHeight Wysokosc tekstury atlasu w pikselach.
     * @param minWorldX Minimalna wspolrzedna X w swiecie.
     * @param minWorldY Minimalna wspolrzedna Y w swiecie.
     * @param maxWorldX Maksymalna wspolrzedna X w swiecie.
     * @param maxWorldY Maksymalna wspolrzedna Y w swiecie.
     */
    ScalerResult<void> Initialize(uint32_t atlasWidth, uint32_t atlasHeight, float minWorldX, float minWorldY, float maxWorldX, float maxWorldY) noexcept;

    /**
     * @brief Konwertuje pozycje ze swiata na pozycje na atlasie minimapy.
     */
    [[nodiscard]] ScalerResult<AtlasPosition> WorldToAtlas(const WorldPosition& worldPos) const noexcept;

    /**
     * @brief Konwertuje pozycje z atlasu na pozycje w swiecie.
     */
    [[nodiscard]] ScalerResult<WorldPosition> AtlasToWorld(const AtlasPosition& atlasPos) const noexcept;
    
    [[nodiscard]] float GetScaleX() const noexcept { return m_scaleX; }
    [[nodiscard]] float GetScaleY() const noexcept { return m_scaleY; }
    [[nodiscard]] bool IsInitialized() const noexcept { return m_initialized; }

private:
    uint32_t m_atlasWidth{0};
    uint32_t m_atlasHeight{0};
    
    float m_minWorldX{0.0f};
    float m_minWorldY{0.0f};
    float m_maxWorldX{0.0f};
    float m_maxWorldY{0.0f};

    float m_scaleX{0.0f};
    float m_scaleY{0.0f};
    
    bool m_initialized{false};
};

} // namespace Client::MiniMap

template <>
struct std::formatter<Client::MiniMap::ScalerError> : std::formatter<std::string_view> {
    auto format(Client::MiniMap::ScalerError err, std::format_context& ctx) const {
        return std::formatter<std::string_view>::format(Client::MiniMap::to_string(err), ctx);
    }
};
