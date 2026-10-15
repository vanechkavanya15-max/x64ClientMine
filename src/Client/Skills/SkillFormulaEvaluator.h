#pragma once

#include <string>
#include <string_view>
#include <unordered_map>
#include <expected>
#include <vector>

namespace Client::Skills {

enum class EvaluationError {
    InvalidSyntax,
    UnknownVariable,
    DivisionByZero,
    MismatchedParentheses,
    UnknownFunction,
    EmptyFormula
};

class SkillFormulaEvaluator {
public:
    using Context = std::unordered_map<std::string, double>;

    SkillFormulaEvaluator() = default;

    [[nodiscard]] std::expected<double, EvaluationError> Evaluate(std::string_view formula, const Context& context) const;

    struct Token {
        enum class Type {
            Number,
            Variable,
            Operator,
            UnaryOperator,
            Function,
            LeftParen,
            RightParen,
            Comma
        };
        Type type;
        std::string value;
    };

private:
    [[nodiscard]] std::expected<std::vector<Token>, EvaluationError> Tokenize(std::string_view formula) const;
    [[nodiscard]] std::expected<std::vector<Token>, EvaluationError> InfixToRPN(const std::vector<Token>& tokens) const;
    [[nodiscard]] std::expected<double, EvaluationError> EvaluateRPN(const std::vector<Token>& rpn, const Context& context) const;
};

} // namespace Client::Skills
