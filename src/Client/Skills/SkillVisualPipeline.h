#pragma once

#include <expected>
#include <optional>
#include <cstdint>
#include <vector>
#include "Client/Core/StrongTypes.h"

namespace Client::Skills {

enum class SkillVisualError {
    InvalidSourceEntity,
    TargetEntityNotFound,
    PipelineQueueFull,
    InvalidConfiguration
};

struct VisualEffectConfig {
    uint32_t effectVnum;
    std::optional<Client::Core::EntityVid> targetVid;
    uint32_t delayMs = 0;
};

struct QueuedVisualEffect {
    Client::Core::EntityVid sourceVid;
    VisualEffectConfig config;
    uint32_t queuedTimeMs;
};

class IVisualEffectRenderer {
public:
    virtual ~IVisualEffectRenderer() = default;
    virtual void DispatchEffect(Client::Core::EntityVid sourceVid, const VisualEffectConfig& config) = 0;
    virtual bool IsEntityValid(Client::Core::EntityVid vid) const = 0;
};

class SkillVisualPipeline {
public:
    explicit SkillVisualPipeline(IVisualEffectRenderer& renderer, size_t maxQueueSize = 100);

    std::expected<void, SkillVisualError> EnqueueEffect(
        Client::Core::EntityVid sourceVid, 
        const VisualEffectConfig& config, 
        uint32_t currentTimeMs);

    void Update(uint32_t currentTimeMs);
    
    size_t GetQueueSize() const noexcept { return queue_.size(); }

private:
    IVisualEffectRenderer& renderer_;
    size_t maxQueueSize_;
    std::vector<QueuedVisualEffect> queue_;
};

} // namespace Client::Skills
