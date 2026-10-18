#pragma once

#include <vector>
#include <memory>
#include <algorithm>
#include <cmath>
#include <cstdint>
#include <limits>
#include "Client/Actors/CollisionMathClean.h"

namespace Client::Collision
{

struct BVHAABB
{
    CleanVec3 min{std::numeric_limits<float>::max(), std::numeric_limits<float>::max(), std::numeric_limits<float>::max()};
    CleanVec3 max{std::numeric_limits<float>::lowest(), std::numeric_limits<float>::lowest(), std::numeric_limits<float>::lowest()};

    [[nodiscard]] CleanVec3 Center() const noexcept
    {
        return (min + max) * 0.5f;
    }

    [[nodiscard]] CleanVec3 Extents() const noexcept
    {
        return (max - min) * 0.5f;
    }

    void Expand(const CleanVec3& point) noexcept
    {
        min.x = std::min(min.x, point.x);
        min.y = std::min(min.y, point.y);
        min.z = std::min(min.z, point.z);

        max.x = std::max(max.x, point.x);
        max.y = std::max(max.y, point.y);
        max.z = std::max(max.z, point.z);
    }

    void Expand(const BVHAABB& other) noexcept
    {
        min.x = std::min(min.x, other.min.x);
        min.y = std::min(min.y, other.min.y);
        min.z = std::min(min.z, other.min.z);

        max.x = std::max(max.x, other.max.x);
        max.y = std::max(max.y, other.max.y);
        max.z = std::max(max.z, other.max.z);
    }

    [[nodiscard]] bool IntersectsAABB(const BVHAABB& other) const noexcept
    {
        if (max.x < other.min.x || min.x > other.max.x) return false;
        if (max.y < other.min.y || min.y > other.max.y) return false;
        if (max.z < other.min.z || min.z > other.max.z) return false;
        return true;
    }

    [[nodiscard]] bool IntersectsSphere(const CleanVec3& center, float radius) const noexcept
    {
        float distSq = 0.0f;
        if (center.x < min.x) distSq += (center.x - min.x) * (center.x - min.x);
        else if (center.x > max.x) distSq += (center.x - max.x) * (center.x - max.x);

        if (center.y < min.y) distSq += (center.y - min.y) * (center.y - min.y);
        else if (center.y > max.y) distSq += (center.y - max.y) * (center.y - max.y);

        if (center.z < min.z) distSq += (center.z - min.z) * (center.z - min.z);
        else if (center.z > max.z) distSq += (center.z - max.z) * (center.z - max.z);

        return distSq <= (radius * radius);
    }

    [[nodiscard]] bool IntersectsSegment(const CleanVec3& p1, const CleanVec3& p2, float radius) const noexcept
    {
        // Utworzenie rozszerzonego AABB o promien
        BVHAABB expanded;
        expanded.min = min - CleanVec3{radius, radius, radius};
        expanded.max = max + CleanVec3{radius, radius, radius};

        CleanVec3 d = p2 - p1;
        float tmin = 0.0f;
        float tmax = 1.0f;

        // Osi X
        if (std::abs(d.x) < 1e-6f)
        {
            if (p1.x < expanded.min.x || p1.x > expanded.max.x) return false;
        }
        else
        {
            float ood = 1.0f / d.x;
            float t1 = (expanded.min.x - p1.x) * ood;
            float t2 = (expanded.max.x - p1.x) * ood;
            if (t1 > t2) std::swap(t1, t2);
            tmin = std::max(tmin, t1);
            tmax = std::min(tmax, t2);
            if (tmin > tmax) return false;
        }

        // Osi Y
        if (std::abs(d.y) < 1e-6f)
        {
            if (p1.y < expanded.min.y || p1.y > expanded.max.y) return false;
        }
        else
        {
            float ood = 1.0f / d.y;
            float t1 = (expanded.min.y - p1.y) * ood;
            float t2 = (expanded.max.y - p1.y) * ood;
            if (t1 > t2) std::swap(t1, t2);
            tmin = std::max(tmin, t1);
            tmax = std::min(tmax, t2);
            if (tmin > tmax) return false;
        }

        // Osi Z
        if (std::abs(d.z) < 1e-6f)
        {
            if (p1.z < expanded.min.z || p1.z > expanded.max.z) return false;
        }
        else
        {
            float ood = 1.0f / d.z;
            float t1 = (expanded.min.z - p1.z) * ood;
            float t2 = (expanded.max.z - p1.z) * ood;
            if (t1 > t2) std::swap(t1, t2);
            tmin = std::max(tmin, t1);
            tmax = std::min(tmax, t2);
            if (tmin > tmax) return false;
        }

        return true;
    }
};

struct BVHSphere
{
    CleanVec3 center{0.0f, 0.0f, 0.0f};
    float radius{0.0f};

