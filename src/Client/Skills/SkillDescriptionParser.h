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

    [[nodiscard]] static std::expected<double, EvaluationError> EvaluateFormula(std::string_view formula, const SkillFormulaEvaluator::Context& context);
    [[nodiscard]] static std::string FormatDescription(std::string_view descTemplate, float minValue, float maxValue, const SkillDescriptionContext* context = nullptr);

    static bool ReplaceNextFormatSpecifier(std::string& s, float value);
    static void UnescapePercent(std::string& s);

private:
    [[nodiscard]] static std::expected<std::string, SkillDescriptionParseError> EvaluateExpression(std::string_view expr, std::string_view formatSpec, const SkillDescriptionContext& context, const SkillFormulaEvaluator& evaluator);
};

} // namespace Client::Skills
