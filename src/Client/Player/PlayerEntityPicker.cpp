#include "PlayerEntityPicker.h"
#include <algorithm>
#include <cmath>
#include <tuple>

namespace Client::Player {

    Client::Core::ExpectedResult<float, PickingError> PlayerEntityPicker::IntersectRaySphere(const Ray& ray, const BoundingSphere& sphere) noexcept {
        auto m = ray.origin - sphere.center;
        float b = m.Dot(ray.direction);
        float c = m.Dot(m) - sphere.radius * sphere.radius;

        // Exit if ray's origin outside sphere (c > 0) and ray pointing away from sphere (b > 0)
        if (c > 0.0f && b > 0.0f) {
            return Client::Core::MakeUnexpected(PickingError::NoIntersection);
        }

        float discr = b * b - c;
        // A negative discriminant corresponds to ray missing sphere
        if (discr < 0.0f) {
            return Client::Core::MakeUnexpected(PickingError::NoIntersection);
        }

        // Ray now found to intersect sphere, compute smallest t value of intersection
        float t = -b - std::sqrt(discr);

        // If t is negative, ray started inside sphere so clamp t to zero
        if (t < 0.0f) {
            t = 0.0f;
        }

        return Client::Core::MakeExpected<float, PickingError>(std::move(t));
    }

    Client::Core::ExpectedResult<float, PickingError> PlayerEntityPicker::IntersectRayCapsule(const Ray& ray, const BoundingCapsule& capsule) noexcept {
        // Capsule is defined by segment AB and radius.
        Client::Core::MapCoords pa = capsule.point_a;
        Client::Core::MapCoords pb = capsule.point_b;
        float r = capsule.radius;

        Client::Core::MapCoords d = pb - pa;
        Client::Core::MapCoords m = ray.origin - pa;
        Client::Core::MapCoords n = ray.direction;

        float md = m.Dot(d);
        float nd = n.Dot(d);
        float dd = d.Dot(d);

        // If segment length is zero, treat it as a sphere.
        if (dd < 1e-6f) {
            BoundingSphere s{pa, r};
            return IntersectRaySphere(ray, s);
        }

        float nn = n.Dot(n);
        float mn = m.Dot(n);

        float a = dd * nn - nd * nd;
        float k = m.Dot(m) - r * r;
        float c = dd * k - md * md;

        if (std::abs(a) < 1e-6f) {
            // Ray is parallel to the capsule axis.
            if (c > 0.0f) {
                return Client::Core::MakeUnexpected(PickingError::NoIntersection); // outside and parallel
            }
            // Ray is inside the infinite cylinder, check intersections with the endcaps.
            auto tSphereA = IntersectRaySphere(ray, BoundingSphere{pa, r});
            auto tSphereB = IntersectRaySphere(ray, BoundingSphere{pb, r});
            
            float tA = tSphereA ? *tSphereA : std::numeric_limits<float>::max();
            float tB = tSphereB ? *tSphereB : std::numeric_limits<float>::max();

            float t = std::min(tA, tB);
            if (t != std::numeric_limits<float>::max()) {
                return Client::Core::MakeExpected<float, PickingError>(std::move(t));
            }

            return Client::Core::MakeUnexpected(PickingError::NoIntersection);
        }

        float b = dd * mn - nd * md;
        float discr = b * b - a * c;

        if (discr < 0.0f) {
            return Client::Core::MakeUnexpected(PickingError::NoIntersection);
        }

        float t = (-b - std::sqrt(discr)) / a;
        if (t < 0.0f) {
            // We'll just check if origin is inside
            if (c < 0.0f) {
                float y = md + t * nd;
                if (y >= 0.0f && y <= dd) {
                    return Client::Core::MakeExpected<float, PickingError>(0.0f);
                }
            }
        }

        float y = md + t * nd;

        if (y >= 0.0f && y <= dd) {
             if (t >= 0.0f) {
                return Client::Core::MakeExpected<float, PickingError>(std::move(t));
             }
        }

        // Intersect with caps
        auto tSphereA = IntersectRaySphere(ray, BoundingSphere{pa, r});
        auto tSphereB = IntersectRaySphere(ray, BoundingSphere{pb, r});

        float tA = tSphereA ? *tSphereA : std::numeric_limits<float>::max();
        float tB = tSphereB ? *tSphereB : std::numeric_limits<float>::max();

        float minT = std::min(tA, tB);

        if (minT != std::numeric_limits<float>::max()) {
            return Client::Core::MakeExpected<float, PickingError>(std::move(minT));
        }

        return Client::Core::MakeUnexpected(PickingError::NoIntersection);
    }

