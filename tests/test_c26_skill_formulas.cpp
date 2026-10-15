#define DOCTEST_CONFIG_IMPLEMENT_WITH_MAIN
#include "doctest.h"
#include "Client/Skills/SkillFormulaEvaluator.h"

using namespace Client::Skills;

TEST_CASE("SkillFormulaEvaluator - Basic Arithmetic") {
    SkillFormulaEvaluator evaluator;
    SkillFormulaEvaluator::Context ctx;

    SUBCASE("Addition") {
        auto result = evaluator.Evaluate("2 + 2", ctx);
        REQUIRE(result.has_value());
        CHECK(result.value() == 4.0);
    }

    SUBCASE("Subtraction") {
        auto result = evaluator.Evaluate("10 - 3", ctx);
        REQUIRE(result.has_value());
        CHECK(result.value() == 7.0);
    }

    SUBCASE("Multiplication") {
        auto result = evaluator.Evaluate("4 * 5", ctx);
        REQUIRE(result.has_value());
        CHECK(result.value() == 20.0);
    }

    SUBCASE("Division") {
        auto result = evaluator.Evaluate("20 / 4", ctx);
        REQUIRE(result.has_value());
        CHECK(result.value() == 5.0);
    }

    SUBCASE("Precedence") {
        auto result = evaluator.Evaluate("2 + 3 * 4", ctx);
        REQUIRE(result.has_value());
        CHECK(result.value() == 14.0);
    }

    SUBCASE("Parentheses") {
        auto result = evaluator.Evaluate("(2 + 3) * 4", ctx);
        REQUIRE(result.has_value());
        CHECK(result.value() == 20.0);
    }
}

TEST_CASE("SkillFormulaEvaluator - Variables Context") {
    SkillFormulaEvaluator evaluator;
    SkillFormulaEvaluator::Context ctx = {
        {"atk", 100.0},
        {"str", 50.0},
        {"level", 10.0}
    };

    SUBCASE("Single Variable") {
        auto result = evaluator.Evaluate("atk", ctx);
        REQUIRE(result.has_value());
        CHECK(result.value() == 100.0);
    }

    SUBCASE("Expression with Variables") {
        auto result = evaluator.Evaluate("atk + str * level", ctx);
        REQUIRE(result.has_value());
        CHECK(result.value() == 600.0);
    }

    SUBCASE("Complex Expression") {
        auto result = evaluator.Evaluate("(atk + str) * (level / 2)", ctx);
        REQUIRE(result.has_value());
        CHECK(result.value() == 750.0);
    }
}

TEST_CASE("SkillFormulaEvaluator - Error Handling") {
    SkillFormulaEvaluator evaluator;
    SkillFormulaEvaluator::Context ctx;

    SUBCASE("Empty Formula") {
        auto result = evaluator.Evaluate("", ctx);
        REQUIRE(!result.has_value());
        CHECK(result.error() == EvaluationError::EmptyFormula);
    }

    SUBCASE("Division By Zero") {
        auto result = evaluator.Evaluate("10 / 0", ctx);
        REQUIRE(!result.has_value());
        CHECK(result.error() == EvaluationError::DivisionByZero);
    }

    SUBCASE("Unknown Variable") {
        auto result = evaluator.Evaluate("unknown_var + 5", ctx);
        REQUIRE(!result.has_value());
        CHECK(result.error() == EvaluationError::UnknownVariable);
    }

    SUBCASE("Mismatched Parentheses - Extra Left") {
        auto result = evaluator.Evaluate("(2 + 3", ctx);
        REQUIRE(!result.has_value());
        CHECK(result.error() == EvaluationError::MismatchedParentheses);
    }

    SUBCASE("Mismatched Parentheses - Extra Right") {
        auto result = evaluator.Evaluate("2 + 3)", ctx);
        REQUIRE(!result.has_value());
        CHECK(result.error() == EvaluationError::MismatchedParentheses);
    }
    
    SUBCASE("Invalid Syntax") {
        // "2 + + 3" now evaluates to 5 because the second + is treated as unary plus.
        // We need a test that actually results in InvalidSyntax.
        auto result = evaluator.Evaluate("2 * * 3", ctx);
        REQUIRE(!result.has_value());
        CHECK(result.error() == EvaluationError::InvalidSyntax);
    }
}

TEST_CASE("SkillFormulaEvaluator - Unary Operators & Functions") {
    SkillFormulaEvaluator evaluator;
    SkillFormulaEvaluator::Context ctx = {{"atk", 100.0}};

    SUBCASE("Unary Minus") {
        auto result = evaluator.Evaluate("-5", ctx);
        REQUIRE(result.has_value());
        CHECK(result.value() == -5.0);
    }

    SUBCASE("Unary Minus with Variable") {
        auto result = evaluator.Evaluate("-atk", ctx);
        REQUIRE(result.has_value());
        CHECK(result.value() == -100.0);
    }

    SUBCASE("Min Function") {
        auto result = evaluator.Evaluate("min(10, 20)", ctx);
        REQUIRE(result.has_value());
        CHECK(result.value() == 10.0);
    }

    SUBCASE("Max Function") {
        auto result = evaluator.Evaluate("max(atk, 50)", ctx);
        REQUIRE(result.has_value());
        CHECK(result.value() == 100.0);
    }

    SUBCASE("Sqrt Function") {
        auto result = evaluator.Evaluate("sqrt(25)", ctx);
        REQUIRE(result.has_value());
        CHECK(result.value() == 5.0);
    }

    SUBCASE("Complex Functions and Unary") {
        auto result = evaluator.Evaluate("max(-10, min(5, sqrt(16)))", ctx);
        REQUIRE(result.has_value());
        CHECK(result.value() == 4.0);
    }
}

TEST_CASE("SkillFormulaEvaluator - Advanced Checks & Nitpicks") {
    SkillFormulaEvaluator evaluator;
    SkillFormulaEvaluator::Context ctx = {{"_base_atk", 150.0}};

    SUBCASE("Variable starting with underscore") {
        auto result = evaluator.Evaluate("_base_atk * 2", ctx);
        REQUIRE(result.has_value());
        CHECK(result.value() == 300.0);
    }

    SUBCASE("Unary Precedence Test (100 / -2 / 5)") {
        auto result = evaluator.Evaluate("100 / -2 / 5", ctx);
        REQUIRE(result.has_value());
        CHECK(result.value() == -10.0);
    }

    SUBCASE("Malformed Number Trailing Chars") {
        auto result = evaluator.Evaluate("3.14.15", ctx);
        REQUIRE(!result.has_value());
        CHECK(result.error() == EvaluationError::InvalidSyntax);
    }
}