    [[nodiscard]] bool Intersects(const BVHSphere& other) const noexcept
    {
        float totalR = radius + other.radius;
        return center.DistanceSq(other.center) <= (totalR * totalR);
    }

    [[nodiscard]] bool IntersectsSegment(const CleanVec3& p1, const CleanVec3& p2, float segRadius) const noexcept
    {
        float totalR = radius + segRadius;
        float totalRSq = totalR * totalR;

        CleanVec3 ab = p2 - p1;
        CleanVec3 ac = center - p1;
        float abLenSq = ab.LengthSq();

        if (abLenSq <= 1e-6f)
        {
            return ac.LengthSq() <= totalRSq;
        }

        float t = ac.Dot(ab) / abLenSq;
        t = std::clamp(t, 0.0f, 1.0f);
        CleanVec3 closest = p1 + ab * t;
        return (center - closest).LengthSq() <= totalRSq;
    }
};

struct BVHSweptSphere
{
    CleanVec3 start{0.0f, 0.0f, 0.0f};
    CleanVec3 end{0.0f, 0.0f, 0.0f};
    float radius{0.0f};

    [[nodiscard]] BVHAABB ComputeAABB() const noexcept
    {
        BVHAABB aabb;
        aabb.min = CleanVec3{
            std::min(start.x, end.x) - radius,
            std::min(start.y, end.y) - radius,
            std::min(start.z, end.z) - radius
        };
        aabb.max = CleanVec3{
            std::max(start.x, end.x) + radius,
            std::max(start.y, end.y) + radius,
            std::max(start.z, end.z) + radius
        };
        return aabb;
    }

    [[nodiscard]] CleanVec3 Velocity() const noexcept
    {
        return end - start;
    }
};

enum class BVHObstacleType : uint8_t
{
    Sphere,
    Wall,
    AABB
};

struct BVHObstacle
{
    uint32_t id{0};
    BVHObstacleType type{BVHObstacleType::Sphere};
    CleanVec3 position{0.0f, 0.0f, 0.0f}; // Center / Anchor
    CleanVec3 normal{0.0f, 0.0f, 1.0f};   // Wall plane normal
    float radius{0.0f};                   // Sphere radius or Wall thickness
    float halfWidth{0.0f};                // Wall half width along X/tangent
    float halfHeight{0.0f};               // Wall half height along Z/bitangent
    BVHAABB bounds;
    bool isWalkable{false};
};

struct BVHContinuousHit
{
    bool hit{false};
    float toi{1.0f};                    // Time of Impact in [0, 1]
    CleanVec3 hitPoint{0.0f, 0.0f, 0.0f};
    CleanVec3 normal{0.0f, 0.0f, 1.0f};
    CleanVec3 resolvedSafePos{0.0f, 0.0f, 0.0f};
    CleanVec3 slideVelocity{0.0f, 0.0f, 0.0f};
    uint32_t obstacleId{0};
};

struct BVHNode
{
    BVHAABB bounds;
    BVHSphere boundingSphere;
    int32_t leftChild{-1};
    int32_t rightChild{-1};
    int32_t obstacleIndex{-1}; // -1 dla wezlow wewnetrznych

    [[nodiscard]] bool IsLeaf() const noexcept
    {
        return leftChild == -1 && rightChild == -1;
    }
};

class SphereTreeBVH
{
public:
    SphereTreeBVH() = default;
    ~SphereTreeBVH() = default;

    void Clear() noexcept
    {
        m_obstacles.clear();
        m_nodes.clear();
        m_rootIndex = -1;
    }

    uint32_t AddSphere(uint32_t id, const CleanVec3& center, float radius, bool isWalkable = false)
    {
        BVHObstacle obs;
        obs.id = id;
        obs.type = BVHObstacleType::Sphere;
        obs.position = center;
        obs.radius = radius;
        obs.isWalkable = isWalkable;

        obs.bounds.min = center - CleanVec3{radius, radius, radius};
        obs.bounds.max = center + CleanVec3{radius, radius, radius};

        m_obstacles.push_back(obs);
        return id;
    }

