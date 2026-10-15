#pragma once

#include <expected>
#include <string_view>
#include <cstdint>
#include <cmath>
#include "Client/Core/StrongTypes.h"

namespace Client::Actor {

enum class CollisionError : uint8_t {
    None = 0,
    InvalidRadius,
    InvalidHeight,
    NoCollision
};

[[nodiscard]] constexpr std::string_view ToString(CollisionError err) noexcept {
    switch (err) {
        case CollisionError::None: return "None";
        case CollisionError::InvalidRadius: return "InvalidRadius";
        case CollisionError::InvalidHeight: return "InvalidHeight";
        case CollisionError::NoCollision: return "NoCollision";
    }
    return "UnknownCollisionError";
}

template <typename T = void>
using CollisionResult = std::expected<T, CollisionError>;

class ActorCollisionComponent {
public:
    constexpr ActorCollisionComponent() noexcept = default;

    constexpr CollisionResult<void> Initialize(float radius, float height) noexcept {
        if (radius <= 0.0f) {
            return std::unexpected(CollisionError::InvalidRadius);
        }
        if (height <= 0.0f) {
            return std::unexpected(CollisionError::InvalidHeight);
        }

        m_radius = radius;
        m_height = height;
        m_isInitialized = true;
        return {};
    }

    [[nodiscard]] constexpr float GetRadius() const noexcept {
        return m_radius;
    }

    [[nodiscard]] constexpr float GetHeight() const noexcept {
        return m_height;
    }

    [[nodiscard]] constexpr bool IsInitialized() const noexcept {
        return m_isInitialized;
    }

    constexpr void SetCenter(const Core::MapCoords& center) noexcept {
        m_center = center;
    }

    [[nodiscard]] constexpr const Core::MapCoords& GetCenter() const noexcept {
        return m_center;
    }

    [[nodiscard]] constexpr bool ContainsPoint(const Core::MapCoords& point) const noexcept {
        if (!m_isInitialized) {
            return false;
        }

        float dx = point.x - m_center.x;
        float dy = point.y - m_center.y;
        float distanceSquared = dx * dx + dy * dy;

        if (distanceSquared > m_radius * m_radius) {
            return false;
        }

        if (point.z < m_center.z || point.z > m_center.z + m_height) {
            return false;
        }

        return true;
    }

    [[nodiscard]] constexpr bool Intersects(const ActorCollisionComponent& other) const noexcept {
        if (!m_isInitialized || !other.m_isInitialized) {
            return false;
        }

        float dx = other.m_center.x - m_center.x;
        float dy = other.m_center.y - m_center.y;
        float distanceSquared = dx * dx + dy * dy;

        float combinedRadius = m_radius + other.m_radius;

        if (distanceSquared > combinedRadius * combinedRadius) {
            return false;
        }

        // Sprawdzanie kolizji na osi Z
        if (m_center.z + m_height < other.m_center.z || other.m_center.z + other.m_height < m_center.z) {
            return false;
        }

        return true;
    }

private:
    Core::MapCoords m_center{};
    float m_radius = 0.0f;
    float m_height = 0.0f;
    bool m_isInitialized = false;
};

} // namespace Client::Actor
