#define DOCTEST_CONFIG_IMPLEMENT_WITH_MAIN
#include "doctest.h"
#include "../src/Client/Actors/ActorSoundComponent.h"
#include "../src/EterBase/EventBus.h"

using namespace Client::Actors;

TEST_CASE("ActorSoundComponent Initialization") {
    ActorSoundComponent soundComponent;
    EterBase::EventBus& eventBus = EterBase::EventBus::Instance();
    EterBase::EntityId entityId{42};
    
    SUBCASE("Initialization fails without EventBus") {
        auto result = soundComponent.Initialize(entityId, nullptr);
        CHECK_FALSE(result.has_value());
        CHECK(result.error() == SoundError::NotInitialized);
    }
    
    SUBCASE("Initialization succeeds with EventBus") {
        auto result = soundComponent.Initialize(entityId, &eventBus);
        CHECK(result.has_value());
    }
}

TEST_CASE("ActorSoundComponent PlayFootstepSound") {
    ActorSoundComponent soundComponent;
    EterBase::EventBus& eventBus = EterBase::EventBus::Instance();
    EterBase::EntityId entityId{42};
    soundComponent.Initialize(entityId, &eventBus);
    
    bool eventReceived = false;
    auto subId = eventBus.Subscribe<ActorFootstepSoundEvent>([&](const ActorFootstepSoundEvent& event) {
        eventReceived = true;
        CHECK(event.entityId == entityId);
        CHECK(event.surface == SurfaceType::Dirt);
        CHECK(event.soundPath == "sound/pc/footstep/dirt.wav");
    });
    
    auto result = soundComponent.PlayFootstepSound(SurfaceType::Dirt);
    CHECK(result.has_value());
    CHECK(eventReceived);
    eventBus.Unsubscribe<ActorFootstepSoundEvent>(subId);
}

TEST_CASE("ActorSoundComponent PlayWeaponSound") {
    ActorSoundComponent soundComponent;
    EterBase::EventBus& eventBus = EterBase::EventBus::Instance();
    EterBase::EntityId entityId{42};
    soundComponent.Initialize(entityId, &eventBus);
    
    bool eventReceived = false;
    auto subId = eventBus.Subscribe<ActorWeaponSoundEvent>([&](const ActorWeaponSoundEvent& event) {
        eventReceived = true;
        CHECK(event.entityId == entityId);
        CHECK(event.weapon == WeaponType::Sword);
        CHECK(event.soundPath == "sound/pc/warrior/sword.wav");
    });
    
    auto result = soundComponent.PlayWeaponSound(WeaponType::Sword);
    CHECK(result.has_value());
    CHECK(eventReceived);
    eventBus.Unsubscribe<ActorWeaponSoundEvent>(subId);
}

TEST_CASE("ActorSoundComponent PlayVoiceSound") {
    ActorSoundComponent soundComponent;
    EterBase::EventBus& eventBus = EterBase::EventBus::Instance();
    EterBase::EntityId entityId{42};
    soundComponent.Initialize(entityId, &eventBus);
    
    bool eventReceived = false;
    auto subId = eventBus.Subscribe<ActorVoiceSoundEvent>([&](const ActorVoiceSoundEvent& event) {
        eventReceived = true;
        CHECK(event.entityId == entityId);
        CHECK(event.soundPath == "sound/pc/voice/attack.wav");
    });
    
    auto result = soundComponent.PlayVoiceSound("attack");
    CHECK(result.has_value());
    CHECK(eventReceived);
    eventBus.Unsubscribe<ActorVoiceSoundEvent>(subId);
}

TEST_CASE("ActorSoundComponent uninitialized state") {
    ActorSoundComponent soundComponent;
    
    SUBCASE("PlayFootstepSound fails when uninitialized") {
        auto result = soundComponent.PlayFootstepSound(SurfaceType::Dirt);
        CHECK_FALSE(result.has_value());
        CHECK(result.error() == SoundError::NotInitialized);
    }
    
    SUBCASE("PlayWeaponSound fails when uninitialized") {
        auto result = soundComponent.PlayWeaponSound(WeaponType::Sword);
        CHECK_FALSE(result.has_value());
        CHECK(result.error() == SoundError::NotInitialized);
    }
    
    SUBCASE("PlayVoiceSound fails when uninitialized") {
        auto result = soundComponent.PlayVoiceSound("attack");
        CHECK_FALSE(result.has_value());
        CHECK(result.error() == SoundError::NotInitialized);
    }
}