    uint32_t AddWall(uint32_t id, const CleanVec3& center, const CleanVec3& normal, float halfWidth, float halfHeight, float thickness = 10.0f)
    {
        BVHObstacle obs;
        obs.id = id;
        obs.type = BVHObstacleType::Wall;
        obs.position = center;
        float normLen = normal.Length();
        obs.normal = (normLen > 1e-6f) ? (normal * (1.0f / normLen)) : CleanVec3{0.0f, 1.0f, 0.0f};
        obs.halfWidth = halfWidth;
        obs.halfHeight = halfHeight;
        obs.radius = thickness;
        obs.isWalkable = false;

        // Oszacowanie bounding boxu sciany
        float maxExtent = std::max(halfWidth, halfHeight) + thickness;
        obs.bounds.min = center - CleanVec3{maxExtent, maxExtent, maxExtent};
        obs.bounds.max = center + CleanVec3{maxExtent, maxExtent, maxExtent};

        m_obstacles.push_back(obs);
        return id;
    }

    void Build()
    {
        m_nodes.clear();
        if (m_obstacles.empty())
        {
            m_rootIndex = -1;
            return;
        }

        std::vector<int32_t> indices(m_obstacles.size());
        for (size_t i = 0; i < indices.size(); ++i)
        {
            indices[i] = static_cast<int32_t>(i);
        }

        m_rootIndex = BuildRecursive(indices, 0, static_cast<int32_t>(indices.size()));
    }

    [[nodiscard]] std::vector<uint32_t> QueryRange(const CleanVec3& center, float radius) const
    {
        std::vector<uint32_t> results;
        if (m_rootIndex < 0 || m_nodes.empty()) return results;

        std::vector<int32_t> stack;
        stack.reserve(64);
        stack.push_back(m_rootIndex);

        while (!stack.empty())
        {
            int32_t nodeIdx = stack.back();
            stack.pop_back();
            const auto& node = m_nodes[nodeIdx];

            if (!node.boundingSphere.IntersectsSegment(center, center, radius) &&
                !node.bounds.IntersectsSphere(center, radius))
            {
                continue; // Hierarchical pruning
            }

            if (node.IsLeaf())
            {
                if (node.obstacleIndex >= 0 && static_cast<size_t>(node.obstacleIndex) < m_obstacles.size())
                {
                    const auto& obs = m_obstacles[node.obstacleIndex];
                    if (obs.type == BVHObstacleType::Sphere)
                    {
                        if (obs.position.DistanceSq(center) <= (obs.radius + radius) * (obs.radius + radius))
                        {
                            results.push_back(obs.id);
                        }
                    }
                    else
                    {
                        if (obs.bounds.IntersectsSphere(center, radius))
                        {
                            results.push_back(obs.id);
                        }
                    }
                }
            }
            else
            {
                if (node.leftChild >= 0) stack.push_back(node.leftChild);
                if (node.rightChild >= 0) stack.push_back(node.rightChild);
            }
        }

        return results;
    }

    [[nodiscard]] std::vector<uint32_t> QuerySweptSphere(const BVHSweptSphere& swept) const
    {
        std::vector<uint32_t> results;
        if (m_rootIndex < 0 || m_nodes.empty()) return results;

        BVHAABB sweptAABB = swept.ComputeAABB();

        std::vector<int32_t> stack;
        stack.reserve(64);
        stack.push_back(m_rootIndex);

        while (!stack.empty())
        {
            int32_t nodeIdx = stack.back();
            stack.pop_back();
            const auto& node = m_nodes[nodeIdx];

            // Hierarchiczne odrzucanie (Dual Bounding Volume: Sphere & AABB)
            if (!node.bounds.IntersectsAABB(sweptAABB) ||
                !node.boundingSphere.IntersectsSegment(swept.start, swept.end, swept.radius))
            {
                continue;
            }

            if (node.IsLeaf())
            {
                if (node.obstacleIndex >= 0 && static_cast<size_t>(node.obstacleIndex) < m_obstacles.size())
                {
                    const auto& obs = m_obstacles[node.obstacleIndex];
                    results.push_back(obs.id);
                }
            }
            else
            {
                if (node.leftChild >= 0) stack.push_back(node.leftChild);
                if (node.rightChild >= 0) stack.push_back(node.rightChild);
            }
        }

        return results;
    }

