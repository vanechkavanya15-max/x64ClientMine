#include "ActorEffectComponent.h"
#include <algorithm>

namespace Client::Actors {

EffectResult<EffectHandle> ActorEffectComponent::AttachEffect(EffectId effectId) {
    if (!effectId) {
        return std::unexpected(EffectError::InvalidEffect);
    }

    if (m_activeEffects.size() >= MAX_ACTIVE_EFFECTS) {
        return std::unexpected(EffectError::LimitReached);
    }

    EffectHandle handle{m_nextHandleId++};
    if (m_nextHandleId == 0) {
        // Prevent handle overflow wrapping to 0, which might be seen as invalid if we want non-zero handles.
        m_nextHandleId = 1;
    }

    m_activeEffects.push_back({effectId, handle});
    return handle;
}

EffectResult<void> ActorEffectComponent::DetachEffect(EffectHandle handle) {
    auto it = std::find_if(m_activeEffects.begin(), m_activeEffects.end(),
        [handle](const ActiveEffect& effect) {
            return effect.handle == handle;
        });

    if (it == m_activeEffects.end()) {
        return std::unexpected(EffectError::EffectNotFound);
    }

    m_activeEffects.erase(it);
    return {};
}

EffectResult<void> ActorEffectComponent::DetachAllEffects() {
    m_activeEffects.clear();
    return {};
}

size_t ActorEffectComponent::GetActiveEffectCount() const noexcept {
    return m_activeEffects.size();
}

bool ActorEffectComponent::HasEffect(EffectHandle handle) const noexcept {
    return std::any_of(m_activeEffects.begin(), m_activeEffects.end(),
        [handle](const ActiveEffect& effect) {
            return effect.handle == handle;
        });
}

std::vector<EffectHandle> ActorEffectComponent::GetActiveEffects() const {
    std::vector<EffectHandle> handles;
    handles.reserve(m_activeEffects.size());
    for (const auto& effect : m_activeEffects) {
        handles.push_back(effect.handle);
    }
    return handles;
}

} // namespace Client::Actors
