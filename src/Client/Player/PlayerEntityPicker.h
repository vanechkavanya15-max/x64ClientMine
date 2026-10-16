#pragma once
#include <cstdint>
#include <optional>
#include <vector>
#include <limits>
#include "../Core/StrongTypes.h"
#include "../Core/ExpectedResult.h"

namespace Client::Player {

    struct Ray {
        Client::Core::MapCoords origin;
        Client::Core::MapCoords direction;
    };

    struct BoundingSphere {
        Client::Core::MapCoords center;
        float radius;
    };

    struct BoundingCapsule {
        Client::Core::MapCoords point_a;
        Client::Core::MapCoords point_b;
        float radius;
    };

    struct AABB {
        Client::Core::MapCoords min;
        Client::Core::MapCoords max;
    };

    struct PickableActor {
        Client::Core::EntityVid vid;
        Client::Core::MapCoords position;
        BoundingCapsule bounds;
        bool isDead{false};
    };

    struct PickableItem {
        Client::Core::EntityVid vid;
        Client::Core::MapCoords position;
        BoundingSphere bounds;
    };

    enum class PickingError {
        InvalidRay,
        NoIntersection
    };

    class PlayerEntityPicker {
    public:
        PlayerEntityPicker() = default;
        ~PlayerEntityPicker() = default;

        void RegisterActor(const PickableActor& actor) noexcept;
        void UnregisterActor(Client::Core::EntityVid vid) noexcept;
        
        void RegisterItem(const PickableItem& item) noexcept;
        void UnregisterItem(Client::Core::EntityVid vid) noexcept;

        void UpdatePicking(const Ray& viewRay, std::optional<Client::Core::MapCoords> terrainHit) noexcept;

        void SetPickedActor(Client::Core::EntityVid vid) noexcept { m_pickedActorVid = vid; }
        void ClearPickedActor() noexcept { m_pickedActorVid.reset(); }
        [[nodiscard]] std::optional<Client::Core::EntityVid> GetPickedActor() const noexcept { return m_pickedActorVid; }

        void SetPickedItem(Client::Core::EntityVid vid) noexcept { m_pickedItemVid = vid; }
        void ClearPickedItem() noexcept { m_pickedItemVid.reset(); }
        [[nodiscard]] std::optional<Client::Core::EntityVid> GetPickedItem() const noexcept { return m_pickedItemVid; }

        [[nodiscard]] std::optional<Client::Core::MapCoords> GetPickedGroundPos() const noexcept { return m_pickedGroundPos; }

        void ClearAllRegistries() noexcept;

        // Mathematical utility functions (stateless) exposed for testing
        static Client::Core::ExpectedResult<float, PickingError> IntersectRaySphere(const Ray& ray, const BoundingSphere& sphere) noexcept;
        static Client::Core::ExpectedResult<float, PickingError> IntersectRayCapsule(const Ray& ray, const BoundingCapsule& capsule) noexcept;
        static Client::Core::ExpectedResult<float, PickingError> IntersectRayAABB(const Ray& ray, const AABB& aabb) noexcept;

    private:
        std::optional<Client::Core::EntityVid> m_pickedActorVid;
        std::optional<Client::Core::EntityVid> m_pickedItemVid;
        std::optional<Client::Core::MapCoords> m_pickedGroundPos;

        std::vector<PickableActor> m_actors;
        std::vector<PickableItem> m_items;

        static constexpr float MAX_PICKING_DISTANCE = 5000.0f;
    };

} // namespace Client::Player