    [[nodiscard]] bool ContinuousDetectCollision(const BVHSweptSphere& swept, BVHContinuousHit& outHit) const
    {
        outHit = BVHContinuousHit{};
        outHit.hit = false;
        outHit.toi = 1.0f;

        if (m_rootIndex < 0 || m_nodes.empty()) return false;

        BVHAABB sweptAABB = swept.ComputeAABB();
        CleanVec3 moveDir = swept.Velocity();
        float moveDist = moveDir.Length();

        std::vector<int32_t> stack;
        stack.reserve(64);
        stack.push_back(m_rootIndex);

        while (!stack.empty())
        {
            int32_t nodeIdx = stack.back();
            stack.pop_back();
            const auto& node = m_nodes[nodeIdx];

            if (!node.bounds.IntersectsAABB(sweptAABB) ||
                !node.boundingSphere.IntersectsSegment(swept.start, swept.end, swept.radius))
            {
                continue; // Pruned
            }

            if (node.IsLeaf())
            {
                if (node.obstacleIndex >= 0 && static_cast<size_t>(node.obstacleIndex) < m_obstacles.size())
                {
                    const auto& obs = m_obstacles[node.obstacleIndex];
                    TestObstacleContinuous(swept, obs, outHit);
                }
            }
            else
            {
                if (node.leftChild >= 0) stack.push_back(node.leftChild);
                if (node.rightChild >= 0) stack.push_back(node.rightChild);
            }
        }

        return outHit.hit;
    }

    [[nodiscard]] BVHContinuousHit ResolveMovementAntiTunneling(
        const CleanVec3& startPos,
        const CleanVec3& targetVelocity,
        float deltaTime,
        float radius,
        float skinWidth = 0.5f) const
    {
        CleanVec3 movement = targetVelocity * deltaTime;
        CleanVec3 endPos = startPos + movement;

        BVHSweptSphere swept;
        swept.start = startPos;
        swept.end = endPos;
        swept.radius = radius;

        BVHContinuousHit hit;
        if (!ContinuousDetectCollision(swept, hit))
        {
            // Brak kolizji na trasie ruchu
            hit.hit = false;
            hit.toi = 1.0f;
            hit.resolvedSafePos = endPos;
            hit.slideVelocity = targetVelocity;
            return hit;
        }

        // Nastapilo uderzenie (kolizja CCD)
        float moveLen = movement.Length();
        float marginToi = (moveLen > 1e-4f) ? (skinWidth / moveLen) : 0.0f;
        float safeToi = std::clamp(hit.toi - marginToi, 0.0f, 1.0f);

        hit.resolvedSafePos = startPos + movement * safeToi + hit.normal * 0.05f;

        // Wektor slizgania wzdluz plaszczyzny uderzenia
        float remainingTime = (1.0f - hit.toi) * deltaTime;
        if (remainingTime > 0.0f)
        {
            CleanVec3 slideDir = targetVelocity - hit.normal * targetVelocity.Dot(hit.normal);
            hit.slideVelocity = slideDir;
        }
        else
        {
            hit.slideVelocity = CleanVec3{0.0f, 0.0f, 0.0f};
        }

        return hit;
    }

    [[nodiscard]] size_t GetNodeCount() const noexcept { return m_nodes.size(); }
    [[nodiscard]] size_t GetObstacleCount() const noexcept { return m_obstacles.size(); }

private:
    int32_t BuildRecursive(std::vector<int32_t>& indices, int32_t start, int32_t count)
    {
        if (count <= 0) return -1;

        int32_t nodeIdx = static_cast<int32_t>(m_nodes.size());
        m_nodes.emplace_back();

        BVHAABB bounds;
        for (int32_t i = start; i < start + count; ++i)
        {
            bounds.Expand(m_obstacles[indices[i]].bounds);
        }

        m_nodes[nodeIdx].bounds = bounds;
        CleanVec3 center = bounds.Center();
        float radius = bounds.Extents().Length();
        m_nodes[nodeIdx].boundingSphere = BVHSphere{center, radius};

        if (count == 1)
        {
            m_nodes[nodeIdx].obstacleIndex = indices[start];
            m_nodes[nodeIdx].leftChild = -1;
            m_nodes[nodeIdx].rightChild = -1;
            return nodeIdx;
        }

        // Podzial wzdluz najdluzszej osi AABB
        CleanVec3 ext = bounds.Extents();
        int axis = 0;
        if (ext.y > ext.x && ext.y > ext.z) axis = 1;
        else if (ext.z > ext.x && ext.z > ext.y) axis = 2;

        std::sort(indices.begin() + start, indices.begin() + start + count,
            [this, axis](int32_t a, int32_t b) {
                CleanVec3 cA = m_obstacles[a].bounds.Center();
                CleanVec3 cB = m_obstacles[b].bounds.Center();
                if (axis == 0) return cA.x < cB.x;
                if (axis == 1) return cA.y < cB.y;
                return cA.z < cB.z;
            });

        int32_t mid = count / 2;
        int32_t leftChild = BuildRecursive(indices, start, mid);
        int32_t rightChild = BuildRecursive(indices, start + mid, count - mid);

        m_nodes[nodeIdx].leftChild = leftChild;
        m_nodes[nodeIdx].rightChild = rightChild;
        m_nodes[nodeIdx].obstacleIndex = -1;

        return nodeIdx;
    }

