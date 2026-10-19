#pragma once

#include "Client/Core/EventBus.h"
#include <string>

namespace Client::Events {

struct SkillCannotUseEvent : public EterBase::IEvent {
    uint32_t characterIndex;
    std::string reason;

    SkillCannotUseEvent(uint32_t charIdx, std::string r)
        : characterIndex(charIdx), reason(std::move(r)) {}
};

struct SkillCooldownEvent : public EterBase::IEvent {
    uint32_t skillSlotIndex;
    float cooldownTime;

    SkillCooldownEvent(uint32_t slotIdx, float cooldown)
        : skillSlotIndex(slotIdx), cooldownTime(cooldown) {}
};

struct SkillAffectEvent : public EterBase::IEvent {
    uint32_t affectId;
    bool isSet; // true = set, false = reset/clear

    SkillAffectEvent(uint32_t id, bool set)
        : affectId(id), isSet(set) {}
};

struct SkillClearAffectsEvent : public EterBase::IEvent {
    SkillClearAffectsEvent() = default;
};

struct SkillCurrentChangedEvent : public EterBase::IEvent {
    uint32_t skillSlotIndex;

    explicit SkillCurrentChangedEvent(uint32_t slotIdx)
        : skillSlotIndex(slotIdx) {}
};

struct SkillCannotShotEvent : public EterBase::IEvent {
    uint32_t characterIndex;
    std::string reason;

    SkillCannotShotEvent(uint32_t charIdx, std::string r)
        : characterIndex(charIdx), reason(std::move(r)) {}
};

} // namespace Client::Events
