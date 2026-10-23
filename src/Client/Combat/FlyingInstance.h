#pragma once

#include "src/Client/Actors/CollisionMathClean.h"
#include <vector>
#include <optional>
#include <cstdint>
#include <cmath>

namespace Client::Combat {

struct FlyingDataClean {
    float fInitVel{0.0f};
    float fGravity{0.0f};
    float fBombRange{0.0f};
    bool bIsHoming{false};
    float fHomingMaxAngle{0.0f};
    float fHomingStartTime{0.0f};
    bool bHitOnBackground{false};
    bool bHitOnAnotherMonster{false};
    int iPierceCount{0};
    float fCollisionSphereRadius{0.0f};
    float fRange{0.0f};
    Collision::CleanVec3 v3Accel{0.0f, 0.0f, 0.0f};
};

struct FlyTargetClean {
    enum class Type { NONE, OBJECT, POSITION };
    Type type{Type::NONE};
    Collision::CleanVec3 position{0.0f, 0.0f, 0.0f};
    uint32_t objectId{0};

    bool IsObject() const { return type == Type::OBJECT; }
    bool IsPosition() const { return type == Type::POSITION; }
};

class IFlyEventHandlerClean {
public:
    virtual ~IFlyEventHandlerClean() = default;
    virtual void OnExplodingOutOfRange() = 0;
    virtual void OnExplodingAtTarget(uint32_t skillIndex) = 0;
    virtual void OnExplodingAtAnotherTarget(uint32_t skillIndex, uint32_t victimId) = 0;
    virtual void OnExplodingAtBackground() = 0;
};

class ICollisionWorld {
public:
    virtual ~ICollisionWorld() = default;
    virtual bool CheckBackgroundCollision(const Collision::CleanDynamicSphere& sphere) = 0;
    virtual std::optional<uint32_t> CheckActorCollision(const Collision::CleanDynamicSphere& sphere, uint32_t ownerId, const std::vector<uint32_t>& ignoreList) = 0;
    virtual float GetTerrainHeight(float x, float y) = 0;
    virtual Collision::CleanVec3 GetTargetPosition(uint32_t targetId) = 0;
};

class FlyingInstance {
public:
    FlyingInstance(ICollisionWorld& collisionWorld);
    ~FlyingInstance() = default;

    void Create(const FlyingDataClean& data, const Collision::CleanVec3& startPos, const FlyTargetClean& target, bool canAttack, float currentTime);
    bool Update(float elapsedTime, float currentTime);

    void SetEventHandler(IFlyEventHandlerClean* handler) { m_handler = handler; }
    void SetOwnerId(uint32_t ownerId) { m_ownerId = ownerId; }
    void SetSkillIndex(uint32_t skillIndex) { m_skillIndex = skillIndex; }

    const Collision::CleanVec3& GetPosition() const { return m_v3Position; }
    bool IsAlive() const { return m_bAlive; }

private:
    void AdjustDirectionForHoming(const Collision::CleanVec3& targetPos);
    void Explode(bool bomb = true);
    void Bomb();
    float DistanceToLineSegment(const Collision::CleanVec3& point, const Collision::CleanVec3& lineStart, const Collision::CleanVec3& lineEnd) const;

    ICollisionWorld& m_collisionWorld;
    FlyingDataClean m_data;
    FlyTargetClean m_target;
    IFlyEventHandlerClean* m_handler{nullptr};

    uint32_t m_ownerId{0};
    uint32_t m_skillIndex{0};

    float m_fStartTime{0.0f};
    bool m_bAlive{false};
    bool m_canAttack{false};
    bool m_bTargetHitted{false};
    int m_iPierceCount{0};

    Collision::CleanVec3 m_v3Position;
    Collision::CleanVec3 m_v3Velocity;
    Collision::CleanVec3 m_v3Accel;
    float m_fRemainRange{0.0f};

    std::vector<uint32_t> m_hittedObjects;
};

} // namespace Client::Combat
