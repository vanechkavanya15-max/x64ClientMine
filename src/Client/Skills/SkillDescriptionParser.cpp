#include "SkillDescriptionParser.h"
#include <format>
#include <cmath>
#include <charconv>
#include <vector>
#include <algorithm>

namespace Client::Skills {

namespace {
    bool IsSpace(char c) {
        return c == ' ' || c == '\t' || c == '\n' || c == '\r';
    }

    std::string_view Trim(std::string_view s) {
        size_t start = 0;
        while (start < s.length() && IsSpace(s[start])) start++;
        size_t end = s.length();
        while (end > start && IsSpace(s[end - 1])) end--;
        return s.substr(start, end - start);
    }
}

std::expected<std::string, SkillDescriptionParseError> SkillDescriptionParser::EvaluateExpression(
    std::string_view expr, std::string_view formatSpec, const SkillDescriptionContext& context, const SkillFormulaEvaluator& evaluator) 
{
    if (expr.empty()) return std::unexpected(SkillDescriptionParseError::EmptyExpression);

    auto evalResult = evaluator.Evaluate(expr, context.variables);
    if (!evalResult.has_value()) return std::unexpected(SkillDescriptionParseError::EvaluationFailed);

    double val = evalResult.value();

    if (formatSpec.empty()) {
        if (std::abs(val - std::round(val)) < 0.0001) {
            return std::format("{}", static_cast<long long>(std::round(val)));
        } else {
            return std::format("{}", val);
        }
    }

    if (formatSpec == "d") {
        return std::format("{}", static_cast<long long>(std::round(val)));
    } else if (formatSpec == "%") {
        return std::format("{}%", static_cast<long long>(std::round(val)));
    } else if (formatSpec == "time") {
        long long totalSeconds = static_cast<long long>(std::round(val));
        if (totalSeconds < 60) {
            return std::format("{}s", totalSeconds);
        } else if (totalSeconds < 3600) {
            return std::format("{}m {}s", totalSeconds / 60, totalSeconds % 60);
        } else {
            long long hours = totalSeconds / 3600;
            long long mins = (totalSeconds % 3600) / 60;
            long long secs = totalSeconds % 60;
            return std::format("{}h {}m {}s", hours, mins, secs);
        }
    } else if (formatSpec.starts_with(".") && formatSpec.ends_with("f")) {
        std::string_view precisionStr = formatSpec.substr(1, formatSpec.length() - 2);
        int precision = 0;
        auto [ptr, ec] = std::from_chars(precisionStr.data(), precisionStr.data() + precisionStr.length(), precision);
        if (ec == std::errc()) {
             if (precision == 0) return std::format("{:.0f}", val);
             if (precision == 1) return std::format("{:.1f}", val);
             if (precision == 2) return std::format("{:.2f}", val);
             return std::format("{}", val);
        }
        return std::unexpected(SkillDescriptionParseError::InvalidFormat);
    }

    return std::unexpected(SkillDescriptionParseError::InvalidFormat);
}

std::expected<std::string, SkillDescriptionParseError> SkillDescriptionParser::Parse(
    std::string_view templateStr, const SkillDescriptionContext& context) 
{
    std::string result;
    SkillFormulaEvaluator evaluator;
    size_t pos = 0;
    
    struct IfBlockState {
        bool conditionMet;
        bool hasElse;
        bool currentlyEvaluating;
    };
    std::vector<IfBlockState> ifStack;

    while (pos < templateStr.length()) {
        bool active = ifStack.empty() ? true : ifStack.back().currentlyEvaluating;

        size_t nextOpenBracket = templateStr.find('[', pos);
        size_t nextOpenBrace = templateStr.find('{', pos);
        size_t nextSpecial = std::string_view::npos;

        if (nextOpenBracket != std::string_view::npos && nextOpenBrace != std::string_view::npos) {
            nextSpecial = std::min(nextOpenBracket, nextOpenBrace);
        } else if (nextOpenBracket != std::string_view::npos) {
            nextSpecial = nextOpenBracket;
        } else {
            nextSpecial = nextOpenBrace;
        }

        if (nextSpecial == std::string_view::npos) {
            if (active) result += templateStr.substr(pos);
            break;
        }

        if (active) result += templateStr.substr(pos, nextSpecial - pos);
        pos = nextSpecial;

        if (templateStr[pos] == '[') {
            size_t closeBracket = templateStr.find(']', pos);
            if (closeBracket == std::string_view::npos) return std::unexpected(SkillDescriptionParseError::UnmatchedBracket);

            std::string_view block = templateStr.substr(pos + 1, closeBracket - pos - 1);
            pos = closeBracket + 1;

            if (block.starts_with("IF ")) {
                std::string_view expr = Trim(block.substr(3));
                if (expr.empty()) return std::unexpected(SkillDescriptionParseError::EmptyExpression);
                bool cond = false;
                if (active) {
                    auto evalResult = evaluator.Evaluate(expr, context.variables);
                    if (!evalResult.has_value()) return std::unexpected(SkillDescriptionParseError::EvaluationFailed);
                    cond = (evalResult.value() != 0.0);
                }
                ifStack.push_back({cond, false, active && cond});
            } else if (block == "ELSE") {
                if (ifStack.empty()) return std::unexpected(SkillDescriptionParseError::UnmatchedCondition);
                auto& state = ifStack.back();
                state.hasElse = true;
                bool parentActive = ifStack.size() > 1 ? ifStack[ifStack.size()-2].currentlyEvaluating : true;
                state.currentlyEvaluating = (parentActive && !state.conditionMet);
                state.conditionMet = true;
            } else if (block == "ENDIF") {
                if (ifStack.empty()) return std::unexpected(SkillDescriptionParseError::UnmatchedCondition);
                ifStack.pop_back();
            } else {
                 if (active) {
                     result += "["; result += block; result += "]";
                 }
            }
        } else if (templateStr[pos] == '{') {
            size_t closeBrace = templateStr.find('}', pos);
            if (closeBrace == std::string_view::npos) return std::unexpected(SkillDescriptionParseError::UnmatchedBracket);

            std::string_view content = templateStr.substr(pos + 1, closeBrace - pos - 1);
            pos = closeBrace + 1;

            if (active) {
                std::string_view expr = content;
                std::string_view formatSpec = "";
                size_t colonPos = content.find(':');
                if (colonPos != std::string_view::npos) {
                    expr = Trim(content.substr(0, colonPos));
                    formatSpec = Trim(content.substr(colonPos + 1));
                } else {
                    expr = Trim(expr);
                }

                auto evalResult = EvaluateExpression(expr, formatSpec, context, evaluator);
                if (!evalResult.has_value()) return std::unexpected(evalResult.error());
                result += evalResult.value();
            }
        }
    }

    if (!ifStack.empty()) return std::unexpected(SkillDescriptionParseError::UnmatchedCondition);
    return result;
}

std::string SkillDescriptionParser::FormatFormula(const std::string& rawDesc, float skillLevel, float playerInt) {
    SkillDescriptionContext ctx;
    ctx.variables["VAL"] = static_cast<double>(skillLevel * 10.0f + playerInt * 1.5f);
    auto res = Parse(rawDesc, ctx);
    return res.has_value() ? res.value() : rawDesc;
}

} // namespace Client::Skills
