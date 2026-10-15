#define DOCTEST_CONFIG_IMPLEMENT_WITH_MAIN
#include "doctest.h"

#include "Client/Skills/SkillVisualPipeline.h"
#include "Client/Core/StrongTypes.h"
#include "Client/Core/DomainErrors.h"
#include "Client/Core/Result.h"

using namespace Client::Skills;
using namespace Client::Core;

class MockSkillVisualEffectSystem : public ISkillVisualEffectSystem {
public:
    Result<void, SkillError> PlayEffect(EntityVid target, SkillIndex skillId, const std::optional<MapCoords>& coords) override {
        if (playShouldFail) {
            return std::unexpected(SkillError::RequirementNotMet);
        }
        playCallCount++;
        lastPlayedTarget = target;
        lastPlayedSkill = skillId;
        lastPlayedCoords = coords;
        return {};
    }

    Result<void, SkillError> StopEffect(EntityVid target, SkillIndex skillId) override {
        if (stopShouldFail) {
            return std::unexpected(SkillError::SkillNotFound);
        }
        stopCallCount++;
        lastStoppedTarget = target;
        lastStoppedSkill = skillId;
        return {};
    }

    bool playShouldFail = false;
    bool stopShouldFail = false;
    
    int playCallCount = 0;
    int stopCallCount = 0;
    
    EntityVid lastPlayedTarget{0};
    SkillIndex lastPlayedSkill{0};
    std::optional<MapCoords> lastPlayedCoords = std::nullopt;

    EntityVid lastStoppedTarget{0};
    SkillIndex lastStoppedSkill{0};
};

TEST_CASE("SkillVisualPipeline - Triggering effects") {
    MockSkillVisualEffectSystem mockSystem;
    SkillVisualPipeline pipeline(mockSystem);

    SUBCASE("Successfully triggers an effect") {
        EntityVid target(123);
        SkillIndex skillId(1);
        float duration = 5.0f;

        auto result = pipeline.TriggerSkillEffect(target, skillId, duration);

        CHECK(result.has_value());
        CHECK(pipeline.GetActiveEffectsCount() == 1);
        CHECK(mockSystem.playCallCount == 1);
        CHECK(mockSystem.lastPlayedTarget == target);
        CHECK(mockSystem.lastPlayedSkill == skillId);
        CHECK_FALSE(mockSystem.lastPlayedCoords.has_value());
    }

    SUBCASE("Fails when target is invalid (VID = 0)") {
        EntityVid target(0);
        SkillIndex skillId(1);
        float duration = 5.0f;

        auto result = pipeline.TriggerSkillEffect(target, skillId, duration);

        CHECK_FALSE(result.has_value());
        CHECK(result.error() == SkillError::RequirementNotMet);
        CHECK(pipeline.GetActiveEffectsCount() == 0);
        CHECK(mockSystem.playCallCount == 0);
    }

    SUBCASE("Fails when duration is 0 or negative") {
        EntityVid target(123);
        SkillIndex skillId(1);
        float duration = 0.0f;

        auto result = pipeline.TriggerSkillEffect(target, skillId, duration);

        CHECK_FALSE(result.has_value());
        CHECK(result.error() == SkillError::RequirementNotMet);
        CHECK(pipeline.GetActiveEffectsCount() == 0);
        CHECK(mockSystem.playCallCount == 0);
    }

    SUBCASE("Fails when underlying system rejects the effect") {
        EntityVid target(123);
        SkillIndex skillId(1);
        float duration = 5.0f;
        
        mockSystem.playShouldFail = true;

        auto result = pipeline.TriggerSkillEffect(target, skillId, duration);

        CHECK_FALSE(result.has_value());
        CHECK(result.error() == SkillError::RequirementNotMet);
        CHECK(pipeline.GetActiveEffectsCount() == 0);
        CHECK(mockSystem.playCallCount == 0);
    }
    
    SUBCASE("Triggers an effect with coordinates") {
        EntityVid target(123);
        SkillIndex skillId(1);
        float duration = 5.0f;
        MapCoords coords{10.0f, 20.0f, 30.0f};

        auto result = pipeline.TriggerSkillEffect(target, skillId, duration, coords);

        CHECK(result.has_value());
        CHECK(pipeline.GetActiveEffectsCount() == 1);
        CHECK(mockSystem.playCallCount == 1);
        CHECK(mockSystem.lastPlayedCoords.has_value());
        CHECK(mockSystem.lastPlayedCoords->x == coords.x);
        CHECK(mockSystem.lastPlayedCoords->y == coords.y);
        CHECK(mockSystem.lastPlayedCoords->z == coords.z);
    }
}

TEST_CASE("SkillVisualPipeline - Updating and canceling effects") {
    MockSkillVisualEffectSystem mockSystem;
    SkillVisualPipeline pipeline(mockSystem);
    
    EntityVid target(123);
    SkillIndex skillId(1);
    float duration = 5.0f;

    pipeline.TriggerSkillEffect(target, skillId, duration);

    SUBCASE("Update decrements duration but does not remove active effects") {
        pipeline.Update(2.0f);

        CHECK(pipeline.GetActiveEffectsCount() == 1);
        CHECK(mockSystem.stopCallCount == 0);
    }

    SUBCASE("Update removes expired effects and calls StopEffect") {
        pipeline.Update(5.1f);

        CHECK(pipeline.GetActiveEffectsCount() == 0);
        CHECK(mockSystem.stopCallCount == 1);
        CHECK(mockSystem.lastStoppedTarget == target);
        CHECK(mockSystem.lastStoppedSkill == skillId);
    }

    SUBCASE("CancelEffect removes active effect and calls StopEffect") {
        auto result = pipeline.CancelEffect(target, skillId);

        CHECK(result.has_value());
        CHECK(pipeline.GetActiveEffectsCount() == 0);
        CHECK(mockSystem.stopCallCount == 1);
        CHECK(mockSystem.lastStoppedTarget == target);
        CHECK(mockSystem.lastStoppedSkill == skillId);
    }

    SUBCASE("CancelEffect fails if effect does not exist") {
        EntityVid otherTarget(456);
        auto result = pipeline.CancelEffect(otherTarget, skillId);

        CHECK_FALSE(result.has_value());
        CHECK(result.error() == SkillError::SkillNotFound);
        CHECK(pipeline.GetActiveEffectsCount() == 1);
        CHECK(mockSystem.stopCallCount == 0);
    }
    
    SUBCASE("CancelEffect fails if underlying system rejects stop") {
        mockSystem.stopShouldFail = true;
        
        auto result = pipeline.CancelEffect(target, skillId);
        
        CHECK_FALSE(result.has_value());
        CHECK(result.error() == SkillError::SkillNotFound);
        CHECK(pipeline.GetActiveEffectsCount() == 1);
    }
}
