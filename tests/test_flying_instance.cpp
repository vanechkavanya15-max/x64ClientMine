#define DOCTEST_CONFIG_IMPLEMENT_WITH_MAIN
#include "doctest.h"
#include "src/Client/Combat/FlyingInstance.h"
#include <algorithm>

using namespace Client::Combat;
using namespace Client::Collision;

class MockFlyEventHandler : public IFlyEventHandlerClean {
public:
    bool explodedOutOfRange = false;
    bool explodedAtTarget = false;
    bool explodedAtAnotherTarget = false;
    bool explodedAtBackground = false;
    uint32_t lastSkillIndex = 0;
    uint32_t lastVictimId = 0;

    void OnExplodingOutOfRange() override { explodedOutOfRange = true; }
    void OnExplodingAtTarget(uint32_t skillIndex) override {
        explodedAtTarget = true;
        lastSkillIndex = skillIndex;
    }
    void OnExplodingAtAnotherTarget(uint32_t skillIndex, uint32_t victimId) override {
        explodedAtAnotherTarget = true;
        lastSkillIndex = skillIndex;
        lastVictimId = victimId;
    }
    void OnExplodingAtBackground() override { explodedAtBackground = true; }
};

class MockCollisionWorld : public ICollisionWorld {
public:
    CleanVec3 targetPos{100.0f, 0.0f, 0.0f};
    bool simulateBackgroundCollision = false;
    bool simulateActorCollision = false;
    uint32_t hitActorId = 2;
    float terrainHeight = -100.0f;

    bool CheckBackgroundCollision(const CleanDynamicSphere&) override { return simulateBackgroundCollision; }
    std::optional<uint32_t> CheckActorCollision(const CleanDynamicSphere&, uint32_t ownerId, const std::vector<uint32_t>& ignoreList) override {
        if (simulateActorCollision && std::find(ignoreList.begin(), ignoreList.end(), hitActorId) == ignoreList.end()) {
            return hitActorId;
        }
        return std::nullopt;
    }
    float GetTerrainHeight(float, float) override { return terrainHeight; }
    CleanVec3 GetTargetPosition(uint32_t) override { return targetPos; }
};

TEST_CASE("FlyingInstance - Normal Flight to Target") {
    MockCollisionWorld world;
    FlyingInstance instance(world);
    MockFlyEventHandler handler;
    instance.SetEventHandler(&handler);

    FlyingDataClean data;
    data.fInitVel = 10.0f;
    data.fRange = 1000.0f;
    data.fBombRange = 5.0f;

    FlyTargetClean target;
    target.type = FlyTargetClean::Type::OBJECT;
    target.objectId = 1;
    world.targetPos = CleanVec3(0.0f, -50.0f, 0.0f); // target is 50 units away along -Y

    instance.Create(data, CleanVec3(0.0f, 0.0f, 0.0f), target, true, 0.0f);

    // Initial velocity should be directed towards target
    // Distance is 50, speed is 10, should hit in 5 seconds
    bool alive = true;
    for (int i = 0; i < 60; ++i) { // 60 updates * 0.1s = 6 seconds
        alive = instance.Update(0.1f, i * 0.1f);
        if (!alive) break;
    }

    CHECK(alive == false);
    CHECK(handler.explodedAtTarget == true);
}

TEST_CASE("FlyingInstance - Homing Logic") {
    MockCollisionWorld world;
    FlyingInstance instance(world);

    FlyingDataClean data;
    data.fInitVel = 10.0f;
    data.fRange = 1000.0f;
    data.bIsHoming = true;
    data.fHomingMaxAngle = 90.0f;

    FlyTargetClean target;
    target.type = FlyTargetClean::Type::OBJECT;
    world.targetPos = CleanVec3(100.0f, 0.0f, 0.0f);

    instance.Create(data, CleanVec3(0.0f, 0.0f, 0.0f), target, true, 0.0f);

    // Move target while projectile is in flight
    world.targetPos = CleanVec3(100.0f, 100.0f, 0.0f);
    instance.Update(1.0f, 1.0f);

    auto pos = instance.GetPosition();
    CHECK((pos.x != 0.0f || pos.y != 0.0f)); // It moved
}

TEST_CASE("FlyingInstance - Background Collision") {
    MockCollisionWorld world;
    FlyingInstance instance(world);
    MockFlyEventHandler handler;
    instance.SetEventHandler(&handler);

    FlyingDataClean data;
    data.fInitVel = 10.0f;
    data.fRange = 100.0f;
    data.bHitOnBackground = true;

    FlyTargetClean target;
    world.targetPos = CleanVec3(100.0f, 0.0f, 0.0f);

    instance.Create(data, CleanVec3(0.0f, 0.0f, 0.0f), target, true, 0.0f);
    world.simulateBackgroundCollision = true;

    bool alive = instance.Update(1.0f, 1.0f);
    CHECK(alive == false);
    CHECK(handler.explodedAtBackground == true);
}

TEST_CASE("FlyingInstance - Out Of Range") {
    MockCollisionWorld world;
    FlyingInstance instance(world);
    MockFlyEventHandler handler;
    instance.SetEventHandler(&handler);

    FlyingDataClean data;
    data.fInitVel = 10.0f;
    data.fRange = 5.0f; // Very short range

    FlyTargetClean target;
    target.type = FlyTargetClean::Type::POSITION;
    target.position = CleanVec3(100.0f, 0.0f, 0.0f);

    instance.Create(data, CleanVec3(0.0f, 0.0f, 0.0f), target, true, 0.0f);

    bool alive = instance.Update(1.0f, 1.0f); // moves 10 units > 5 range
    CHECK(alive == false);
    CHECK(handler.explodedOutOfRange == true);
}
