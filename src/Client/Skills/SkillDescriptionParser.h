#pragma once
#include <string>
#include <string_view>
#include <expected>
#include <unordered_map>
#include "Client/Skills/SkillFormulaEvaluator.h"

namespace Client::Skills {

enum class SkillDescriptionParseError {
    UnmatchedBracket,
    UnmatchedCondition,
    InvalidFormat,
    EvaluationFailed,
    InvalidCondition,
    UnknownVariable,
    EmptyExpression,
    InvalidTimeFormat
};

struct SkillDescriptionContext {
    SkillFormulaEvaluator::Context variables;
    float skillLevel = 0.0f;
    float playerInt = 0.0f;
};

class SkillDescriptionParser {
public:
    SkillDescriptionParser() = default;
    ~SkillDescriptionParser() = default;

    [[nodiscard]] static std::expected<std::string, SkillDescriptionParseError> Parse(std::string_view templateStr, const SkillDescriptionContext& context);
    [[nodiscard]] static std::string FormatFormula(const std::string& rawDesc, float skillLevel, float playerInt);

private:
    [[nodiscard]] static std::expected<std::string, SkillDescriptionParseError> EvaluateExpression(std::string_view expr, std::string_view formatSpec, const SkillDescriptionContext& context, const SkillFormulaEvaluator& evaluator);
};

} // namespace Client::Skills
