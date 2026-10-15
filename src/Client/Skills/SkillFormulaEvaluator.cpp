#include "Client/Skills/SkillFormulaEvaluator.h"
#include <cctype>
#include <cmath>
#include <charconv>
#include <algorithm>

namespace Client::Skills {

namespace {
    bool IsOperator(char c) {
        return c == '+' || c == '-' || c == '*' || c == '/';
    }

    int Precedence(const SkillFormulaEvaluator::Token& token) {
        if (token.type == SkillFormulaEvaluator::Token::Type::UnaryOperator) return 3;
        if (token.type == SkillFormulaEvaluator::Token::Type::Operator) {
            if (token.value == "+" || token.value == "-") return 1;
            if (token.value == "*" || token.value == "/") return 2;
        }
        return 0;
    }
}

std::expected<double, EvaluationError> SkillFormulaEvaluator::Evaluate(std::string_view formula, const Context& context) const {
    if (formula.empty()) {
        return std::unexpected(EvaluationError::EmptyFormula);
    }

    auto tokensResult = Tokenize(formula);
    if (!tokensResult) return std::unexpected(tokensResult.error());

    auto rpnResult = InfixToRPN(tokensResult.value());
    if (!rpnResult) return std::unexpected(rpnResult.error());

    return EvaluateRPN(rpnResult.value(), context);
}

std::expected<std::vector<SkillFormulaEvaluator::Token>, EvaluationError> SkillFormulaEvaluator::Tokenize(std::string_view formula) const {
    std::vector<Token> tokens;
    size_t i = 0;

    while (i < formula.length()) {
        if (std::isspace(static_cast<unsigned char>(formula[i]))) {
            i++;
            continue;
        }

        if (std::isdigit(static_cast<unsigned char>(formula[i])) || formula[i] == '.') {
            std::string num;
            while (i < formula.length() && (std::isdigit(static_cast<unsigned char>(formula[i])) || formula[i] == '.')) {
                num += formula[i++];
            }
            tokens.push_back({Token::Type::Number, num});
            continue;
        }

        if (std::isalpha(static_cast<unsigned char>(formula[i])) || formula[i] == '_') {
            std::string name;
            while (i < formula.length() && (std::isalnum(static_cast<unsigned char>(formula[i])) || formula[i] == '_')) {
                name += formula[i++];
            }
            
            if (name == "min" || name == "max" || name == "sqrt") {
                 tokens.push_back({Token::Type::Function, name});
            } else {
                 tokens.push_back({Token::Type::Variable, name});
            }
            continue;
        }

        if (formula[i] == ',') {
            tokens.push_back({Token::Type::Comma, ","});
            i++;
            continue;
        }

        if (IsOperator(formula[i])) {
            bool isUnary = false;
            if (formula[i] == '-' || formula[i] == '+') {
                if (tokens.empty() || 
                    tokens.back().type == Token::Type::Operator || 
                    tokens.back().type == Token::Type::LeftParen ||
                    tokens.back().type == Token::Type::Comma) {
                    isUnary = true;
                }
            }

            if (isUnary) {
                 tokens.push_back({Token::Type::UnaryOperator, std::string(1, formula[i])});
            } else {
                 tokens.push_back({Token::Type::Operator, std::string(1, formula[i])});
            }
            i++;
            continue;
        }

        if (formula[i] == '(') {
            tokens.push_back({Token::Type::LeftParen, "("});
            i++;
            continue;
        }

        if (formula[i] == ')') {
            tokens.push_back({Token::Type::RightParen, ")"});
            i++;
            continue;
        }

        return std::unexpected(EvaluationError::InvalidSyntax);
    }

    return tokens;
}

std::expected<std::vector<SkillFormulaEvaluator::Token>, EvaluationError> SkillFormulaEvaluator::InfixToRPN(const std::vector<Token>& tokens) const {
    std::vector<Token> output;
    std::vector<Token> stack;

    for (const auto& token : tokens) {
        if (token.type == Token::Type::Number || token.type == Token::Type::Variable) {
            output.push_back(token);
        } else if (token.type == Token::Type::Function) {
            stack.push_back(token);
        } else if (token.type == Token::Type::Comma) {
            bool matched = false;
            while (!stack.empty()) {
                if (stack.back().type == Token::Type::LeftParen) {
                    matched = true;
                    break;
                }
                output.push_back(stack.back());
                stack.pop_back();
            }
            if (!matched) return std::unexpected(EvaluationError::InvalidSyntax); // Mismatched parens / comma out of place
        } else if (token.type == Token::Type::Operator) {
            while (!stack.empty() && (stack.back().type == Token::Type::Operator || stack.back().type == Token::Type::UnaryOperator) &&
                   Precedence(stack.back()) >= Precedence(token)) {
                output.push_back(stack.back());
                stack.pop_back();
            }
            stack.push_back(token);
        } else if (token.type == Token::Type::UnaryOperator) {
             // Unary operators have high precedence, right-associative usually, 
             // but here we just pop higher or equal. Unary minus/plus has higher precedence than * / + -
             while (!stack.empty() && stack.back().type == Token::Type::UnaryOperator) {
                 output.push_back(stack.back());
                 stack.pop_back();
             }
             stack.push_back(token);
        } else if (token.type == Token::Type::LeftParen) {
            stack.push_back(token);
        } else if (token.type == Token::Type::RightParen) {
            bool matched = false;
            while (!stack.empty()) {
                if (stack.back().type == Token::Type::LeftParen) {
                    matched = true;
                    stack.pop_back();
                    if (!stack.empty() && stack.back().type == Token::Type::Function) {
                        output.push_back(stack.back());
                        stack.pop_back();
                    }
                    break;
                }
                output.push_back(stack.back());
                stack.pop_back();
            }
            if (!matched) {
                return std::unexpected(EvaluationError::MismatchedParentheses);
            }
        }
    }

    while (!stack.empty()) {
        if (stack.back().type == Token::Type::LeftParen || stack.back().type == Token::Type::RightParen) {
            return std::unexpected(EvaluationError::MismatchedParentheses);
        }
        output.push_back(stack.back());
        stack.pop_back();
    }

    return output;
}

std::expected<double, EvaluationError> SkillFormulaEvaluator::EvaluateRPN(const std::vector<Token>& rpn, const Context& context) const {
    std::vector<double> stack;

    for (const auto& token : rpn) {
        if (token.type == Token::Type::Number) {
            double val = 0;
            auto [ptr, ec] = std::from_chars(token.value.data(), token.value.data() + token.value.size(), val);
            if (ec != std::errc() || ptr != token.value.data() + token.value.size()) {
                return std::unexpected(EvaluationError::InvalidSyntax);
            }
            stack.push_back(val);
        } else if (token.type == Token::Type::Variable) {
            auto it = context.find(token.value);
            if (it == context.end()) {
                return std::unexpected(EvaluationError::UnknownVariable);
            }
            stack.push_back(it->second);
        } else if (token.type == Token::Type::Operator) {
            if (stack.size() < 2) return std::unexpected(EvaluationError::InvalidSyntax);
            double b = stack.back(); stack.pop_back();
            double a = stack.back(); stack.pop_back();

            if (token.value == "+") stack.push_back(a + b);
            else if (token.value == "-") stack.push_back(a - b);
            else if (token.value == "*") stack.push_back(a * b);
            else if (token.value == "/") {
                if (b == 0) return std::unexpected(EvaluationError::DivisionByZero);
                stack.push_back(a / b);
            }
        } else if (token.type == Token::Type::UnaryOperator) {
            if (stack.empty()) return std::unexpected(EvaluationError::InvalidSyntax);
            double a = stack.back(); stack.pop_back();
            if (token.value == "-") stack.push_back(-a);
            else if (token.value == "+") stack.push_back(a);
        } else if (token.type == Token::Type::Function) {
            if (token.value == "min" || token.value == "max") {
                if (stack.size() < 2) return std::unexpected(EvaluationError::InvalidSyntax);
                double b = stack.back(); stack.pop_back();
                double a = stack.back(); stack.pop_back();
                if (token.value == "min") stack.push_back(std::min(a, b));
                else stack.push_back(std::max(a, b));
            } else if (token.value == "sqrt") {
                if (stack.empty()) return std::unexpected(EvaluationError::InvalidSyntax);
                double a = stack.back(); stack.pop_back();
                if (a < 0) return std::unexpected(EvaluationError::InvalidSyntax); // Real numbers only
                stack.push_back(std::sqrt(a));
            } else {
                return std::unexpected(EvaluationError::UnknownFunction);
            }
        }
    }

    if (stack.size() != 1) return std::unexpected(EvaluationError::InvalidSyntax);

    return stack.back();
}

} // namespace Client::Skills
