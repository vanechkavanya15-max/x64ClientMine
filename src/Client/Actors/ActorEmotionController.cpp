#include "ActorEmotionController.h"
#include "Client/Events/ActorEvents.h"
#include "EterBase/EventBus.h"

namespace Client::Actors {

ActorEmotionController::ActorEmotionController(Core::EntityId ownerId)
    : ownerId_(ownerId),
      currentEmotion_(0),
      targetId_(Core::EntityId{0}),
      isPlaying_(false),
      elapsedTime_(0.0f),
      targetDuration_(0.0f) {
}

EmotionResult ActorEmotionController::StartEmotion(uint32_t emotionId, float durationSeconds) {
    if (isPlaying_) {
        return std::unexpected(EmotionError::AlreadyInEmotion);
    }
    
    if (emotionId == 0) {
        return std::unexpected(EmotionError::InvalidEmotion);
    }

    currentEmotion_ = emotionId;
    targetId_ = Core::EntityId{0};
    isPlaying_ = true;
    elapsedTime_ = 0.0f;
    targetDuration_ = durationSeconds;
    
    EterBase::EventBus::GetInstance().Publish(Events::ActorEmotionStartedEvent{ownerId_, emotionId, Core::EntityId{0}});
    
    return {};
}

EmotionResult ActorEmotionController::StartInteractionEmotion(uint32_t emotionId, Core::EntityId targetId, float durationSeconds) {
    if (isPlaying_) {
        return std::unexpected(EmotionError::AlreadyInEmotion);
    }
    
    if (emotionId == 0) {
        return std::unexpected(EmotionError::InvalidEmotion);
    }
    
    if (targetId == Core::EntityId{0}) {
        return std::unexpected(EmotionError::TargetNotFound);
    }

    currentEmotion_ = emotionId;
    targetId_ = targetId;
    isPlaying_ = true;
    elapsedTime_ = 0.0f;
    targetDuration_ = durationSeconds;
    
    EterBase::EventBus::GetInstance().Publish(Events::ActorEmotionStartedEvent{ownerId_, emotionId, targetId});
    
    return {};
}

void ActorEmotionController::Update(float deltaTimeSeconds) {
    if (isPlaying_) {
        elapsedTime_ += deltaTimeSeconds;
        if (elapsedTime_ >= targetDuration_) {
            CancelEmotion();
        }
    }
}

void ActorEmotionController::CancelEmotion() {
    if (isPlaying_) {
        isPlaying_ = false;
        EterBase::EventBus::GetInstance().Publish(Events::ActorEmotionEndedEvent{ownerId_, currentEmotion_});
        currentEmotion_ = 0;
        targetId_ = Core::EntityId{0};
        elapsedTime_ = 0.0f;
        targetDuration_ = 0.0f;
    }
}

bool ActorEmotionController::IsPlayingEmotion() const noexcept {
    return isPlaying_;
}

uint32_t ActorEmotionController::GetCurrentEmotion() const noexcept {
    return currentEmotion_;
}

Core::EntityId ActorEmotionController::GetInteractionTarget() const noexcept {
    return targetId_;
}

} // namespace Client::Actors
