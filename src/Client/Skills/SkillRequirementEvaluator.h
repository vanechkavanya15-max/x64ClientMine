#pragma once

#include <cstdint>
#include <memory>
#include <vector>
#include <string_view>
#include <optional>
#include "Client/Core/ExpectedResult.h"
#include "Client/Core/StrongTypes.h"

namespace Client::Skills {

enum class SkillEvaluationError : uint8_t {
    None = 0,
    NotEnoughSP,
    NotEnoughHP,
    LevelTooLow,
    OnCooldown,
    InvalidWeapon,
    OutOfRange,
    InvalidTarget,
    SkillNotLearned,
    InSafeZone,
    CannotCastWhileRiding,
    CannotCastWhilePolymorphed
};

[[nodiscard]] constexpr std::string_view ToString(SkillEvaluationError err) noexcept {
    switch (err) {
        case SkillEvaluationError::None: return "None";
        case SkillEvaluationError::NotEnoughSP: return "NotEnoughSP";
        case SkillEvaluationError::NotEnoughHP: return "NotEnoughHP";
        case SkillEvaluationError::LevelTooLow: return "LevelTooLow";
        case SkillEvaluationError::OnCooldown: return "OnCooldown";
        case SkillEvaluationError::InvalidWeapon: return "InvalidWeapon";
        case SkillEvaluationError::OutOfRange: return "OutOfRange";
        case SkillEvaluationError::InvalidTarget: return "InvalidTarget";
        case SkillEvaluationError::SkillNotLearned: return "SkillNotLearned";
        case SkillEvaluationError::InSafeZone: return "InSafeZone";
        case SkillEvaluationError::CannotCastWhileRiding: return "CannotCastWhileRiding";
        case SkillEvaluationError::CannotCastWhilePolymorphed: return "CannotCastWhilePolymorphed";
    }
    return "UnknownError";
}

template <typename T = void>
using EvaluationResult = Client::Core::ExpectedResult<T, SkillEvaluationError>;

struct EvalManaPointsTag {}; using EvalManaPoints = Client::Core::StrongType<EvalManaPointsTag, uint32_t, 0>;
struct EvalHealthPointsTag {}; using EvalHealthPoints = Client::Core::StrongType<EvalHealthPointsTag, uint32_t, 0>;
struct EvalLevelTag {}; using EvalLevel = Client::Core::StrongType<EvalLevelTag, uint32_t, 1>;
struct EvalWeaponSubtypeTag {}; using EvalWeaponSubtype = Client::Core::StrongType<EvalWeaponSubtypeTag, uint32_t, 0>;

struct CharacterState {
    EvalManaPoints currentSP{0};
    EvalHealthPoints currentHP{0};
    EvalLevel actorLevel{1};
    std::optional<EvalWeaponSubtype> equippedWeaponSubtype{std::nullopt};
    bool isRiding{false};
    bool isPolymorphed{false};
    bool inSafeZone{false};
};

struct SkillData {
    bool isLearned{false};
    bool isOnCooldown{false};
    EvalManaPoints requiredSP{0};
    EvalHealthPoints requiredHP{0};
    EvalLevel minLevel{1};
    std::optional<EvalWeaponSubtype> requiredWeaponSubtype{std::nullopt};
    bool isPassive{false};
    bool requiresTarget{false};
    float maxRange{0.0f};
    bool castableWhileRiding{false};
};

struct TargetState {
    bool hasTarget{false};
    float distanceToTarget{0.0f};
    bool isTargetValid{false};
};

struct SkillEvaluationContext {
    CharacterState character;
    SkillData skill;
    TargetState target;
};

class ISkillRequirementRule {
public:
    virtual ~ISkillRequirementRule() = default;
    [[nodiscard]] virtual EvaluationResult<void> Evaluate(const SkillEvaluationContext& ctx) const = 0;
};

class SpHpRequirementRule : public ISkillRequirementRule {
public:
    [[nodiscard]] EvaluationResult<void> Evaluate(const SkillEvaluationContext& ctx) const override;
};

class LevelRequirementRule : public ISkillRequirementRule {
public:
    [[nodiscard]] EvaluationResult<void> Evaluate(const SkillEvaluationContext& ctx) const override;
};

class CooldownRequirementRule : public ISkillRequirementRule {
public:
    [[nodiscard]] EvaluationResult<void> Evaluate(const SkillEvaluationContext& ctx) const override;
};

class WeaponRequirementRule : public ISkillRequirementRule {
public:
    [[nodiscard]] EvaluationResult<void> Evaluate(const SkillEvaluationContext& ctx) const override;
};

class TargetRequirementRule : public ISkillRequirementRule {
public:
    [[nodiscard]] EvaluationResult<void> Evaluate(const SkillEvaluationContext& ctx) const override;
};

class StateRestrictionRule : public ISkillRequirementRule {
public:
    [[nodiscard]] EvaluationResult<void> Evaluate(const SkillEvaluationContext& ctx) const override;
};

class SkillRequirementEvaluator {
public:
    SkillRequirementEvaluator() = default;
    ~SkillRequirementEvaluator() = default;

    SkillRequirementEvaluator(const SkillRequirementEvaluator&) = delete;
    SkillRequirementEvaluator& operator=(const SkillRequirementEvaluator&) = delete;
    SkillRequirementEvaluator(SkillRequirementEvaluator&&) = default;
    SkillRequirementEvaluator& operator=(SkillRequirementEvaluator&&) = default;

    void AddRule(std::unique_ptr<ISkillRequirementRule> rule);

    [[nodiscard]] EvaluationResult<void> CanCast(const SkillEvaluationContext& ctx) const;

    [[nodiscard]] static SkillRequirementEvaluator CreateStandardEvaluator();

private:
    std::vector<std::unique_ptr<ISkillRequirementRule>> rules_;
};

} // namespace Client::Skills
