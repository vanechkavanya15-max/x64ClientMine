#include "src/Client/Combat/FlyingInstance.h"
#include <algorithm>
#include <cmath>
#include <numbers>

namespace Client::Combat {

FlyingInstance::FlyingInstance(ICollisionWorld& collisionWorld)
    : m_collisionWorld(collisionWorld)
{
}

void FlyingInstance::Create(const FlyingDataClean& data, const Collision::CleanVec3& startPos, const FlyTargetClean& target, bool canAttack, float currentTime)
{
    m_data = data;
    m_v3Position = startPos;
    m_target = target;
    m_canAttack = canAttack;
    m_fStartTime = currentTime;
    m_bAlive = true;
    m_bTargetHitted = false;
    m_iPierceCount = data.iPierceCount;
    m_fRemainRange = data.fRange;
    m_v3Accel = data.v3Accel;
    m_hittedObjects.clear();

    // Initial velocity
    m_v3Velocity = Collision::CleanVec3(0.0f, -data.fInitVel, 0.0f);

    // Setup initial direction based on target
    Collision::CleanVec3 targetPos = target.position;
    if (target.IsObject()) {
        targetPos = m_collisionWorld.GetTargetPosition(target.objectId);
    }

    Collision::CleanVec3 dir = targetPos - m_v3Position;
    float dirLenSq = dir.LengthSq();
    if (dirLenSq > 1e-6f) {
        dir = dir / std::sqrt(dirLenSq);
        m_v3Velocity = dir * data.fInitVel;
    }
}

float FlyingInstance::DistanceToLineSegment(const Collision::CleanVec3& point, const Collision::CleanVec3& lineStart, const Collision::CleanVec3& lineEnd) const
{
    Collision::CleanVec3 lineDir = lineEnd - lineStart;
    float lineLenSq = lineDir.LengthSq();
    if (lineLenSq < 1e-6f) {
        return point.Distance(lineStart);
    }

    float t = std::max(0.0f, std::min(1.0f, (point - lineStart).Dot(lineDir) / lineLenSq));
    Collision::CleanVec3 projection = lineStart + lineDir * t;
    return point.Distance(projection);
}

void FlyingInstance::AdjustDirectionForHoming(const Collision::CleanVec3& v3TargetPosition)
{
    Collision::CleanVec3 vTargetDir = v3TargetPosition - m_v3Position;
    float fDistSq = vTargetDir.LengthSq();
    if (fDistSq < 1e-6f || std::isnan(fDistSq))
        return;
    vTargetDir = vTargetDir / std::sqrt(fDistSq);

    float fVelSq = m_v3Velocity.LengthSq();
    if (fVelSq < 1e-6f || std::isnan(fVelSq))
        return;
    Collision::CleanVec3 vVelNorm = m_v3Velocity / std::sqrt(fVelSq);

    Collision::CleanVec3 vDiff = vVelNorm - vTargetDir;
    if (vDiff.LengthSq() < 0.001f)
        return;

    // We implement homing by rotating vVelNorm towards vTargetDir by at most fHomingMaxAngle
    float dot = vVelNorm.Dot(vTargetDir);
    dot = std::max(-1.0f, std::min(1.0f, dot));
    float angle = std::acos(dot);

    float maxAngle = m_data.fHomingMaxAngle * (std::numbers::pi_v<float> / 180.0f);

    if (angle > maxAngle && maxAngle > 0.0f) {
        Collision::CleanVec3 axis = vVelNorm.Cross(vTargetDir);
        float axisLenSq = axis.LengthSq();
        if (axisLenSq > 1e-6f) {
            axis = axis / std::sqrt(axisLenSq);
            // Rodrigues' rotation formula for maxAngle
            float c = std::cos(maxAngle);
            float s = std::sin(maxAngle);

            auto rotate = [axis, c, s](const Collision::CleanVec3& v) {
                return v * c + axis.Cross(v) * s + axis * (axis.Dot(v) * (1.0f - c));
            };

            m_v3Velocity = rotate(m_v3Velocity);
            m_v3Accel = rotate(m_v3Accel);
        }
    } else {
        // Just align velocity with target
        float speed = std::sqrt(fVelSq);
        m_v3Velocity = vTargetDir * speed;
        // Approximation for acceleration alignment
        float accelSq = m_v3Accel.LengthSq();
        if (accelSq > 1e-6f) {
            m_v3Accel = vTargetDir * std::sqrt(accelSq);
        }
    }

    if (std::isnan(m_v3Velocity.x) || std::isnan(m_v3Velocity.y) || std::isnan(m_v3Velocity.z))
    {
        float speed = m_data.fInitVel > 0.0f ? m_data.fInitVel : 1000.0f;
        m_v3Velocity = vTargetDir * speed;
        m_v3Accel = Collision::CleanVec3(0.0f, 0.0f, 0.0f);
    }
}

bool FlyingInstance::Update(float elapsedTime, float currentTime)
{
    if (!m_bAlive)
        return false;

    if (m_data.bIsHoming && m_data.fHomingStartTime + m_fStartTime < currentTime)
    {
        if (m_target.IsObject()) {
            AdjustDirectionForHoming(m_collisionWorld.GetTargetPosition(m_target.objectId));
        }
    }

    Collision::CleanVec3 v3LastPosition = m_v3Position;

    m_v3Velocity += m_v3Accel * elapsedTime;
    m_v3Velocity += Collision::CleanVec3(0.0f, 0.0f, m_data.fGravity) * elapsedTime;

    Collision::CleanVec3 v3Movement = m_v3Velocity * elapsedTime;
    m_v3Position += v3Movement;

    float moveDistance = v3Movement.Length();
    float collisionSphereRadius = std::max(moveDistance * 2.0f, m_data.fCollisionSphereRadius);
    m_fRemainRange -= moveDistance;

    constexpr float MAX_FLY_TTL = 15.0f;
    if (currentTime - m_fStartTime > MAX_FLY_TTL || std::isnan(m_fRemainRange) ||
        std::isnan(m_v3Position.x) || std::isnan(m_v3Position.y) || std::isnan(m_v3Position.z))
    {
        if (m_handler)
            m_handler->OnExplodingOutOfRange();
        Explode(false);
        return false;
    }

    if (m_fRemainRange < 0.0f)
    {
        if (m_handler)
            m_handler->OnExplodingOutOfRange();
        Explode(false);
        return false;
    }

    if (m_target.IsObject())
    {
        if (!m_bTargetHitted)
        {
            Collision::CleanVec3 targetPos = m_collisionWorld.GetTargetPosition(m_target.objectId);
            float distToTargetLine = DistanceToLineSegment(targetPos, v3LastPosition, m_v3Position);

            if (distToTargetLine < m_data.fBombRange)
            {
                m_bTargetHitted = true;

                if (m_handler)
                    m_handler->OnExplodingAtTarget(m_skillIndex);

                if (m_iPierceCount > 0)
                {
                    m_iPierceCount--;
                    Bomb();
                }
                else
                {
                    Explode();
                    return false;
                }
                return true;
            }
        }
    }
    else if (m_target.IsPosition())
    {
        float distToTargetLine = DistanceToLineSegment(m_target.position, v3LastPosition, m_v3Position);
        if (distToTargetLine < m_data.fBombRange)
        {
            Explode();
            return false;
        }
    }

    if (m_data.bHitOnAnotherMonster)
    {
        Collision::CleanDynamicSphere sphere(v3LastPosition, m_v3Position, collisionSphereRadius);
        auto hittedObj = m_collisionWorld.CheckActorCollision(sphere, m_ownerId, m_hittedObjects);
        if (hittedObj.has_value())
        {
            m_hittedObjects.push_back(hittedObj.value());

            if (m_handler)
                m_handler->OnExplodingAtAnotherTarget(m_skillIndex, hittedObj.value());

            if (m_iPierceCount > 0)
            {
                m_iPierceCount--;
                Bomb();
            }
            else
            {
                Explode();
                return false;
            }
            return true;
        }
    }

    if (m_data.bHitOnBackground)
    {
        float groundHeight = m_collisionWorld.GetTerrainHeight(m_v3Position.x, -m_v3Position.y);
        if (groundHeight > m_v3Position.z)
        {
            if (m_handler)
                m_handler->OnExplodingAtBackground();
            Explode();
            return false;
        }

        Collision::CleanDynamicSphere sphere(v3LastPosition, m_v3Position, collisionSphereRadius);
        if (m_collisionWorld.CheckBackgroundCollision(sphere))
        {
            if (m_handler)
                m_handler->OnExplodingAtBackground();
            Explode();
            return false;
        }
    }

    return true;
}

void FlyingInstance::Explode(bool bomb)
{
    if (!m_bAlive) return;
    m_bAlive = false;
    if (bomb) Bomb();
}

void FlyingInstance::Bomb()
{
    // Effect instantiation is intentionally headless for tests
}

} // namespace Client::Combat
