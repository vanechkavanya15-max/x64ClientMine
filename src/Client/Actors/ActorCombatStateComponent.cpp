#include "ActorCombatStateComponent.h"
#include <algorithm>

namespace Client::Actors {

uint32_t ActorCombatStateComponent::GetHp() const noexcept {
    return hp_;
}

uint32_t ActorCombatStateComponent::GetMaxHp() const noexcept {
    return maxHp_;
}

void ActorCombatStateComponent::SetHp(uint32_t hp) noexcept {
    hp_ = std::min(hp, maxHp_);
    if (hp_ == 0 && maxHp_ > 0) {
        isDead_ = true;
    } else if (hp_ > 0) {
        isDead_ = false;
    }
}

void ActorCombatStateComponent::SetMaxHp(uint32_t maxHp) noexcept {
    maxHp_ = maxHp;
    if (hp_ > maxHp_) {
        hp_ = maxHp_;
    }
}

uint32_t ActorCombatStateComponent::GetSp() const noexcept {
    return sp_;
}

uint32_t ActorCombatStateComponent::GetMaxSp() const noexcept {
    return maxSp_;
}

void ActorCombatStateComponent::SetSp(uint32_t sp) noexcept {
    sp_ = std::min(sp, maxSp_);
}

void ActorCombatStateComponent::SetMaxSp(uint32_t maxSp) noexcept {
    maxSp_ = maxSp;
    if (sp_ > maxSp_) {
        sp_ = maxSp_;
    }
}

std::optional<Client::Core::EntityVid> ActorCombatStateComponent::GetTargetVid() const noexcept {
    return targetVid_;
}

void ActorCombatStateComponent::SetTargetVid(Client::Core::EntityVid vid) noexcept {
    targetVid_ = vid;
}

void ActorCombatStateComponent::ClearTargetVid() noexcept {
    targetVid_ = std::nullopt;
}

bool ActorCombatStateComponent::IsDead() const noexcept {
    return isDead_;
}

void ActorCombatStateComponent::SetDead(bool dead) noexcept {
    isDead_ = dead;
    if (isDead_) {
        hp_ = 0;
    }
}

Client::Core::Result<void, Client::Core::ActorError> ActorCombatStateComponent::TakeDamage(uint32_t damage) noexcept {
    if (isDead_) {
        return std::unexpected(Client::Core::ActorError::AlreadyDead);
    }

    if (damage >= hp_) {
        hp_ = 0;
        isDead_ = true;
    } else {
        hp_ -= damage;
    }

    return {};
}

} // namespace Client::Actors