    void TestObstacleContinuous(const BVHSweptSphere& swept, const BVHObstacle& obs, BVHContinuousHit& bestHit) const
    {
        CleanVec3 d = swept.Velocity();
        float dLenSq = d.LengthSq();

        if (obs.type == BVHObstacleType::Sphere)
        {
            // Swept Sphere vs Static Sphere
            float totalR = swept.radius + obs.radius;
            CleanVec3 m = swept.start - obs.position;
            float b = m.Dot(d);
            float c = m.Dot(m) - (totalR * totalR);

            if (c <= 0.0f)
            {
                // Kolizja w punkcie startowym
                if (0.0f < bestHit.toi || !bestHit.hit)
                {
                    bestHit.hit = true;
                    bestHit.toi = 0.0f;
                    CleanVec3 norm = m;
                    float nLen = norm.Length();
                    bestHit.normal = (nLen > 1e-4f) ? norm * (1.0f / nLen) : CleanVec3{0.0f, 0.0f, 1.0f};
                    bestHit.hitPoint = obs.position + bestHit.normal * obs.radius;
                    bestHit.obstacleId = obs.id;
                }
                return;
            }

            if (dLenSq < 1e-6f) return;

            float discr = b * b - dLenSq * c;
            if (discr < 0.0f) return;

            float t = (-b - std::sqrt(discr)) / dLenSq;
            if (t >= 0.0f && t <= 1.0f && t < bestHit.toi)
            {
                bestHit.hit = true;
                bestHit.toi = t;
                CleanVec3 hitCenter = swept.start + d * t;
                CleanVec3 norm = hitCenter - obs.position;
                float nLen = norm.Length();
                bestHit.normal = (nLen > 1e-4f) ? norm * (1.0f / nLen) : CleanVec3{0.0f, 0.0f, 1.0f};
                bestHit.hitPoint = obs.position + bestHit.normal * obs.radius;
                bestHit.obstacleId = obs.id;
            }
        }
        else if (obs.type == BVHObstacleType::Wall)
        {
            // Swept Sphere vs Planar Wall with finite bounds
            float d0 = (swept.start - obs.position).Dot(obs.normal);
            float d1 = (swept.end - obs.position).Dot(obs.normal);

            float signedRadius = (d0 >= 0.0f) ? swept.radius : -swept.radius;
            float denom = d0 - d1;

            if (std::abs(denom) < 1e-6f) return; // Ruch rownolegly do plaszczyzny sciany

            float t = (d0 - signedRadius) / denom;
            if (t >= 0.0f && t <= 1.0f && t < bestHit.toi)
            {
                CleanVec3 hitCenter = swept.start + d * t;
                CleanVec3 localDiff = hitCenter - obs.position;

                // Sprawdzenie czy punkt uderzenia miesci sie w granicach sciany (halfWidth, halfHeight)
                float latDist = localDiff.Length();
                float maxReach = std::max(obs.halfWidth, obs.halfHeight) + swept.radius;

                if (latDist <= maxReach)
                {
                    bestHit.hit = true;
                    bestHit.toi = t;
                    bestHit.normal = (d0 >= 0.0f) ? obs.normal : (obs.normal * -1.0f);
                    bestHit.hitPoint = hitCenter - bestHit.normal * swept.radius;
                    bestHit.obstacleId = obs.id;
                }
            }
        }
    }

private:
    std::vector<BVHObstacle> m_obstacles;
    std::vector<BVHNode> m_nodes;
    int32_t m_rootIndex{-1};
};

} // namespace Client::Collision
