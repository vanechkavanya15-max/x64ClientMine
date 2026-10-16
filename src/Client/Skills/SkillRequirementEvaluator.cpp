#include "SkillRequirementEvaluator.h"

namespace Client::Skills {

EvaluationResult<void> SpHpRequirementRule::Evaluate(const SkillEvaluationContext& ctx) const {
    if (ctx.character.currentSP < ctx.skill.requiredSP) {
        return Client::Core::MakeUnexpected(SkillEvaluationError::NotEnoughSP);
    }
    if (ctx.character.currentHP < ctx.skill.requiredHP) {
        return Client::Core::MakeUnexpected(SkillEvaluationError::NotEnoughHP);
    }
    return {};
}

EvaluationResult<void> LevelRequirementRule::Evaluate(const SkillEvaluationContext& ctx) const {
    if (ctx.character.actorLevel < ctx.skill.minLevel) {
        return Client::Core::MakeUnexpected(SkillEvaluationError::LevelTooLow);
    }
    if (!ctx.skill.isLearned) {
        return Client::Core::MakeUnexpected(SkillEvaluationError::SkillNotLearned);
    }
    return {};
}

EvaluationResult<void> CooldownRequirementRule::Evaluate(const SkillEvaluationContext& ctx) const {
    if (ctx.skill.isOnCooldown) {
        return Client::Core::MakeUnexpected(SkillEvaluationError::OnCooldown);
    }
    return {};
}

EvaluationResult<void> WeaponRequirementRule::Evaluate(const SkillEvaluationContext& ctx) const {
    if (ctx.skill.requiredWeaponSubtype.has_value()) {
        if (!ctx.character.equippedWeaponSubtype.has_value() ||
            ctx.character.equippedWeaponSubtype.value() != ctx.skill.requiredWeaponSubtype.value()) {
            return Client::Core::MakeUnexpected(SkillEvaluationError::InvalidWeapon);
        }
    }
    return {};
}

EvaluationResult<void> TargetRequirementRule::Evaluate(const SkillEvaluationContext& ctx) const {
    if (ctx.skill.requiresTarget) {
        if (!ctx.target.hasTarget || !ctx.target.isTargetValid) {
            return Client::Core::MakeUnexpected(SkillEvaluationError::InvalidTarget);
        }
        if (ctx.skill.maxRange > 0.0f && ctx.target.distanceToTarget > ctx.skill.maxRange) {
            return Client::Core::MakeUnexpected(SkillEvaluationError::OutOfRange);
        }
    }
    return {};
}

EvaluationResult<void> StateRestrictionRule::Evaluate(const SkillEvaluationContext& ctx) const {
    if (ctx.character.inSafeZone) {
        return Client::Core::MakeUnexpected(SkillEvaluationError::InSafeZone);
    }
    if (ctx.character.isPolymorphed) {
        return Client::Core::MakeUnexpected(SkillEvaluationError::CannotCastWhilePolymorphed);
    }
    if (ctx.character.isRiding && !ctx.skill.castableWhileRiding) {
        return Client::Core::MakeUnexpected(SkillEvaluationError::CannotCastWhileRiding);
    }
    return {};
}

void SkillRequirementEvaluator::AddRule(std::unique_ptr<ISkillRequirementRule> rule) {
    if (rule) {
        rules_.push_back(std::move(rule));
    }
}

EvaluationResult<void> SkillRequirementEvaluator::CanCast(const SkillEvaluationContext& ctx) const {
    for (const auto& rule : rules_) {
        auto result = rule->Evaluate(ctx);
        if (!result.has_value()) {
            return result;
        }
    }
    return {};
}

SkillRequirementEvaluator SkillRequirementEvaluator::CreateStandardEvaluator() {
    SkillRequirementEvaluator evaluator;
    evaluator.AddRule(std::make_unique<StateRestrictionRule>());
    evaluator.AddRule(std::make_unique<LevelRequirementRule>());
    evaluator.AddRule(std::make_unique<CooldownRequirementRule>());
    evaluator.AddRule(std::make_unique<WeaponRequirementRule>());
    evaluator.AddRule(std::make_unique<SpHpRequirementRule>());
    evaluator.AddRule(std::make_unique<TargetRequirementRule>());
    return evaluator;
}

} // namespace Client::Skills
