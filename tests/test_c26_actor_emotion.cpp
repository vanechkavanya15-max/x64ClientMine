#include "Client/Actors/ActorEmotionController.h"
#include "Client/Events/ActorEvents.h"
#include "EterBase/EventBus.h"
#include <doctest.h>

using namespace Client::Actors;
using namespace Client::Core;
using namespace Client::Events;

TEST_SUITE("ActorEmotionController") {

    TEST_CASE("StartEmotion - Valid Emotion - Emits Event") {
        auto& eventBus = EterBase::EventBus::GetInstance();
        bool eventFired = false;
        EntityId receivedActorId{0};
        uint32_t receivedEmotionId = 0;
        
        auto subId = eventBus.Subscribe<ActorEmotionStartedEvent>([&](const ActorEmotionStartedEvent& e) {
            eventFired = true;
            receivedActorId = e.actorId;
            receivedEmotionId = e.emotionId;
        });

        EntityId ownerId{123};
        ActorEmotionController controller(ownerId);
        
        auto result = controller.StartEmotion(1, 2.0f); // 1 = dance
        
        CHECK(result.has_value());
        CHECK(controller.IsPlayingEmotion());
        CHECK(controller.GetCurrentEmotion() == 1);
        CHECK(controller.GetInteractionTarget() == EntityId{0});
        
        CHECK(eventFired);
        CHECK(receivedActorId == ownerId);
        CHECK(receivedEmotionId == 1);
        
        eventBus.Unsubscribe<ActorEmotionStartedEvent>(subId);
    }
    
    TEST_CASE("StartEmotion - Invalid Emotion - Returns Error") {
        EntityId ownerId{123};
        ActorEmotionController controller(ownerId);
        
        auto result = controller.StartEmotion(0, 1.0f); // 0 is invalid
        
        CHECK(!result.has_value());
        CHECK(result.error() == EmotionError::InvalidEmotion);
        CHECK(!controller.IsPlayingEmotion());
    }
    
    TEST_CASE("StartInteractionEmotion - Valid Target - Emits Event") {
        auto& eventBus = EterBase::EventBus::GetInstance();
        bool eventFired = false;
        EntityId receivedTargetId{0};
        
        auto subId = eventBus.Subscribe<ActorEmotionStartedEvent>([&](const ActorEmotionStartedEvent& e) {
            eventFired = true;
            receivedTargetId = e.targetId;
        });

        EntityId ownerId{123};
        EntityId targetId{456};
        ActorEmotionController controller(ownerId);
        
        auto result = controller.StartInteractionEmotion(2, targetId, 2.0f); // 2 = kiss
        
        CHECK(result.has_value());
        CHECK(controller.IsPlayingEmotion());
        CHECK(controller.GetCurrentEmotion() == 2);
        CHECK(controller.GetInteractionTarget() == targetId);
        
        CHECK(eventFired);
        CHECK(receivedTargetId == targetId);
        
        eventBus.Unsubscribe<ActorEmotionStartedEvent>(subId);
    }
    
    TEST_CASE("StartInteractionEmotion - Invalid Target - Returns Error") {
        EntityId ownerId{123};
        ActorEmotionController controller(ownerId);
        
        auto result = controller.StartInteractionEmotion(2, EntityId{0}, 2.0f);
        
        CHECK(!result.has_value());
        CHECK(result.error() == EmotionError::TargetNotFound);
        CHECK(!controller.IsPlayingEmotion());
    }

    TEST_CASE("StartEmotion - Already Playing - Returns Error") {
        EntityId ownerId{123};
        ActorEmotionController controller(ownerId);
        
        controller.StartEmotion(1, 1.0f);
        
        auto result = controller.StartEmotion(2, 1.0f);
        
        CHECK(!result.has_value());
        CHECK(result.error() == EmotionError::AlreadyInEmotion);
        CHECK(controller.GetCurrentEmotion() == 1); // Remains 1
    }

    TEST_CASE("CancelEmotion - Playing Emotion - Emits Ended Event") {
        auto& eventBus = EterBase::EventBus::GetInstance();
        bool eventFired = false;
        EntityId receivedActorId{0};
        uint32_t receivedEmotionId = 0;
        
        auto subId = eventBus.Subscribe<ActorEmotionEndedEvent>([&](const ActorEmotionEndedEvent& e) {
            eventFired = true;
            receivedActorId = e.actorId;
            receivedEmotionId = e.emotionId;
        });

        EntityId ownerId{123};
        ActorEmotionController controller(ownerId);
        
        controller.StartEmotion(1, 1.0f);
        controller.CancelEmotion();
        
        CHECK(!controller.IsPlayingEmotion());
        CHECK(controller.GetCurrentEmotion() == 0);
        
        CHECK(eventFired);
        CHECK(receivedActorId == ownerId);
        CHECK(receivedEmotionId == 1);
        
        eventBus.Unsubscribe<ActorEmotionEndedEvent>(subId);
    }

    TEST_CASE("Update - Playing Emotion - Completes After Duration") {
        auto& eventBus = EterBase::EventBus::GetInstance();
        bool endedFired = false;
        auto subId = eventBus.Subscribe<ActorEmotionEndedEvent>([&](const ActorEmotionEndedEvent& e) {
            endedFired = true;
        });
        
        EntityId ownerId{123};
        ActorEmotionController controller(ownerId);
        
        controller.StartEmotion(1, 2.0f);
        CHECK(controller.IsPlayingEmotion());
        
        controller.Update(1.0f);
        CHECK(controller.IsPlayingEmotion());
        CHECK(!endedFired);
        
        controller.Update(1.0f);
        CHECK(!controller.IsPlayingEmotion());
        CHECK(controller.GetCurrentEmotion() == 0);
        CHECK(endedFired);
        
        eventBus.Unsubscribe<ActorEmotionEndedEvent>(subId);
    }
}
