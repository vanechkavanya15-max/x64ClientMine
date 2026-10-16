#pragma once

#include <cstdint>
#include <expected>
#include <string_view>
#include <format>
#include "Client/Core/StrongTypes.h"

namespace Client::Actors {

enum class EmotionError : uint8_t {
    None = 0,
    InvalidEmotion,
    TargetNotFound,
    AlreadyInEmotion,
    EmotionNotAllowed
};

[[nodiscard]] inline std::string_view to_string(EmotionError error) noexcept {
    switch (error) {
        case EmotionError::None: return "EmotionError::None";
        case EmotionError::InvalidEmotion: return "EmotionError::InvalidEmotion - Emotion ID is invalid or out of range";
        case EmotionError::TargetNotFound: return "EmotionError::TargetNotFound - Interaction target could not be found";
        case EmotionError::AlreadyInEmotion: return "EmotionError::AlreadyInEmotion - Actor is already performing an emotion";
        case EmotionError::EmotionNotAllowed: return "EmotionError::EmotionNotAllowed - Emotion cannot be performed in current state";
        default: return "EmotionError::Unknown";
    }
}

using EmotionResult = std::expected<void, EmotionError>;

class ActorEmotionController {
public:
    ActorEmotionController(Core::EntityId ownerId);
    ~ActorEmotionController() = default;

    EmotionResult StartEmotion(uint32_t emotionId, float durationSeconds);
    EmotionResult StartInteractionEmotion(uint32_t emotionId, Core::EntityId targetId, float durationSeconds);
    
    void Update(float deltaTimeSeconds);
    void CancelEmotion();
    
    [[nodiscard]] bool IsPlayingEmotion() const noexcept;
    [[nodiscard]] uint32_t GetCurrentEmotion() const noexcept;
    [[nodiscard]] Core::EntityId GetInteractionTarget() const noexcept;

private:
    Core::EntityId ownerId_;
    uint32_t currentEmotion_;
    Core::EntityId targetId_;
    bool isPlaying_;
    float elapsedTime_;
    float targetDuration_;
};

} // namespace Client::Actors

template <>
struct std::formatter<Client::Actors::EmotionError> : std::formatter<std::string_view> {
    auto format(Client::Actors::EmotionError err, std::format_context& ctx) const {
        return std::formatter<std::string_view>::format(Client::Actors::to_string(err), ctx);
    }
};

