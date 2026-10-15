#include "SkillVisualPipeline.h"
#include <algorithm>

namespace Client::Skills {

SkillVisualPipeline::SkillVisualPipeline(IVisualEffectRenderer& renderer, size_t maxQueueSize)
    : renderer_(renderer), maxQueueSize_(maxQueueSize) {
}

std::expected<void, SkillVisualError> SkillVisualPipeline::EnqueueEffect(
    Client::Core::EntityVid sourceVid, 
    const VisualEffectConfig& config, 
    uint32_t currentTimeMs) {
    
    if (config.effectVnum == 0) {
        return std::unexpected(SkillVisualError::InvalidConfiguration);
    }

    if (!renderer_.IsEntityValid(sourceVid)) {
        return std::unexpected(SkillVisualError::InvalidSourceEntity);
    }

    if (config.targetVid.has_value() && !renderer_.IsEntityValid(config.targetVid.value())) {
        return std::unexpected(SkillVisualError::TargetEntityNotFound);
    }

    if (queue_.size() >= maxQueueSize_) {
        return std::unexpected(SkillVisualError::PipelineQueueFull);
    }

    queue_.push_back({sourceVid, config, currentTimeMs});
    return {};
}

void SkillVisualPipeline::Update(uint32_t currentTimeMs) {
    auto it = queue_.begin();
    while (it != queue_.end()) {
        uint32_t elapsedTime = currentTimeMs - it->queuedTimeMs;
        
        if (elapsedTime >= it->config.delayMs) {
            // Check if source is still valid
            if (renderer_.IsEntityValid(it->sourceVid)) {
                // If there's a target, check if it's still valid
                bool dispatch = true;
                if (it->config.targetVid.has_value() && !renderer_.IsEntityValid(it->config.targetVid.value())) {
                    dispatch = false;
                }
                
                if (dispatch) {
                    renderer_.DispatchEffect(it->sourceVid, it->config);
                }
            }
            
            it = queue_.erase(it);
        } else {
            ++it;
        }
    }
}

} // namespace Client::Skills
