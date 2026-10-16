#pragma once

#include "EterBase/EventBus.h"
#include "Client/Core/StrongTypes.h"
#include <cstdint>

namespace Client::Events {

struct ActorEmotionStartedEvent : public EterBase::IEvent {
    Core::EntityId actorId;
    uint32_t emotionId;
    Core::EntityId targetId; // 0 if no target
    
    ActorEmotionStartedEvent(Core::EntityId actorId, uint32_t emotionId, Core::EntityId targetId = Core::EntityId{0})
        : actorId(actorId), emotionId(emotionId), targetId(targetId) {}
};

struct ActorEmotionEndedEvent : public EterBase::IEvent {
    Core::EntityId actorId;
    uint32_t emotionId;
    
    ActorEmotionEndedEvent(Core::EntityId actorId, uint32_t emotionId)
        : actorId(actorId), emotionId(emotionId) {}
};

} // namespace Client::Events