    Client::Core::ExpectedResult<float, PickingError> PlayerEntityPicker::IntersectRayAABB(const Ray& ray, const AABB& aabb) noexcept {
        float tmin = 0.0f;
        float tmax = std::numeric_limits<float>::max();

        auto checkAxis = [&](float p_origin, float p_dir, float min_val, float max_val) -> bool {
            if (std::abs(p_dir) < 1e-6f) {
                // Ray is parallel to axis
                if (p_origin < min_val || p_origin > max_val) {
                    return false;
                }
            } else {
                float ood = 1.0f / p_dir;
                float t1 = (min_val - p_origin) * ood;
                float t2 = (max_val - p_origin) * ood;

                if (t1 > t2) std::swap(t1, t2);

                if (t1 > tmin) tmin = t1;
                if (t2 < tmax) tmax = t2;

                if (tmin > tmax) return false;
            }
            return true;
        };

        if (!checkAxis(ray.origin.x, ray.direction.x, aabb.min.x, aabb.max.x)) return Client::Core::MakeUnexpected(PickingError::NoIntersection);
        if (!checkAxis(ray.origin.y, ray.direction.y, aabb.min.y, aabb.max.y)) return Client::Core::MakeUnexpected(PickingError::NoIntersection);
        if (!checkAxis(ray.origin.z, ray.direction.z, aabb.min.z, aabb.max.z)) return Client::Core::MakeUnexpected(PickingError::NoIntersection);

        return Client::Core::MakeExpected<float, PickingError>(std::move(tmin));
    }



    void PlayerEntityPicker::RegisterActor(const PickableActor& actor) noexcept {
        auto it = std::find_if(m_actors.begin(), m_actors.end(), [&](const PickableActor& a) { return a.vid == actor.vid; });
        if (it != m_actors.end()) {
            *it = actor;
        } else {
            m_actors.push_back(actor);
        }
    }

    void PlayerEntityPicker::UnregisterActor(Client::Core::EntityVid vid) noexcept {
        m_actors.erase(std::remove_if(m_actors.begin(), m_actors.end(), [&](const PickableActor& a) { return a.vid == vid; }), m_actors.end());
        if (m_pickedActorVid == vid) ClearPickedActor();
    }
    
    void PlayerEntityPicker::RegisterItem(const PickableItem& item) noexcept {
        auto it = std::find_if(m_items.begin(), m_items.end(), [&](const PickableItem& i) { return i.vid == item.vid; });
        if (it != m_items.end()) {
            *it = item;
        } else {
            m_items.push_back(item);
        }
    }

    void PlayerEntityPicker::UnregisterItem(Client::Core::EntityVid vid) noexcept {
        m_items.erase(std::remove_if(m_items.begin(), m_items.end(), [&](const PickableItem& i) { return i.vid == vid; }), m_items.end());
        if (m_pickedItemVid == vid) ClearPickedItem();
    }

    void PlayerEntityPicker::ClearAllRegistries() noexcept {
        m_actors.clear();
        m_items.clear();
        ClearPickedActor();
        ClearPickedItem();
        m_pickedGroundPos.reset();
    }

    void PlayerEntityPicker::UpdatePicking(const Ray& viewRay, std::optional<Client::Core::MapCoords> terrainHit) noexcept {
        ClearPickedActor();
        ClearPickedItem();
        m_pickedGroundPos = terrainHit;

        float closestActorDist = MAX_PICKING_DISTANCE;
        std::optional<Client::Core::EntityVid> closestActorVid;

        for (const auto& actor : m_actors) {
            if (actor.isDead) continue; // Skip dead actors if needed based on rules

            auto result = IntersectRayCapsule(viewRay, actor.bounds);
            if (result.has_value()) {
                float t = result.value();
                if (t >= 0.0f && t < closestActorDist) {
                    closestActorDist = t;
                    closestActorVid = actor.vid;
                }
            }
        }

        float closestItemDist = MAX_PICKING_DISTANCE;
        std::optional<Client::Core::EntityVid> closestItemVid;

        for (const auto& item : m_items) {
            auto result = IntersectRaySphere(viewRay, item.bounds);
            if (result.has_value()) {
                float t = result.value();
                if (t >= 0.0f && t < closestItemDist) {
                    closestItemDist = t;
                    closestItemVid = item.vid;
                }
            }
        }

        // Logic rules: Actor has priority unless Item is much closer
        if (closestActorVid && closestItemVid) {
            if (closestItemDist < closestActorDist - 100.0f) {
                SetPickedItem(*closestItemVid);
            } else {
                SetPickedActor(*closestActorVid);
            }
        } else if (closestActorVid) {
            SetPickedActor(*closestActorVid);
        } else if (closestItemVid) {
            SetPickedItem(*closestItemVid);
        }
    }
} // namespace Client::Player
