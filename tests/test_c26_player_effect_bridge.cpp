#include "doctest/doctest.h"
#include "Client/Player/PlayerEffectRegistryBridge.h"

using namespace Client::Player;
using namespace Client::Core;

TEST_CASE("PlayerEffectRegistryBridge: Effect registration") {
    PlayerEffectRegistryBridge reg;
    EffectId validId{1};
    EffectId invalidId{0};
    
    SUBCASE("Successful registration") {
        auto result = reg.RegisterEffect(validId, "effect/level_up.mse", false, 2.0f, 1.5f);
        CHECK(result.has_value());
        
        auto tmpl = reg.GetEffectTemplate(validId);
        REQUIRE(tmpl.has_value());
        CHECK(tmpl->id == validId);
        CHECK(tmpl->filePath == "effect/level_up.mse");
        CHECK(tmpl->isLooping == false);
        CHECK(tmpl->durationSeconds == 2.0f);
        CHECK(tmpl->defaultScale == 1.5f);
    }
    
    SUBCASE("Prevent duplicate registration") {
        CHECK(reg.RegisterEffect(validId, "effect/level_up.mse").has_value());
        auto result = reg.RegisterEffect(validId, "effect/level_up.mse");
        REQUIRE(!result.has_value());
        CHECK(result.error() == EffectError::AlreadyRegistered);
    }
    
    SUBCASE("Prevent invalid ID") {
        auto result = reg.RegisterEffect(invalidId, "effect/level_up.mse");
        REQUIRE(!result.has_value());
        CHECK(result.error() == EffectError::InvalidId);
    }
}

TEST_CASE("PlayerEffectRegistryBridge: Effect playback and lifecycle") {
    PlayerEffectRegistryBridge reg;
    EffectId effectId{1};
    EntityVid targetVid{100};
    EntityVid invalidVid{0};
    
    REQUIRE(reg.RegisterEffect(effectId, "effect/test.mse", false, 1.0f).has_value());
    
    SUBCASE("Show effect successfully") {
        auto result = reg.ShowEffect(targetVid, effectId, EffectAttachmentPoint::Head, 2.0f);
        REQUIRE(result.has_value());
        
        auto instanceId = result.value();
        auto instance = reg.GetEffectInstance(instanceId);
        REQUIRE(instance.has_value());
        CHECK(instance->effectId == effectId);
        CHECK(instance->targetVid == targetVid);
        CHECK(instance->attachmentPoint == EffectAttachmentPoint::Head);
        CHECK(instance->state == EffectState::Playing);
        CHECK(instance->scale == 2.0f);
        
        auto activeEffects = reg.GetActiveEffects();
        CHECK(activeEffects.size() == 1);
        
        auto targetEffects = reg.GetEffectsOnTarget(targetVid);
        CHECK(targetEffects.size() == 1);
    }
    
    SUBCASE("Show effect invalid target") {
        auto result = reg.ShowEffect(invalidVid, effectId);
        REQUIRE(!result.has_value());
        CHECK(result.error() == EffectError::InvalidTarget);
    }
    
    SUBCASE("Stop specific effect") {
        auto instanceId = reg.ShowEffect(targetVid, effectId).value();
        
        auto stopResult = reg.StopEffect(instanceId);
        REQUIRE(stopResult.has_value());
        
        auto instance = reg.GetEffectInstance(instanceId);
        REQUIRE(instance.has_value());
        CHECK(instance->state == EffectState::Stopped);
        
        auto activeEffects = reg.GetActiveEffects();
        CHECK(activeEffects.empty());
    }
    
    SUBCASE("Stop all effects on target") {
        reg.ShowEffect(targetVid, effectId);
        reg.ShowEffect(targetVid, effectId);
        
        CHECK(reg.GetActiveEffects().size() == 2);
        
        auto stopResult = reg.StopAllEffectsOnTarget(targetVid);
        REQUIRE(stopResult.has_value());
        
        CHECK(reg.GetActiveEffects().empty());
    }
}

TEST_CASE("PlayerEffectRegistryBridge: Update system and time progression") {
    PlayerEffectRegistryBridge reg;
    EffectId nonLoopingId{1};
    EffectId loopingId{2};
    EntityVid targetVid{100};
    
    REQUIRE(reg.RegisterEffect(nonLoopingId, "effect/non_looping.mse", false, 1.0f).has_value());
    REQUIRE(reg.RegisterEffect(loopingId, "effect/looping.mse", true, 1.0f).has_value());
    
    auto instNonLooping = reg.ShowEffect(targetVid, nonLoopingId).value();
    auto instLooping = reg.ShowEffect(targetVid, loopingId).value();
    
    SUBCASE("Update advances playback time but does not stop before duration") {
        reg.UpdateEffects(0.5f);
        
        auto nl = reg.GetEffectInstance(instNonLooping).value();
        CHECK(nl.playbackTime == 0.5f);
        CHECK(nl.state == EffectState::Playing);
    }
    
    SUBCASE("Update stops non-looping effect when duration exceeded and cleans up") {
        reg.UpdateEffects(1.5f); // Duration is 1.0f
        
        // The first update marks it as Stopped and then it gets cleaned up on the NEXT update if it was already stopped
        // Wait, the logic in std::erase_if is:
        // if stopped -> return true (erase)
        // if playing -> advance time -> if > duration -> state=Stopped -> return true (erase)
        // So it should be completely erased!
        
        auto nl = reg.GetEffectInstance(instNonLooping);
        CHECK(!nl.has_value());
        CHECK(nl.error() == EffectError::InstanceNotFound);
        
        // Looping effect should still be playing and NOT erased
        auto l = reg.GetEffectInstance(instLooping);
        REQUIRE(l.has_value());
        CHECK(l->state == EffectState::Playing);
        CHECK(l->playbackTime == 1.5f);
    }
}